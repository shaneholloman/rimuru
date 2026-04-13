//! Cross-agent configuration synchronization (#26).
//!
//! Reads MCP servers, allowed/denied tools, custom instructions, and
//! model preferences from each installed agent's native config file,
//! merges them into a canonical `SyncConfig`, and can write the same
//! canonical state back into every agent's native format.
//!
//! Three iii functions are exposed:
//!
//! - `rimuru.sync.export` — read all installed agents, return canonical JSON
//! - `rimuru.sync.import` — apply canonical JSON to every agent (dry-run by default)
//! - `rimuru.sync.diff`   — show what would change between current state and a target
//!
//! Safety:
//!
//! - **Dry-run by default.** Import only writes when `apply=true`.
//! - **Backups before write.** Every target file is copied to
//!   `<file>.rimuru-backup-<timestamp>` before being overwritten.
//! - **Read errors are non-fatal.** A missing or malformed config for
//!   one agent does not abort the whole export — that agent surfaces
//!   as `read_error` and the others continue.

use std::collections::BTreeMap;
use std::path::PathBuf;

use chrono::Utc;
use iii_sdk::{III, RegisterFunctionMessage};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use tracing::warn;

use super::sysutil::{api_response, extract_input};
use crate::state::StateKV;

// ---------- canonical format ----------

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct McpServerConfig {
    pub command: String,
    #[serde(default)]
    pub args: Vec<String>,
    #[serde(default)]
    pub env: BTreeMap<String, String>,
    #[serde(default)]
    pub disabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SyncConfig {
    /// MCP server entries keyed by server name. BTreeMap so output
    /// order is deterministic and diffs are reproducible.
    #[serde(default)]
    pub mcp_servers: BTreeMap<String, McpServerConfig>,
    #[serde(default)]
    pub allowed_tools: Vec<String>,
    #[serde(default)]
    pub denied_tools: Vec<String>,
    #[serde(default)]
    pub custom_instructions: Option<String>,
    #[serde(default)]
    pub model_preferences: BTreeMap<String, String>,
}

impl SyncConfig {
    /// Merge `other` on top of `self`, taking `other`'s values where
    /// they conflict. Used by export to roll up every agent's state.
    pub fn merge(&mut self, other: SyncConfig) {
        for (k, v) in other.mcp_servers {
            self.mcp_servers.insert(k, v);
        }
        for tool in other.allowed_tools {
            if !self.allowed_tools.contains(&tool) {
                self.allowed_tools.push(tool);
            }
        }
        for tool in other.denied_tools {
            if !self.denied_tools.contains(&tool) {
                self.denied_tools.push(tool);
            }
        }
        if other.custom_instructions.is_some() {
            self.custom_instructions = other.custom_instructions;
        }
        for (k, v) in other.model_preferences {
            self.model_preferences.insert(k, v);
        }
    }
}

// ---------- per-agent adapters ----------

/// Each entry knows how to read its native format into a SyncConfig
/// and how to write a SyncConfig back. Supported agents today:
/// Claude Code, Cursor, Codex, Gemini CLI. Adding more is a matter
/// of extending this table.
struct SyncAgent {
    name: &'static str,
    config_file: PathBuf,
    read: fn(&Value) -> SyncConfig,
    write: fn(SyncConfig, &Value) -> Value,
}

fn home_join(p: &str) -> PathBuf {
    dirs::home_dir()
        .unwrap_or_else(|| PathBuf::from("/tmp"))
        .join(p)
}

fn agent_table() -> Vec<SyncAgent> {
    vec![
        SyncAgent {
            name: "claude_code",
            config_file: home_join(".claude/settings.json"),
            read: read_claude_code,
            write: write_claude_code,
        },
        SyncAgent {
            name: "cursor",
            config_file: home_join(".cursor/mcp.json"),
            read: read_cursor,
            write: write_cursor,
        },
        SyncAgent {
            name: "codex",
            config_file: home_join(".config/codex/config.yaml"),
            read: read_codex,
            write: write_codex,
        },
        SyncAgent {
            name: "gemini_cli",
            config_file: home_join(".gemini/settings.json"),
            read: read_gemini,
            write: write_gemini,
        },
    ]
}

// ---------- shared parsers ----------

/// Lift a `mcpServers` object (Claude / Gemini / Cursor shape) into
/// the canonical map. Tolerates missing fields and unknown extras.
fn parse_mcp_servers(value: &Value) -> BTreeMap<String, McpServerConfig> {
    let mut out = BTreeMap::new();
    let Some(map) = value.as_object() else {
        return out;
    };
    for (name, raw) in map {
        let cmd = raw
            .get("command")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();
        let args = raw
            .get("args")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                    .collect()
            })
            .unwrap_or_default();
        let env = raw
            .get("env")
            .and_then(|v| v.as_object())
            .map(|m| {
                m.iter()
                    .filter_map(|(k, v)| v.as_str().map(|s| (k.clone(), s.to_string())))
                    .collect::<BTreeMap<String, String>>()
            })
            .unwrap_or_default();
        let disabled = raw
            .get("disabled")
            .and_then(|v| v.as_bool())
            .unwrap_or(false);
        out.insert(
            name.clone(),
            McpServerConfig {
                command: cmd,
                args,
                env,
                disabled,
            },
        );
    }
    out
}

