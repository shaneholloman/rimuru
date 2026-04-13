//! Optimization recommendation engine (#33).
//!
//! Reads rimuru's own state (cost records, sessions, mcp proxy stats)
//! and produces a list of actionable recommendations. Each entry
//! carries an estimated savings in tokens and dollars, a category,
//! and a confidence score so the UI can rank them.
//!
//! Not every category the issue spec lists can be computed today:
//!
//! | Category         | Status      | Why |
//! |------------------|-------------|-----|
//! | mcp_schema       | implemented | derives from mcp_proxy_stats tool byte counts |
//! | model_mismatch   | implemented | derives from cost_records model + I/O ratio |
//! | output_verbose   | implemented | derives from mcp_proxy_stats output byte averages |
//! | repeated_calls   | stub        | needs per-turn tool-call records; not captured yet |
//! | file_reread      | stub        | needs file-read tracking per session; not captured yet |
//!
//! The stubs return zero recommendations rather than fabricating
//! anything. They'll start producing entries as soon as the data
//! sources exist.
//!
//! Apply / acknowledge state is persisted to the state KV under
//! `optimize_applied` so the UI can tag recommendations as actioned.

use chrono::{DateTime, Utc};
use iii_sdk::{III, RegisterFunctionMessage};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use uuid::Uuid;

