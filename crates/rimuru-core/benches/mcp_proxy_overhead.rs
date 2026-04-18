use criterion::{Criterion, black_box, criterion_group, criterion_main};
use rimuru_core::mcp::McpTool;
use rimuru_core::mcp::proxy::McpProxy;
use serde_json::json;

fn bench_list_and_search(c: &mut Criterion) {
    let rt = tokio::runtime::Runtime::new().unwrap();
    let proxy = rt.block_on(async {
        let p = McpProxy::new();
        let tools: Vec<_> = (0..500)
            .map(|i| McpTool {
                name: format!("tool_{i}"),
                description: Some(format!("description for tool {i}, search keyword alpha")),
                input_schema: Some(json!({
                    "type": "object",
                    "properties": {"x": {"type": "string"}},
                })),
            })
            .collect();
        p.seed_tools_for_test("bench_server", tools).await;
        p
    });

    c.bench_function("list_tools_500_progressive", |b| {
        b.to_async(&rt).iter(|| async {
            let _ = proxy
                .list_tools(black_box(Some("bench_server")), true, 10)
                .await;
        })
    });

    c.bench_function("search_tools_500_keyword", |b| {
        b.to_async(&rt).iter(|| async {
            let _ = proxy.search_tools(black_box("alpha"), 10).await;
        })
    });
}

criterion_group!(benches, bench_list_and_search);
criterion_main!(benches);
