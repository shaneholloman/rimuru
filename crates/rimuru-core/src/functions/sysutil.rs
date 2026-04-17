use serde_json::Value;
use tracing::warn;

pub fn kv_err(e: impl std::fmt::Display) -> iii_sdk::IIIError {
    iii_sdk::IIIError::Handler(e.to_string())
}

pub fn extract_input(input: Value) -> Value {
    if input.get("path_params").is_some() || input.get("query_params").is_some() {
        let mut merged = input
            .get("body")
            .cloned()
            .unwrap_or(Value::Object(serde_json::Map::new()));

        if let Some(params) = input.get("path_params").and_then(|v| v.as_object())
            && let Value::Object(ref mut map) = merged
        {
            for (k, v) in params {
                map.insert(k.clone(), v.clone());
            }
        }

        if let Some(params) = input.get("query_params").and_then(|v| v.as_object())
            && let Value::Object(ref mut map) = merged
        {
            for (k, v) in params {
                if !map.contains_key(k) {
                    map.insert(k.clone(), v.clone());
                }
            }
        }

        merged
    } else {
        input
    }
}

pub fn api_response(data: Value) -> Value {
    serde_json::json!({
        "status_code": 200,
        "body": data
    })
}

pub fn require_str(input: &Value, key: &str) -> Result<String, iii_sdk::IIIError> {
    input
        .get(key)
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
        .ok_or_else(|| iii_sdk::IIIError::Handler(format!("{} is required", key)))
}

pub fn require_uuid(input: &Value, key: &str) -> Result<uuid::Uuid, iii_sdk::IIIError> {
    let s = require_str(input, key)?;
    s.parse()
        .map_err(|_| iii_sdk::IIIError::Handler(format!("invalid UUID for {}: {}", key, s)))
}

pub async fn run_cmd(cmd: &str, args: &[&str]) -> String {
    match tokio::process::Command::new(cmd).args(args).output().await {
        Ok(o) => String::from_utf8_lossy(&o.stdout).to_string(),
        Err(e) => {
            warn!("Command `{}` failed: {}", cmd, e);
            String::new()
        }
    }
}

pub fn parse_vm_stat_value(line: &str) -> u64 {
    line.split(':')
        .nth(1)
        .unwrap_or("")
        .trim()
        .trim_end_matches('.')
        .parse()
        .unwrap_or(0)
}

pub fn parse_meminfo_kb(line: &str) -> u64 {
    line.split_whitespace()
        .nth(1)
        .unwrap_or("0")
        .parse()
        .unwrap_or(0)
}

pub async fn collect_memory_info() -> (f64, f64) {
    if cfg!(target_os = "macos") {
        collect_memory_macos().await
    } else if cfg!(target_os = "linux") {
        collect_memory_linux().await
    } else {
        (0.0, 0.0)
    }
}

async fn collect_memory_macos() -> (f64, f64) {
    let output = run_cmd("sysctl", &["-n", "hw.memsize"]).await;
    let total_bytes: u64 = output.trim().parse().unwrap_or(0);
    let total_mb = total_bytes as f64 / (1024.0 * 1024.0);

    let vm_output = run_cmd("vm_stat", &[]).await;
    let page_size: u64 = 16384;
    let mut active: u64 = 0;
    let mut wired: u64 = 0;
    let mut compressed: u64 = 0;

    for line in vm_output.lines() {
        if line.contains("Pages active:") {
            active = parse_vm_stat_value(line);
        } else if line.contains("Pages wired down:") {
            wired = parse_vm_stat_value(line);
        } else if line.contains("Pages occupied by compressor:") {
            compressed = parse_vm_stat_value(line);
        }
    }

    let used_mb = ((active + wired + compressed) * page_size) as f64 / (1024.0 * 1024.0);
    (used_mb, total_mb)
}