use super::sysutil::{api_response, extract_input, kv_err};
use crate::state::StateKV;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Recommendation {
    pub id: Uuid,
    pub category: String,
    pub description: String,
    pub estimated_savings_tokens: u64,
    pub estimated_savings_dollars: f64,
    /// 0.0 - 1.0. Higher = more certain the savings estimate is real.
    pub confidence: f32,
    pub source: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppliedRecommendation {
    pub id: Uuid,
    pub category: String,
    pub description: String,
    pub applied_at: DateTime<Utc>,
    pub savings_tokens: u64,
    pub savings_dollars: f64,
}

// ---------- analyzers ----------

/// mcp_schema: tools whose schemas exceed a token budget relative
/// to their usage frequency. The proxy stats expose `tool_bytes` or
/// `schema_tokens` per tool — anything over ~5k tokens that's rarely
/// called is a candidate for progressive disclosure.
fn analyze_mcp_schemas(stats: &Value) -> Vec<Recommendation> {
    let Some(tools) = stats.get("tools").and_then(|v| v.as_array()) else {
        return Vec::new();
    };

    let mut out = Vec::new();
    for tool in tools {
        let name = tool
            .get("name")
            .and_then(|v| v.as_str())
            .unwrap_or("unknown");
        let schema_tokens = tool
            .get("schema_tokens")
            .and_then(|v| v.as_u64())
            .unwrap_or(0);
        let calls = tool.get("call_count").and_then(|v| v.as_u64()).unwrap_or(0);

        // Heuristic: schemas over 5k tokens called fewer than 10 times
        // per session average are worth gating behind explicit
        // progressive disclosure. Savings = schema_tokens * (1 - 0.2)
        // = 80% reduction assuming minimal descriptor stays.
        if schema_tokens >= 5_000 && calls < 10 {
            let savings_tokens = (schema_tokens as f64 * 0.8) as u64;
            let savings_dollars = savings_tokens as f64 / 1_000_000.0 * 3.0; // Sonnet rate
            out.push(Recommendation {
                id: Uuid::new_v4(),
                category: "mcp_schema".into(),
                description: format!(
                    "Enable progressive disclosure for `{}` MCP tool — \
                     schema is {} tokens but only called {} times. \
                     Estimated savings: {} tokens/session.",
                    name, schema_tokens, calls, savings_tokens
                ),
                estimated_savings_tokens: savings_tokens,
                estimated_savings_dollars: savings_dollars,
                confidence: 0.7,
                source: "mcp_proxy_stats".into(),
                created_at: Utc::now(),
            });
        }
    }
    out
}

/// output_verbose: tools whose output averages exceed 2k tokens and
/// who support compression. Bash, read_file, grep, and similar
/// high-output tools are the usual targets.
fn analyze_verbose_outputs(stats: &Value) -> Vec<Recommendation> {
    let Some(tools) = stats.get("tools").and_then(|v| v.as_array()) else {
        return Vec::new();
    };
    let mut out = Vec::new();
    for tool in tools {
        let name = tool
            .get("name")
            .and_then(|v| v.as_str())
            .unwrap_or("unknown");
        let avg_output = tool
            .get("avg_output_tokens")
            .and_then(|v| v.as_u64())
            .unwrap_or(0);
        let calls = tool.get("call_count").and_then(|v| v.as_u64()).unwrap_or(0);

        if avg_output >= 2_000 && calls > 0 {
            // 60% savings from compression is the realistic ceiling
            let total_output = avg_output * calls;
            let savings_tokens = (total_output as f64 * 0.6) as u64;
            let savings_dollars = savings_tokens as f64 / 1_000_000.0 * 3.0;
            out.push(Recommendation {
                id: Uuid::new_v4(),
                category: "output_verbose".into(),
                description: format!(
                    "`{}` output averages {} tokens across {} calls. \
                     Enable `rimuru slim` compression in the MCP proxy \
                     for an estimated {}% saving ({} tokens).",
                    name, avg_output, calls, 60, savings_tokens
                ),
                estimated_savings_tokens: savings_tokens,
                estimated_savings_dollars: savings_dollars,
                confidence: 0.6,
                source: "mcp_proxy_stats".into(),
                created_at: Utc::now(),
            });
        }
    }
    out
}

/// model_mismatch: sessions billed at Opus rates where the output
/// was small enough that Haiku would have produced the same result.
/// Heuristic: output_tokens < 1000 AND output/input ratio < 0.5
/// (i.e. the model was mostly consuming context and emitting a
/// short answer) on an Opus tier.
fn analyze_model_mismatch(cost_records: &[Value]) -> Vec<Recommendation> {
    let mut total_candidates = 0u64;
    let mut total_overspend_dollars = 0.0;
    let mut total_overspend_tokens = 0u64;

    for rec in cost_records {
        let model = rec.get("model").and_then(|v| v.as_str()).unwrap_or("");
        let input = rec
            .get("input_tokens")
            .and_then(|v| v.as_u64())
            .unwrap_or(0);
        let output = rec
            .get("output_tokens")
            .and_then(|v| v.as_u64())
            .unwrap_or(0);
        let cost = rec.get("cost").and_then(|v| v.as_f64()).unwrap_or(0.0);

        // Target: Opus with small, low-ratio output
        if !model.to_lowercase().contains("opus") {
            continue;
        }
        if output >= 1_000 {
            continue;
        }
        if input == 0 || (output as f64 / input as f64) >= 0.5 {
            continue;
        }

        // Haiku rate is roughly 20x cheaper than Opus; assume 95% savings.
        total_candidates += 1;
        total_overspend_dollars += cost * 0.95;
        total_overspend_tokens += output;
    }

    if total_candidates == 0 {
        return Vec::new();
    }

    vec![Recommendation {
        id: Uuid::new_v4(),
        category: "model_mismatch".into(),
        description: format!(
            "{} session{} billed to Opus produced short output with a \
             low output/input ratio. Routing simple queries to Haiku \
             would recover approximately ${:.2} at current prices.",
            total_candidates,
            if total_candidates == 1 { "" } else { "s" },
            total_overspend_dollars
        ),
        estimated_savings_tokens: total_overspend_tokens,
        estimated_savings_dollars: total_overspend_dollars,
        confidence: 0.5,
        source: "cost_records".into(),
        created_at: Utc::now(),
    }]
}

/// repeated_calls: placeholder. Needs per-turn tool-call records
/// with argument hashing so we can detect identical repeat calls.
/// rimuru doesn't capture that today, so we return empty rather
/// than fabricating values.
fn analyze_repeated_calls(_turns: &[Value]) -> Vec<Recommendation> {
    Vec::new()
}

/// file_reread: placeholder. Needs per-session file-read tracking
/// (path + mtime) captured from the tool call arguments. Not wired
/// up yet.
fn analyze_file_rereads(_session_context: &Value) -> Vec<Recommendation> {
    Vec::new()
}

// ---------- function registration ----------

pub fn register(iii: &III, kv: &StateKV) {
    register_recommendations(iii, kv);
    register_apply(iii, kv);
    register_applied_list(iii, kv);
}

fn register_recommendations(iii: &III, kv: &StateKV) {
    let kv = kv.clone();
    iii.register_function_with(
        RegisterFunctionMessage::with_id("rimuru.optimize.recommendations".to_string()),
        move |_input: Value| {
            let kv = kv.clone();
            async move {
                // Pull raw state needed by the analyzers.
                let cost_records: Vec<Value> = kv.list("cost_records").await.map_err(kv_err)?;
                let mcp_stats: Value = kv
                    .get("mcp_proxy", "stats")
                    .await
                    .map_err(kv_err)?
                    .unwrap_or(Value::Object(serde_json::Map::new()));

                let mut recs = Vec::new();
                recs.extend(analyze_mcp_schemas(&mcp_stats));
                recs.extend(analyze_verbose_outputs(&mcp_stats));
                recs.extend(analyze_model_mismatch(&cost_records));
                recs.extend(analyze_repeated_calls(&[]));
                recs.extend(analyze_file_rereads(&Value::Null));

                // Sort by dollar savings, biggest first.
                recs.sort_by(|a, b| {
                    b.estimated_savings_dollars
                        .partial_cmp(&a.estimated_savings_dollars)
                        .unwrap_or(std::cmp::Ordering::Equal)
                });

                let total_tokens: u64 = recs.iter().map(|r| r.estimated_savings_tokens).sum();
                let total_dollars: f64 = recs.iter().map(|r| r.estimated_savings_dollars).sum();

                Ok(api_response(json!({
                    "recommendations": recs,
                    "total_count": recs.len(),
                    "total_savings_tokens": total_tokens,
                    "total_savings_dollars": total_dollars,
                    "generated_at": Utc::now().to_rfc3339(),
                    "note": "repeated_calls and file_reread are currently stubs — they \
                             need per-turn tool-call and file-read tracking that rimuru \
                             does not capture yet.",
                })))
            }
        },
    );
}

fn register_apply(iii: &III, kv: &StateKV) {
    let kv = kv.clone();
    iii.register_function_with(
        RegisterFunctionMessage::with_id("rimuru.optimize.apply".to_string()),
        move |input: Value| {
            let kv = kv.clone();
            async move {
                let input = extract_input(input);
                let rec: Recommendation =
                    serde_json::from_value(input.get("recommendation").cloned().ok_or_else(
                        || iii_sdk::IIIError::Handler("missing recommendation".into()),
                    )?)
                    .map_err(|e| {
                        iii_sdk::IIIError::Handler(format!("invalid recommendation: {}", e))
                    })?;

                let applied = AppliedRecommendation {
                    id: rec.id,
                    category: rec.category.clone(),
                    description: rec.description.clone(),
                    applied_at: Utc::now(),
                    savings_tokens: rec.estimated_savings_tokens,
                    savings_dollars: rec.estimated_savings_dollars,
                };

                kv.set("optimize_applied", &applied.id.to_string(), &applied)
                    .await
                    .map_err(kv_err)?;

                Ok(api_response(json!({
                    "applied": applied,
                    "note": "recorded as acknowledged — rimuru does not yet take \
                             automated action on recommendations",
                })))
            }
        },
    );
}

fn register_applied_list(iii: &III, kv: &StateKV) {
    let kv = kv.clone();
    iii.register_function_with(
        RegisterFunctionMessage::with_id("rimuru.optimize.applied".to_string()),
        move |_input: Value| {
            let kv = kv.clone();
            async move {
                let applied: Vec<AppliedRecommendation> =
                    kv.list("optimize_applied").await.map_err(kv_err)?;

                let total_tokens: u64 = applied.iter().map(|a| a.savings_tokens).sum();
                let total_dollars: f64 = applied.iter().map(|a| a.savings_dollars).sum();

                Ok(api_response(json!({
                    "applied": applied,
                    "count": applied.len(),
                    "total_savings_tokens": total_tokens,
                    "total_savings_dollars": total_dollars,
                })))
            }
        },
    );
}
