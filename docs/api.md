# HTTP API Reference

All endpoints are served by `rimuru-worker` via `iii-http` on `:3111`. Payloads go through `extract_input()` which merges path params, query params, and body into one JSON object before dispatching to the function handler.

Response shape: every handler returns `api_response(data)` which wraps the result as `{"status_code": 200, "body": <data>}`. The CLI's `unwrap_body()` unpacks this for 2xx responses.

## Agents

| Method | Path | Function |
|--------|------|----------|
| GET | `/api/agents` | `rimuru.agents.list` |
| GET | `/api/agents/:id` | `rimuru.agents.get` |
| POST | `/api/agents` | `rimuru.agents.create` |
| PATCH | `/api/agents/:id` | `rimuru.agents.update` |
| DELETE | `/api/agents/:id` | `rimuru.agents.delete` |
| GET | `/api/agents/:id/status` | `rimuru.agents.status` |
| POST | `/api/agents/detect` | `rimuru.agents.detect` |
| POST | `/api/agents/connect` | `rimuru.agents.connect` |
| POST | `/api/agents/:id/disconnect` | `rimuru.agents.disconnect` |
| POST | `/api/agents/:id/sync` | `rimuru.agents.sync` |

## Sessions

| Method | Path | Function |
|--------|------|----------|
| GET | `/api/sessions` | `rimuru.sessions.list` |
| GET | `/api/sessions/:id` | `rimuru.sessions.get` |
| GET | `/api/sessions/active` | `rimuru.sessions.active` |
| GET | `/api/sessions/history` | `rimuru.sessions.history` |
| POST | `/api/sessions/cleanup` | `rimuru.sessions.cleanup` |

## Costs

| Method | Path | Function |
|--------|------|----------|
| POST | `/api/costs/record` | `rimuru.costs.record` |
| GET | `/api/costs/summary` | `rimuru.costs.summary` |
| GET | `/api/costs/daily` | `rimuru.costs.daily` |
| GET | `/api/costs/by-agent/:agent_id` | `rimuru.costs.by_agent` |
| POST | `/api/costs/daily-rollup` | `rimuru.costs.daily_rollup` |

## Budget

| Method | Path | Function |
|--------|------|----------|
| POST | `/api/budget/check` | `rimuru.budget.check` |
| GET | `/api/budget/status` | `rimuru.budget.status` |
| POST | `/api/budget/set` | `rimuru.budget.set` |
| GET | `/api/budget/alerts` | `rimuru.budget.alerts` |

## Runaway

| Method | Path | Function |
|--------|------|----------|
| POST | `/api/runaway/analyze` | `rimuru.runaway.analyze` |
| GET | `/api/runaway/scan` | `rimuru.runaway.scan` |
| POST | `/api/runaway/configure` | `rimuru.runaway.configure` |

## Guard

| Method | Path | Function |
|--------|------|----------|
| POST | `/api/guard/register` | `rimuru.guard.register` |
| POST | `/api/guard/complete` | `rimuru.guard.complete` |
| GET | `/api/guard/list` | `rimuru.guard.list` |
| GET | `/api/guard/history` | `rimuru.guard.history` |

## MCP Proxy

| Method | Path | Function |
|--------|------|----------|
| POST | `/api/mcp/proxy/connect` | `rimuru.mcp.proxy.connect` |
| GET | `/api/mcp/proxy/tools` | `rimuru.mcp.proxy.tools` |
| POST | `/api/mcp/proxy/call` | `rimuru.mcp.proxy.call` |
| GET | `/api/mcp/proxy/search` | `rimuru.mcp.proxy.search` |
| GET | `/api/mcp/proxy/stats` | `rimuru.mcp.proxy.stats` |
| POST | `/api/mcp/proxy/disconnect` | `rimuru.mcp.proxy.disconnect` |

## Context

| Method | Path | Function |
|--------|------|----------|
| GET | `/api/context/breakdown/:session_id` | `rimuru.context.breakdown` |
| GET | `/api/context/breakdowns` | `rimuru.context.breakdown_by_session` |
| GET | `/api/context/utilization` | `rimuru.context.utilization` |
| GET | `/api/context/waste` | `rimuru.context.waste` |

## Models

| Method | Path | Function |
|--------|------|----------|
| GET | `/api/models` | `rimuru.models.list` |
| GET | `/api/models/:id` | `rimuru.models.get` |
| POST | `/api/models/sync` | `rimuru.models.sync` |
| GET | `/api/models/advisor` | `rimuru.models.advisor` |
| GET | `/api/models/catalog` | `rimuru.models.catalog` |

## Metrics

| Method | Path | Function |
|--------|------|----------|
| GET | `/api/metrics` | `rimuru.metrics.current` |
| GET | `/api/metrics/history` | `rimuru.metrics.history` |

## Hooks

| Method | Path | Function |
|--------|------|----------|
| GET | `/api/hooks` | `rimuru.hooks.list` |
| POST | `/api/hooks/register` | `rimuru.hooks.register` |
| POST | `/api/hooks/dispatch` | `rimuru.hooks.dispatch` |

## Config

| Method | Path | Function |
|--------|------|----------|
| GET | `/api/config` | `rimuru.config.get` |
| GET | `/api/config/:key` | `rimuru.config.get` |
| PUT | `/api/config/:key` | `rimuru.config.set` |

## Health

| Method | Path | Function |
|--------|------|----------|
| GET | `/api/health` | `rimuru.health.check` |