async fn collect_memory_linux() -> (f64, f64) {
    let content = match tokio::fs::read_to_string("/proc/meminfo").await {
        Ok(c) => c,
        Err(_) => return (0.0, 0.0),
    };

    let mut total_kb: u64 = 0;
    let mut available_kb: u64 = 0;

    for line in content.lines() {
        if line.starts_with("MemTotal:") {
            total_kb = parse_meminfo_kb(line);
        } else if line.starts_with("MemAvailable:") {
            available_kb = parse_meminfo_kb(line);
        }
    }

    let total_mb = total_kb as f64 / 1024.0;
    let used_mb = (total_kb - available_kb) as f64 / 1024.0;
    (used_mb, total_mb)
}

pub async fn collect_cpu_usage() -> f64 {
    if cfg!(target_os = "macos") {
        collect_cpu_macos().await
    } else if cfg!(target_os = "linux") {
        collect_cpu_linux().await
    } else {
        0.0
    }
}

async fn collect_cpu_macos() -> f64 {
    let output = run_cmd("ps", &["-A", "-o", "%cpu"]).await;
    let total: f64 = output
        .lines()
        .skip(1)
        .filter_map(|line| line.trim().parse::<f64>().ok())
        .sum();

    let num_cpus = std::thread::available_parallelism()
        .map(|n| n.get() as f64)
        .unwrap_or(1.0);

    (total / num_cpus).min(100.0)
}

async fn collect_cpu_linux() -> f64 {
    let output = run_cmd("grep", &["cpu ", "/proc/stat"]).await;
    let parts: Vec<u64> = output
        .split_whitespace()
        .skip(1)
        .filter_map(|s| s.parse().ok())
        .collect();

    if parts.len() >= 4 {
        let total: u64 = parts.iter().sum();
        let idle = parts[3];
        if total > 0 {
            ((total - idle) as f64 / total as f64) * 100.0
        } else {
            0.0
        }
    } else {
        0.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn extract_input_merges_body_with_path_params() {
        let input = json!({
            "body": {"other": "value"},
            "path_params": {"id": "42"},
        });
        let merged = extract_input(input);
        assert_eq!(merged["id"], "42");
        assert_eq!(merged["other"], "value");
    }

    #[test]
    fn extract_input_body_overrides_query_on_conflict() {
        let input = json!({
            "body": {"k": "from_body"},
            "query_params": {"k": "from_query"},
        });
        let merged = extract_input(input);
        assert_eq!(merged["k"], "from_body");
    }

    #[test]
    fn extract_input_passthrough_when_no_params() {
        let input = json!({"direct": true});
        assert_eq!(extract_input(input.clone()), input);
    }

    #[test]
    fn api_response_wraps_body_with_200() {
        let r = api_response(json!({"ok": true}));
        assert_eq!(r["status_code"], 200);
        assert_eq!(r["body"]["ok"], true);
    }

    #[test]
    fn require_str_returns_err_when_missing() {
        let err = require_str(&json!({}), "name").unwrap_err();
        assert!(err.to_string().contains("name"));
    }

    #[test]
    fn require_str_returns_value_when_present() {
        let got = require_str(&json!({"name": "alice"}), "name").unwrap();
        assert_eq!(got, "alice");
    }

    #[test]
    fn require_uuid_parses_valid_uuid() {
        let id = "00000000-0000-0000-0000-000000000001";
        let parsed = require_uuid(&json!({"id": id}), "id").unwrap();
        assert_eq!(parsed.to_string(), id);
    }

    #[test]
    fn require_uuid_rejects_bad_uuid() {
        let err = require_uuid(&json!({"id": "nope"}), "id").unwrap_err();
        assert!(err.to_string().contains("invalid UUID"));
    }

    #[test]
    fn parse_vm_stat_value_parses_trailing_dot() {
        assert_eq!(
            parse_vm_stat_value("Pages active:                1234."),
            1234
        );
        assert_eq!(parse_vm_stat_value("no colon"), 0);
    }

    #[test]
    fn parse_meminfo_kb_extracts_number() {
        assert_eq!(parse_meminfo_kb("MemTotal:       16384000 kB"), 16384000);
        assert_eq!(parse_meminfo_kb(""), 0);
    }
}