fn render_mcp_servers(servers: &BTreeMap<String, McpServerConfig>) -> Value {
    let mut out = serde_json::Map::new();
    for (name, cfg) in servers {
        let mut entry = serde_json::Map::new();
        entry.insert("command".into(), Value::String(cfg.command.clone()));
        entry.insert(
            "args".into(),
            Value::Array(cfg.args.iter().map(|s| Value::String(s.clone())).collect()),
        );
        if !cfg.env.is_empty() {
            let env: serde_json::Map<String, Value> = cfg
                .env
                .iter()
                .map(|(k, v)| (k.clone(), Value::String(v.clone())))
                .collect();
            entry.insert("env".into(), Value::Object(env));
        }
        if cfg.disabled {
            entry.insert("disabled".into(), Value::Bool(true));
        }
        out.insert(name.clone(), Value::Object(entry));
    }
    Value::Object(out)
}

// ---------- shared helpers ----------

/// Collect a JSON array of strings, dropping non-string entries.
fn json_str_array(v: &Value) -> Vec<String> {
    v.as_array()
        .map(|arr| {
            arr.iter()
                .filter_map(|x| x.as_str().map(String::from))
                .collect()
        })
        .unwrap_or_default()
}

/// Build a JSON array from a slice of strings.
fn to_json_str_array(items: &[String]) -> Value {
    Value::Array(items.iter().map(|s| Value::String(s.clone())).collect())
}

/// Shared read path: grab mcpServers under `servers_key` and the
/// default model under `model`. Used by read_cursor (model=None),
/// read_codex (servers_key="mcp_servers"), and read_gemini.
fn read_servers_and_model(content: &Value, servers_key: &str, with_model: bool) -> SyncConfig {
    let mut cfg = SyncConfig::default();
    if let Some(servers) = content.get(servers_key) {
        cfg.mcp_servers = parse_mcp_servers(servers);
    }
    if with_model && let Some(model) = content.get("model").and_then(|v| v.as_str()) {
        cfg.model_preferences
            .insert("default".into(), model.to_string());
    }
    cfg
}

/// Shared write path: start from the existing object so unknown keys
/// survive, overwrite mcpServers under `servers_key`, and optionally
/// write the default model.
fn write_servers_and_model(
    cfg: SyncConfig,
    existing: &Value,
    servers_key: &str,
    with_model: bool,
) -> Value {
    let mut out = existing.as_object().cloned().unwrap_or_default();
    out.insert(servers_key.into(), render_mcp_servers(&cfg.mcp_servers));
    if with_model && let Some(default_model) = cfg.model_preferences.get("default") {
        out.insert("model".into(), Value::String(default_model.clone()));
    }
    Value::Object(out)
}

// ---------- Claude Code ----------

