use criterion::{Criterion, black_box, criterion_group, criterion_main};
use rimuru_core::mcp::McpClient;
use serde_json::json;

fn bench_estimate_tokens(c: &mut Criterion) {
    let small = json!({"x": 1, "y": "hello"});
    let medium = json!({
        "tools": (0..50)
            .map(|i| json!({
                "name": format!("tool_{i}"),
                "description": "a tool that does a thing",
                "input_schema": {"type": "object", "properties": {"x": {"type": "string"}}},
            }))
            .collect::<Vec<_>>()
    });
    let large_text = "x".repeat(100_000);
    let large = json!({"output": large_text});

    let mut group = c.benchmark_group("estimate_tokens");
    group.bench_function("small", |b| {
        b.iter(|| McpClient::estimate_tokens(black_box(&small)))
    });
    group.bench_function("medium_50_tools", |b| {
        b.iter(|| McpClient::estimate_tokens(black_box(&medium)))
    });
    group.bench_function("large_100kb", |b| {
        b.iter(|| McpClient::estimate_tokens(black_box(&large)))
    });
    group.finish();
}

fn bench_compression(c: &mut Criterion) {
    use rimuru_core::mcp::compress::{CompressionStrategy, compress};

    let big_text = (0..1_000)
        .map(|i| format!("line {i} with some content"))
        .collect::<Vec<_>>()
        .join("\n");
    let value = serde_json::Value::String(big_text);

    c.bench_function("compress_summarize_1000_lines", |b| {
        b.iter(|| compress(black_box(&value), CompressionStrategy::Summarize, 100))
    });

    c.bench_function("compress_auto_1000_lines", |b| {
        b.iter(|| compress(black_box(&value), CompressionStrategy::Auto, 100))
    });
}

criterion_group!(benches, bench_estimate_tokens, bench_compression);
criterion_main!(benches);
