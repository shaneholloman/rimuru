# Rimuru v0.4.0 — Benimaru: Guardrails

Cost governance for AI agents. Hard budget limits, runaway loop detection, process wrappers that kill overspending agents, and MCP output compression that cuts context bloat by the strategy the response actually needs.

## Budget Engine

Real-time budget enforcement that stops the "$200 vanished in 3 days" problem. Monthly, daily, per-session, and per-agent-daily caps — all checked before every cost record lands in state.

```bash
rimuru config set budget_monthly 50
rimuru config set budget_daily 5
rimuru config set budget_session 2
rimuru config set budget_action block
```

**Key features:**
- **Pre-write enforcement** — budget check runs before state mutation. If `budget_action=block` and any cap is exceeded, cost recording is rejected with an error rather than persisted and then flagged
- **Four cap levels** — `budget_monthly`, `budget_daily`, `budget_session`, `budget_daily_agent`, with shared `budget_alert_threshold` (default 0.8)
- **Alert thresholds** — fires `budget.warning` hook at threshold, `budget.exceeded` hook at limit
- **Burn rate projection** — `rimuru budget status` projects end-of-month spend from current daily rate against the actual days in the current month
- **Aggregate-backed** — monthly/daily/agent totals read from `cost_daily` and `cost_agent` rollups, not full `cost_records` scans
- **Fail-open on unavailability** — if the budget service itself is unreachable, cost recording logs a warning and proceeds; only a successful "exceeded + block" response halts the write
- **Alert ledger** — `rimuru.budget.alerts` returns the persisted alert history with true total count and requested page size

## Runaway Loop Detection

Detect when an agent is stuck burning tokens without progress. Analyzes the last N turns of a session for four patterns and produces a severity score plus wasted-token estimate.

```bash
rimuru.runaway.analyze '{"session_id": "..."}'
rimuru.runaway.scan              # scan all active sessions
rimuru.runaway.configure '{"window": 15, "repeat_threshold": 4}'
```

**Detection patterns:**

| Pattern            | Trigger                                         | Severity formula      |
|--------------------|-------------------------------------------------|-----------------------|
| `repeated_calls`   | Same tool name repeated consecutively > threshold | `min(1.0, count/10)` |
| `repeated_errors`  | Same `content_type+role` error signature > threshold | `min(1.0, count/8)` |
| `token_explosion`  | Last 3 turns > N× baseline input tokens (baseline excludes last 3) | `min(1.0, (ratio-1)/4)` |
| `oscillation`      | Two tools alternating > 4 times                | `min(1.0, count/8)`   |

**Guarantees:**
- **Baseline integrity** — the token-explosion baseline excludes the last 3 turns so spike samples don't inflate the average they're compared against
- **Distinct-message errors** — consecutive error turns only count when the error signature matches; noisy retries with different errors don't register as a loop
- **Configured thresholds** — `analyze` and `scan` read `runaway_window`, `runaway_repeat_threshold`, and `runaway_token_explosion_ratio` from config so `configure` changes actually affect detection
- **Error propagation** — KV failures in `scan` surface as errors rather than silently presenting as "no loops detected"
- **Validated config set** — zero window, zero repeat threshold, or explosion ratio ≤ 1.0 are rejected at write time

## `rimuru guard` — Process Wrapper

Wrap any agent process with a cost limit. The wrapper polls cost totals every 5 seconds against the guard's start timestamp and either warns or kills the child when the limit is crossed.

```bash
rimuru guard start --limit 5.00 --action kill -- claude
rimuru guard status
rimuru guard history
```

**Key features:**
- **Typed `--action`** — `kill` or `warn`, validated by `clap::ValueEnum` at parse time
- **Validated `--limit`** — rejects NaN, infinity, and non-positive values via a custom value parser
- **Spawn-then-register** — child process is spawned first; if spawn fails there is no phantom guard in KV. Registration happens with the real PID
- **Scoped cost queries** — `rimuru.costs.summary` is called with `since=started_at` so the guard only accounts for spend incurred after the wrapper started
- **PID tracking** — `GuardRecord.pid` is populated on register and shown in `rimuru guard status`
- **Stderr diagnostics** — wrapper banner, warnings, and summary go to `stderr` so they don't corrupt the wrapped process's `stdout` (critical for piped agent output)
- **Atomic completion** — on exit the history record is written before the active guard is deleted
- **Logged completion failures** — if `rimuru.guard.complete` fails, the wrapper prints a warning so operators know the guard ledger may be incomplete