fn read_claude_code(content: &Value) -> SyncConfig {
    let mut cfg = SyncConfig::default();
    if let Some(servers) = content.get("mcpServers") {
        cfg.mcp_servers = parse_mcp_servers(servers);
    }
    if let Some(allowed) = content.get("permissions").and_then(|p| p.get("allow")) {
        cfg.allowed_tools = json_str_array(allowed);
    }
    if let Some(denied) = content.get("permissions").and_then(|p| p.get("deny")) {
        cfg.denied_tools = json_str_array(denied);
    }
    if let Some(instructions) = content.get("customInstructions").and_then(|v| v.as_str()) {
        cfg.custom_instructions = Some(instructions.to_string());
    }
    cfg
}

fn write_claude_code(cfg: SyncConfig, existing: &Value) -> Value {
    // Preserve any unknown keys the user has set by starting from the
    // existing object and overwriting only the fields we manage.
    let mut out = existing.as_object().cloned().unwrap_or_default();
    out.insert("mcpServers".into(), render_mcp_servers(&cfg.mcp_servers));

    let mut perms = out
        .get("permissions")
        .and_then(|v| v.as_object())
        .cloned()
        .unwrap_or_default();
    perms.insert("allow".into(), to_json_str_array(&cfg.allowed_tools));
    perms.insert("deny".into(), to_json_str_array(&cfg.denied_tools));
    out.insert("permissions".into(), Value::Object(perms));

    if let Some(instructions) = cfg.custom_instructions {
        out.insert("customInstructions".into(), Value::String(instructions));
    }
    Value::Object(out)
}

// ---------- Cursor / Codex / Gemini CLI ----------

fn read_cursor(content: &Value) -> SyncConfig {
    read_servers_and_model(content, "mcpServers", false)
}

fn write_cursor(cfg: SyncConfig, existing: &Value) -> Value {
    write_servers_and_model(cfg, existing, "mcpServers", false)
}

fn read_codex(content: &Value) -> SyncConfig {
    // Codex stores config as YAML. Field is mcp_servers (snake_case).
    read_servers_and_model(content, "mcp_servers", true)
}

fn write_codex(cfg: SyncConfig, existing: &Value) -> Value {
    write_servers_and_model(cfg, existing, "mcp_servers", true)
}

fn read_gemini(content: &Value) -> SyncConfig {
    read_servers_and_model(content, "mcpServers", true)
}

fn write_gemini(cfg: SyncConfig, existing: &Value) -> Value {
    write_servers_and_model(cfg, existing, "mcpServers", true)
}

// ---------- file IO ----------

/// Read a config file. Supports JSON and YAML by file extension.
/// Returns Ok(None) when the file doesn't exist (so import/export
/// can skip uninstalled agents cleanly).
fn load_config_file(path: &PathBuf) -> Result<Option<Value>, String> {
    if !path.exists() {
        return Ok(None);
    }
    let raw = std::fs::read_to_string(path).map_err(|e| format!("read failed: {}", e))?;
    let ext = path.extension().and_then(|e| e.to_str());
    let val =
        match ext {
            Some("yaml") | Some("yml") => serde_yaml::from_str::<Value>(&raw)
                .map_err(|e| format!("yaml parse failed: {}", e))?,
            _ => serde_json::from_str::<Value>(&raw)
                .map_err(|e| format!("json parse failed: {}", e))?,
        };
    Ok(Some(val))
}

/// Write a config value back, preserving extension format. Creates a
/// timestamped backup of the existing file beforehand.
fn write_config_file(path: &PathBuf, value: &Value) -> Result<Option<PathBuf>, String> {
    let backup = if path.exists() {
        let stamp = Utc::now().format("%Y%m%dT%H%M%S").to_string();
        let backup_path = path.with_extension(format!(
            "{}.rimuru-backup-{}",
            path.extension().and_then(|s| s.to_str()).unwrap_or("bak"),
            stamp
        ));
        std::fs::copy(path, &backup_path).map_err(|e| format!("backup failed: {}", e))?;
        Some(backup_path)
    } else {
        None
    };

    if let Some(parent) = path.parent()
        && !parent.exists()
    {
        std::fs::create_dir_all(parent).map_err(|e| format!("mkdir failed: {}", e))?;
    }

    let ext = path.extension().and_then(|e| e.to_str());
    let serialized = match ext {
        Some("yaml") | Some("yml") => {
            serde_yaml::to_string(value).map_err(|e| format!("yaml serialize failed: {}", e))?
        }
        _ => serde_json::to_string_pretty(value)
            .map_err(|e| format!("json serialize failed: {}", e))?,
    };
    std::fs::write(path, serialized).map_err(|e| format!("write failed: {}", e))?;
    Ok(backup)
}

