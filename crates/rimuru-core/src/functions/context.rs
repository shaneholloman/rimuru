use iii_sdk::{III, RegisterFunctionMessage};
use serde_json::{Value, json};

use super::sysutil::{kv_err, require_str};
use crate::adapters::ClaudeCodeAdapter;
use crate::models::{AgentType, ContextBreakdown, ContextUtilization, Session};
use crate::state::StateKV;

pub fn register(iii: &III, kv: &StateKV) {
    register_breakdown(iii, kv);
    register_breakdown_by_session(iii, kv);
    register_utilization(iii, kv);
    register_waste(iii, kv);
}

fn register_breakdown(iii: &III, kv: &StateKV) {
    let kv = kv.clone();
    iii.register_function_with(
        RegisterFunctionMessage::with_id("rimuru.context.breakdown".to_string()),
        move |input: Value| {
            let kv = kv.clone();
            async move {
                let session_id = require_str(&input, "session_id")?;

                let cached: Option<ContextBreakdown> = kv
                    .get("context_breakdowns", &session_id)
                    .await
                    .map_err(kv_err)?;

                if let Some(breakdown) = cached {
                    return Ok(serde_json::to_value(breakdown).unwrap_or_default());
                }

                let sessions: Vec<Session> = kv.list("sessions").await.map_err(kv_err)?;
                let session = sessions
                    .iter()
                    .find(|s| s.id.to_string() == session_id)
                    .ok_or_else(|| {
                        iii_sdk::IIIError::Handler(format!("Session not found: {}", session_id))
                    })?;

                if session.agent_type == AgentType::ClaudeCode {
                    let adapter = ClaudeCodeAdapter::new();
                    let session_files = adapter.find_session_file(&session_id);

                    if let Some(path) = session_files {
                        let (_session, breakdown) =
                            adapter.parse_session_jsonl_full(&path).map_err(|e| {
                                iii_sdk::IIIError::Handler(format!("Parse error: {}", e))
                            })?;

                        if let Err(e) = kv.set("context_breakdowns", &session_id, &breakdown).await
                        {
                            tracing::warn!("Failed to cache breakdown: {}", e);
                        }

                        return Ok(serde_json::to_value(breakdown).unwrap_or_default());
                    }
                }

                Ok(json!({"error": "No breakdown available for this session type"}))
            }
        },
    );
}

fn register_breakdown_by_session(iii: &III, kv: &StateKV) {
    let kv = kv.clone();
    iii.register_function_with(
        RegisterFunctionMessage::with_id("rimuru.context.breakdown_by_session".to_string()),
        move |_input: Value| {
            let kv = kv.clone();
            async move {
                let breakdowns: Vec<ContextBreakdown> =
                    kv.list("context_breakdowns").await.unwrap_or_default();

                Ok(json!({
                    "breakdowns": breakdowns,
                    "total": breakdowns.len()
                }))
            }
        },
    );
}

fn register_utilization(iii: &III, kv: &StateKV) {
    let kv = kv.clone();
    iii.register_function_with(
        RegisterFunctionMessage::with_id("rimuru.context.utilization".to_string()),
        move |_input: Value| {
            let kv = kv.clone();
            async move {
                let sessions: Vec<Session> = kv.list("sessions").await.map_err(kv_err)?;
                let active: Vec<&Session> = sessions
                    .iter()
                    .filter(|s| matches!(s.status, crate::models::SessionStatus::Active))
                    .collect();

                let threshold: f64 = 80.0;

                let utilizations: Vec<ContextUtilization> = active
                    .iter()
                    .map(|s| {
                        let model = s.model.clone().unwrap_or_else(|| "unknown".to_string());
                        let ctx_size = model_context_window(&model);
                        let used = s.total_tokens;
                        let pct = if ctx_size > 0 {
                            (used as f64 / ctx_size as f64) * 100.0
                        } else {
                            0.0
                        };

                        ContextUtilization {
                            session_id: s.id,
                            model: model.clone(),
                            context_window_size: ctx_size,
                            tokens_used: used,
                            utilization_percent: pct,
                            is_near_limit: pct >= threshold,
                        }
                    })
                    .collect();

                Ok(json!({
                    "utilizations": utilizations,
                    "total_active": active.len(),
                    "threshold_percent": threshold
                }))
            }
        },
    );
}

fn register_waste(iii: &III, kv: &StateKV) {
    let kv = kv.clone();
    iii.register_function_with(
        RegisterFunctionMessage::with_id("rimuru.context.waste".to_string()),
        move |_input: Value| {
            let kv = kv.clone();
            async move {
                let breakdowns: Vec<ContextBreakdown> =
                    kv.list("context_breakdowns").await.unwrap_or_default();

                let mut waste_reports: Vec<Value> = breakdowns
                    .iter()
                    .filter(|b| b.total_tokens > 0)
                    .map(|b| {
                        json!({
                            "session_id": b.session_id,
                            "total_tokens": b.total_tokens,
                            "tool_schema_tokens": b.tool_schema_tokens,
                            "bash_output_tokens": b.bash_output_tokens,
                            "mcp_tokens": b.mcp_tokens,
                            "waste_percent": b.waste_percent(),
                            "potential_savings": b.tool_schema_tokens + b.bash_output_tokens,
                        })
                    })
                    .collect();

                waste_reports.sort_by(|a, b| {
                    let a_waste = a
                        .get("waste_percent")
                        .and_then(|v| v.as_f64())
                        .unwrap_or(0.0);
                    let b_waste = b
                        .get("waste_percent")
                        .and_then(|v| v.as_f64())
                        .unwrap_or(0.0);
                    b_waste
                        .partial_cmp(&a_waste)
                        .unwrap_or(std::cmp::Ordering::Equal)
                });

                let total_waste: u64 = waste_reports
                    .iter()
                    .filter_map(|r| r.get("potential_savings").and_then(|v| v.as_u64()))
                    .sum();

                Ok(json!({
                    "sessions": waste_reports,
                    "total_waste_tokens": total_waste,
                    "total_sessions_analyzed": breakdowns.len()
                }))
            }
        },
    );
}

fn model_context_window(model: &str) -> u64 {
    match model {
        m if m.contains("opus") => 200_000,
        m if m.contains("sonnet") => 200_000,
        m if m.contains("haiku") => 200_000,
        m if m.contains("gpt-5") => 1_000_000,
        m if m.contains("gpt-4o") => 128_000,
        m if m.contains("gpt-4") => 128_000,
        m if m.contains("gemini") => 1_000_000,
        _ => 200_000,
    }
}