## MCP Output Compression

Large tool responses get compressed before they reach the agent. Auto-mode inspects the payload and picks the strategy that actually fits the content shape.

```bash
rimuru mcp stats    # now shows tokens_saved_by_compression + compression_count
```

**Strategies:**

| Strategy     | Picked when                                          | What it does                                    |
|--------------|------------------------------------------------------|--------------------------------------------------|
| `Auto`       | default — inspects content                          | Routes to the best strategy below                |
| `JsonPaths`  | Object/array with > 50 keys or > 5K estimated tokens | Keeps shallow structure; deep nodes → `{__depth, __keys}`; long arrays → head/tail + `__truncated` |
| `ErrorsOnly` | String with error/warning/fail/panic/traceback lines | Keeps only error lines with 3 lines of context before each match |
| `TreeView`   | String that looks like a file listing               | Renders paths as an indented tree                |
| `Summarize`  | Plain text                                           | Keeps first 10 + last 5 lines, removes middle    |
| `Truncate`   | Fallback                                             | Char-safe truncation at `~max_tokens * 3` chars  |

**Key features:**
- **Threshold-gated** — responses ≤ 2000 tokens skip compression entirely (zero overhead for small payloads)
- **UTF-8 safe** — all string truncation uses `char_indices().nth()` so multi-byte characters can't split mid-codepoint
- **Overflow safe** — `max_tokens * 3` uses `saturating_mul` so extreme `max_tokens` values can't wrap on 32-bit targets
- **Smart fallback** — if a strategy doesn't reduce below the cap, the fallback is `Truncate` (not the other way around)
- **Never loses errors** — error-line detection preserves the full matched line plus 3 lines of prior context
- **Metrics tracked** — `tokens_saved_by_compression` and `compression_count` surface in `rimuru.mcp.proxy.stats`
- **Cached post-compression** — the LRU cache stores the compressed payload, so cache hits don't re-expand it

## Feature Matrix (v0.1 → v0.4)

| Feature                  | CLI | Web UI | TUI |
|--------------------------|-----|--------|-----|
| Agents / Sessions / Costs | ✅ | ✅ | ✅ |
| Context observability    | ✅ | ✅ | ✅ |
| MCP proxy                | ✅ | ✅ | ✅ |
| Models / advisor         | ✅ | ✅ | ✅ |
| Metrics / health         | ✅ | ✅ | ✅ |
| Budget engine            | ✅ | ⏳ | ⏳ |
| Runaway detection        | ✅ | ⏳ | ⏳ |
| Guard wrapper            | ✅ | — | — |
| Output compression       | ✅ | ⏳ | ⏳ |

UI/TUI pages for the v0.4.0 features ship in v0.5.0 Shuna.

## New iii Functions

```
rimuru.budget.check                rimuru.runaway.analyze
rimuru.budget.status               rimuru.runaway.scan
rimuru.budget.set                  rimuru.runaway.configure
rimuru.budget.alerts

rimuru.guard.register              rimuru.mcp.proxy.stats (extended)
rimuru.guard.complete
rimuru.guard.list
rimuru.guard.history
```

## New CLI Commands

```bash
rimuru guard start --limit N --action kill|warn -- <cmd> <args...>
rimuru guard status
rimuru guard history
```

## Install

```bash
curl -fsSL https://raw.githubusercontent.com/rohitg00/rimuru/main/install.sh | bash
```

## Upgrade Notes

- `budget_monthly`, `budget_daily`, `budget_session`, `budget_daily_agent`, and `budget_alert_threshold` all default to `0.0` (disabled). Set them explicitly to enable enforcement
- `budget_action` defaults to `alert` — switch to `block` to halt cost recording when caps are crossed
- Budget check race window is acknowledged: concurrent `rimuru.costs.record` calls snapshot state before writing, so tight bursts can collectively land just over a cap. A compare-and-swap primitive in iii-state will close this gap in v0.5.0