// ---------- diff ----------

/// Lightweight diff between two SyncConfigs reported as a JSON
/// structure that's readable on the CLI and renderable in the UI
/// without further processing.
fn diff_configs(current: &SyncConfig, target: &SyncConfig) -> Value {
    let mut servers_added = Vec::new();
    let mut servers_removed = Vec::new();
    let mut servers_changed = Vec::new();

    for (name, target_cfg) in &target.mcp_servers {
        match current.mcp_servers.get(name) {
            None => servers_added.push(name.clone()),
            Some(cur_cfg) => {
                if cur_cfg.command != target_cfg.command
                    || cur_cfg.args != target_cfg.args
                    || cur_cfg.env != target_cfg.env
                    || cur_cfg.disabled != target_cfg.disabled
                {
                    servers_changed.push(name.clone());
                }
            }
        }
    }
    for name in current.mcp_servers.keys() {
        if !target.mcp_servers.contains_key(name) {
            servers_removed.push(name.clone());
        }
    }

    let allow_added: Vec<&String> = target
        .allowed_tools
        .iter()
        .filter(|t| !current.allowed_tools.contains(t))
        .collect();
    let allow_removed: Vec<&String> = current
        .allowed_tools
        .iter()
        .filter(|t| !target.allowed_tools.contains(t))
        .collect();
    let deny_added: Vec<&String> = target
        .denied_tools
        .iter()
        .filter(|t| !current.denied_tools.contains(t))
        .collect();
    let deny_removed: Vec<&String> = current
        .denied_tools
        .iter()
        .filter(|t| !target.denied_tools.contains(t))
        .collect();

    let instructions_changed = current.custom_instructions != target.custom_instructions;

    json!({
        "mcp_servers": {
            "added": servers_added,
            "removed": servers_removed,
            "changed": servers_changed,
        },
        "allowed_tools": {
            "added": allow_added,
            "removed": allow_removed,
        },
        "denied_tools": {
            "added": deny_added,
            "removed": deny_removed,
        },
        "custom_instructions_changed": instructions_changed,
    })
}

// ---------- iii functions ----------

pub fn register(iii: &III, kv: &StateKV) {
    register_export(iii, kv);
    register_diff(iii, kv);
    register_import(iii, kv);
}

fn register_export(iii: &III, _kv: &StateKV) {
    iii.register_function_with(
        RegisterFunctionMessage::with_id("rimuru.sync.export".to_string()),
        move |_input: Value| async move {
            let mut canonical = SyncConfig::default();
            let mut per_agent = serde_json::Map::new();
            let mut errors = serde_json::Map::new();

            for agent in agent_table() {
                match load_config_file(&agent.config_file) {
                    Ok(Some(content)) => {
                        let cfg = (agent.read)(&content);
                        per_agent.insert(
                            agent.name.into(),
                            serde_json::to_value(&cfg).unwrap_or(Value::Null),
                        );
                        canonical.merge(cfg);
                    }
                    Ok(None) => {
                        // Agent not installed; skip silently.
                    }
                    Err(e) => {
                        warn!(
                            "Failed to read {} config at {}: {}",
                            agent.name,
                            agent.config_file.display(),
                            e
                        );
                        errors.insert(agent.name.into(), Value::String(e));
                    }
                }
            }

            Ok(api_response(json!({
                "canonical": canonical,
                "per_agent": Value::Object(per_agent),
                "errors": Value::Object(errors),
                "exported_at": Utc::now().to_rfc3339(),
            })))
        },
    );
}

