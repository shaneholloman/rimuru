use criterion::{Criterion, black_box, criterion_group, criterion_main};
use rimuru_core::adapters::ClaudeCodeAdapter;
use std::io::Write;

fn build_fixture(lines: usize) -> (tempfile::TempDir, std::path::PathBuf) {
    let dir = tempfile::tempdir().expect("tempdir");
    let projects = dir.path().join(".claude").join("projects").join("proj");
    std::fs::create_dir_all(&projects).unwrap();
    let session_id = "00000000-0000-0000-0000-000000000001";
    let path = projects.join(format!("{session_id}.jsonl"));
    let mut f = std::fs::File::create(&path).unwrap();
    for i in 0..lines {
        let entry = serde_json::json!({
            "timestamp": "2026-01-01T00:00:00Z",
            "sessionId": session_id,
            "message": {
                "role": if i % 2 == 0 { "user" } else { "assistant" },
                "model": "claude-sonnet-4-5",
                "usage": {"input_tokens": 100, "output_tokens": 50},
                "content": [{"type": "text", "text": format!("message body number {i} with some filler")}],
            }
        });
        writeln!(f, "{}", serde_json::to_string(&entry).unwrap()).unwrap();
    }
    (dir, path)
}

fn bench_claude_code_parse(c: &mut Criterion) {
    let (_dir, path) = build_fixture(1_000);

    let adapter = {
        let mut a = ClaudeCodeAdapter::new();
        // writeable because bench lives in same crate
        a.set_config_path_for_bench(_dir.path().join(".claude"));
        a
    };

    c.bench_function("parse_claude_code_1000_lines", |b| {
        b.iter(|| {
            let _ = adapter.parse_session_jsonl_full(black_box(&path)).unwrap();
        })
    });
}

criterion_group!(benches, bench_claude_code_parse);
criterion_main!(benches);