fn register_diff(iii: &III, _kv: &StateKV) {
    iii.register_function_with(
        RegisterFunctionMessage::with_id("rimuru.sync.diff".to_string()),
        move |input: Value| async move {
            let input = extract_input(input);
            // If the caller provides a `target` SyncConfig we diff
            // against that. Otherwise we diff each agent against the
            // merged canonical built from all agents (i.e. show the
            // drift between agents).
            let target_param = input.get("target").cloned();
            let agent_filter = input
                .get("agent")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string());

            // First pass: collect every agent's current cfg + the canonical.
            let mut agents = Vec::new();
            let mut canonical = SyncConfig::default();
            for agent in agent_table() {
                match load_config_file(&agent.config_file) {
                    Ok(Some(content)) => {
                        let cfg = (agent.read)(&content);
                        canonical.merge(cfg.clone());
                        agents.push((agent.name, agent.config_file.clone(), Some(cfg)));
                    }
                    Ok(None) => {
                        agents.push((agent.name, agent.config_file.clone(), None));
                    }
                    Err(e) => {
                        warn!("diff: read failed for {}: {}", agent.name, e);
                        agents.push((agent.name, agent.config_file.clone(), None));
                    }
                }
            }

            let target: SyncConfig = match target_param {
                Some(v) => serde_json::from_value(v).unwrap_or(canonical.clone()),
                None => canonical.clone(),
            };

            let mut diffs = serde_json::Map::new();
            for (name, _path, cfg_opt) in agents {
                if let Some(filter) = &agent_filter
                    && filter != name
                {
                    continue;
                }
                let current = cfg_opt.unwrap_or_default();
                diffs.insert(name.into(), diff_configs(&current, &target));
            }

            Ok(api_response(json!({
                "diffs": Value::Object(diffs),
                "target_source": if input.get("target").is_some() { "explicit" } else { "merged_canonical" },
            })))
        },
    );
}

fn register_import(iii: &III, _kv: &StateKV) {
    iii.register_function_with(
        RegisterFunctionMessage::with_id("rimuru.sync.import".to_string()),
        move |input: Value| async move {
            let input = extract_input(input);
            let apply = input
                .get("apply")
                .and_then(|v| v.as_bool())
                .unwrap_or(false);
            let canonical_value = input.get("canonical").cloned().ok_or_else(|| {
                iii_sdk::IIIError::Handler("missing required field: canonical".into())
            })?;
            let canonical: SyncConfig = serde_json::from_value(canonical_value)
                .map_err(|e| iii_sdk::IIIError::Handler(format!("invalid canonical: {}", e)))?;

            let mut results = serde_json::Map::new();

            for agent in agent_table() {
                let existing = load_config_file(&agent.config_file)
                    .unwrap_or(None)
                    .unwrap_or(Value::Object(serde_json::Map::new()));
                let current_cfg = (agent.read)(&existing);
                let diff = diff_configs(&current_cfg, &canonical);
                let new_value = (agent.write)(canonical.clone(), &existing);

                let mut entry = json!({
                    "config_file": agent.config_file.to_string_lossy(),
                    "diff": diff,
                });

                if apply && agent.config_file.parent().is_some_and(|p| p.exists()) {
                    match write_config_file(&agent.config_file, &new_value) {
                        Ok(backup) => {
                            entry["applied"] = Value::Bool(true);
                            entry["backup_file"] = backup
                                .map(|p| Value::String(p.to_string_lossy().into_owned()))
                                .unwrap_or(Value::Null);
                        }
                        Err(e) => {
                            entry["applied"] = Value::Bool(false);
                            entry["error"] = Value::String(e);
                        }
                    }
                } else {
                    entry["applied"] = Value::Bool(false);
                    if !apply {
                        entry["reason"] = Value::String("dry_run".into());
                    } else {
                        entry["reason"] = Value::String("agent_not_installed".into());
                    }
                }

                results.insert(agent.name.into(), entry);
            }

            Ok(api_response(json!({
                "results": Value::Object(results),
                "applied": apply,
                "imported_at": Utc::now().to_rfc3339(),
            })))
        },
    );
}
