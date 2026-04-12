<p align="center">
  <img src="docs/assets/rimuru-banner.svg" alt="Rimuru - AI agent cost monitor" width="720" />
</p>

<p align="center">
  <strong>Cost control for AI coding agents.</strong><br/>
  Budget guardrails, runaway loop detection, process wrappers, and output compression for every agent on your machine.
</p>

<p align="center">
  <a href="https://github.com/rohitg00/rimuru/releases"><img src="https://img.shields.io/github/v/release/rohitg00/rimuru?style=for-the-badge&color=6366F1&label=release" alt="Release" /></a>
  <a href="https://github.com/rohitg00/rimuru/actions/workflows/ci.yml"><img src="https://img.shields.io/github/actions/workflow/status/rohitg00/rimuru/ci.yml?style=for-the-badge&label=ci&logo=github" alt="CI" /></a>
  <a href="https://opensource.org/licenses/Apache-2.0"><img src="https://img.shields.io/badge/license-Apache_2.0-blue?style=for-the-badge" alt="License" /></a>
  <a href="https://www.rust-lang.org"><img src="https://img.shields.io/badge/rust-1.85+-orange?style=for-the-badge&logo=rust&logoColor=white" alt="Rust" /></a>
  <a href="https://github.com/rohitg00/rimuru/stargazers"><img src="https://img.shields.io/github/stars/rohitg00/rimuru?style=for-the-badge&color=yellow&logo=github" alt="Stars" /></a>
</p>

<p align="center">
  <picture><source media="(prefers-color-scheme: dark)" srcset="docs/assets/tags/light/stat-agents.svg"><img src="docs/assets/tags/stat-agents.svg" alt="8 agents tracked" height="48" /></picture>
  <picture><source media="(prefers-color-scheme: dark)" srcset="docs/assets/tags/light/stat-functions.svg"><img src="docs/assets/tags/stat-functions.svg" alt="60+ iii functions" height="48" /></picture>
  <picture><source media="(prefers-color-scheme: dark)" srcset="docs/assets/tags/light/stat-interfaces.svg"><img src="docs/assets/tags/stat-interfaces.svg" alt="4 interfaces" height="48" /></picture>
  <picture><source media="(prefers-color-scheme: dark)" srcset="docs/assets/tags/light/stat-caps.svg"><img src="docs/assets/tags/stat-caps.svg" alt="4 budget cap levels" height="48" /></picture>
  <picture><source media="(prefers-color-scheme: dark)" srcset="docs/assets/tags/light/stat-strategies.svg"><img src="docs/assets/tags/stat-strategies.svg" alt="6 compression strategies" height="48" /></picture>
  <picture><source media="(prefers-color-scheme: dark)" srcset="docs/assets/tags/light/stat-endpoints.svg"><img src="docs/assets/tags/stat-endpoints.svg" alt="45+ http endpoints" height="48" /></picture>
</p>

<p align="center">
  <picture><source media="(prefers-color-scheme: dark)" srcset="docs/assets/tags/light/pill-v040.svg"><img src="docs/assets/tags/pill-v040.svg" alt="v0.4.0 Benimaru" height="24" /></picture>
  <picture><source media="(prefers-color-scheme: dark)" srcset="docs/assets/tags/light/pill-rust.svg"><img src="docs/assets/tags/pill-rust.svg" alt="Rust 1.85" height="24" /></picture>
  <picture><source media="(prefers-color-scheme: dark)" srcset="docs/assets/tags/light/pill-iii.svg"><img src="docs/assets/tags/pill-iii.svg" alt="iii-engine" height="24" /></picture>
  <picture><source media="(prefers-color-scheme: dark)" srcset="docs/assets/tags/light/pill-tauri.svg"><img src="docs/assets/tags/pill-tauri.svg" alt="Tauri v2" height="24" /></picture>
  <picture><source media="(prefers-color-scheme: dark)" srcset="docs/assets/tags/light/pill-apache2.svg"><img src="docs/assets/tags/pill-apache2.svg" alt="Apache 2.0" height="24" /></picture>
</p>

<p align="center">
  <img src="docs/assets/rimuru-ui.gif" alt="Rimuru Web UI" width="720" />
</p>

<p align="center">
  <a href="#quick-start">Quick start</a> &bull;
  <a href="#budget-engine">Budget</a> &bull;
  <a href="#runaway-detection">Runaway</a> &bull;
  <a href="#guard-wrapper">Guard</a> &bull;
  <a href="#output-compression">Compression</a> &bull;
  <a href="#interfaces">Interfaces</a> &bull;
  <a href="#architecture">Architecture</a> &bull;
  <a href="#api">API</a>
</p>

<br/>

<h2 id="why-rimuru"><picture><source media="(prefers-color-scheme: dark)" srcset="docs/assets/tags/light/section-why.svg"><img src="docs/assets/tags/section-why.svg" alt="Why Rimuru" height="44" /></picture></h2>

One developer. Four agents running in parallel. Each one racking up API spend you don't see until the monthly invoice lands. Rimuru is the control plane between you and that bill.

- **Hard caps**. Monthly, daily, per-session, per-agent daily. Exceeded caps halt cost recording when `budget_action = block`.
- **Loop detection**. Four patterns (repeated calls, repeated errors, token explosion, oscillation) with severity scoring and wasted-token estimates.
- **Process wrappers**. `rimuru guard start --limit 5 --action kill -- claude` kills the child when it overspends.
- **Output compression**. MCP tool results over 2000 tokens get routed through one of six compression strategies before reaching the agent.
- **Zero dependencies**. No Postgres, no Redis, no Docker. One engine, one worker, one dashboard.

Built on the [iii-engine](https://github.com/iii-hq/iii) primitive set (Worker / Function / Trigger). All state lives in an in-memory KV backed by the engine.

<br/>

<h2 id="quick-start"><picture><source media="(prefers-color-scheme: dark)" srcset="docs/assets/tags/light/section-quickstart.svg"><img src="docs/assets/tags/section-quickstart.svg" alt="Quick start" height="44" /></picture></h2>

One-line install for Linux and macOS. Installs the iii engine if missing, drops four binaries in `~/.local/bin`.

```bash
curl -fsSL https://raw.githubusercontent.com/rohitg00/rimuru/main/install.sh | bash
```

Run the engine, start the worker, open the dashboard.

```bash
iii                       # start the iii engine
rimuru-worker             # start the worker
open http://localhost:3100
```

Detect installed agents and see what you're spending.

```bash
rimuru agents detect
rimuru costs summary
rimuru budget status
```

Other platforms: [pre-built binaries](https://github.com/rohitg00/rimuru/releases), [Docker image](https://github.com/rohitg00/rimuru/pkgs/container/rimuru), or [build from source](#development).

<br/>

<h2 id="works-with-every-agent"><picture><source media="(prefers-color-scheme: dark)" srcset="docs/assets/tags/light/section-agents.svg"><img src="docs/assets/tags/section-agents.svg" alt="Works with every agent" height="44" /></picture></h2>

Rimuru auto-discovers agents from their local config paths. No token swap, no auth dance, no proxy — it reads session history the same way each tool stores it.

<table>
<tr>
<td align="center" width="16.6%">
<a href="https://claude.com/product/claude-code"><img src="https://github.com/anthropics.png?size=120" alt="Claude Code" width="48" height="48" /></a><br/>
<strong>Claude Code</strong><br/>
<sub><code>~/.claude/</code></sub>
</td>
<td align="center" width="16.6%">
<a href="https://cursor.com"><img src="https://github.com/cursor.png?size=120" alt="Cursor" width="48" height="48" /></a><br/>
<strong>Cursor</strong><br/>
<sub><code>~/Library/Application Support/Cursor/</code></sub>
</td>
<td align="center" width="16.6%">
<a href="https://github.com/features/copilot"><img src="https://github.com/github.png?size=120" alt="GitHub Copilot" width="48" height="48" /></a><br/>
<strong>GitHub Copilot</strong><br/>
<sub>VS Code extension</sub>
</td>
<td align="center" width="16.6%">
<a href="https://github.com/openai/codex"><img src="https://github.com/openai.png?size=120" alt="Codex" width="48" height="48" /></a><br/>
<strong>Codex</strong><br/>
<sub><code>~/.config/codex/</code></sub>
</td>
<td align="center" width="16.6%">
<a href="https://github.com/block/goose"><img src="https://github.com/block.png?size=120" alt="Goose" width="48" height="48" /></a><br/>
<strong>Goose</strong><br/>
<sub><code>~/.config/goose/</code></sub>
</td>
<td align="center" width="16.6%">
<a href="https://github.com/opencode-ai/opencode"><img src="https://github.com/opencode-ai.png?size=120" alt="OpenCode" width="48" height="48" /></a><br/>
<strong>OpenCode</strong><br/>
<sub><code>~/.opencode/</code></sub>
</td>
</tr>
</table>

Pricing is maintained for 8 models across 5 providers. Cost records are idempotent — re-syncing the same session overwrites rather than duplicates.

<br/>

<h2 id="budget-engine"><picture><source media="(prefers-color-scheme: dark)" srcset="docs/assets/tags/light/section-budget.svg"><img src="docs/assets/tags/section-budget.svg" alt="Budget engine" height="44" /></picture></h2>

Four cap levels, one enforcement path. Every cost record runs through `rimuru.budget.check` *before* it hits state. When caps are exceeded and `budget_action = block`, the recording is rejected and the error bubbles back to the caller.

```bash
rimuru config set budget_monthly 50
rimuru config set budget_daily 5
rimuru config set budget_session 2
rimuru config set budget_daily_agent 10
rimuru config set budget_action block
```

| Field                  | Default | What it caps                                           |
|------------------------|---------|--------------------------------------------------------|
| `budget_monthly`       | `0.0`   | Total spend for the current calendar month             |
| `budget_daily`         | `0.0`   | Total spend for today                                  |
| `budget_session`       | `0.0`   | Spend for a single session_id                          |
| `budget_daily_agent`   | `0.0`   | Per-agent spend for today                              |
| `budget_alert_threshold` | `0.8` | Warning fires at this fraction of any cap              |
| `budget_action`        | `alert` | `alert` (log + hook) or `block` (reject the record)    |

Status query projects end-of-month spend using the actual days in the current month.

```bash
rimuru budget status
#  monthly_limit: $50.00    monthly_spent: $18.42    monthly_remaining: $31.58
#  daily_limit:   $5.00     daily_spent:   $1.37     daily_remaining:   $3.63
#  burn_rate:     $1.84/day projected_monthly: $55.20 ( over cap )
```

Threshold crossings fire the `budget.warning` hook. Cap breaches fire `budget.exceeded`. Alerts are persisted with millisecond + UUID keys so burst alerts don't collide.

**Fail-open on service outage.** If the budget check itself is unreachable, cost recording logs a warning and proceeds. Only a successful *"exceeded + block"* response halts the write.

<br/>

<h2 id="runaway-detection"><picture><source media="(prefers-color-scheme: dark)" srcset="docs/assets/tags/light/section-runaway.svg"><img src="docs/assets/tags/section-runaway.svg" alt="Runaway detection" height="44" /></picture></h2>

Four detection patterns run over the last N turns of a session. Severity scoring tells you how stuck it is. Token accounting tells you how much was wasted.

| Pattern            | Trigger                                                          | Severity        |
|--------------------|------------------------------------------------------------------|-----------------|
| `repeated_calls`   | Same tool name repeated > `runaway_repeat_threshold` times       | `min(1, n/10)`  |
| `repeated_errors`  | Identical `content_type + role` signature > threshold times      | `min(1, n/8)`   |
| `token_explosion`  | Last 3 turns > `ratio` × baseline input tokens (baseline excludes last 3) | `min(1, (ratio-1)/4)` |
| `oscillation`      | Two tools alternating > 4 times                                  | `min(1, n/8)`   |

```bash
rimuru.runaway.analyze  '{"session_id": "...", "window": 15}'
rimuru.runaway.scan     '{}'
rimuru.runaway.configure '{"repeat_threshold": 4, "token_explosion_ratio": 2.5}'
```

The token-explosion baseline *excludes* the last 3 turns so spike samples never inflate the average they're compared against. Invalid thresholds (zero window, ratio ≤ 1.0, non-boolean `auto_scan_enabled`) are rejected at write time.

<br/>

<h2 id="guard-wrapper"><picture><source media="(prefers-color-scheme: dark)" srcset="docs/assets/tags/light/section-guard.svg"><img src="docs/assets/tags/section-guard.svg" alt="Guard wrapper" height="44" /></picture></h2>

Wrap any agent process in a cost limit. The wrapper polls cost totals every five seconds from the guard start time and either warns or kills the child when the limit is crossed.

```bash
rimuru guard start --limit 5.00 --action kill -- claude
rimuru guard start --limit 2.00 --action warn -- cursor --cli
rimuru guard status
rimuru guard history
```

- **Spawn first, register second.** If `Command::spawn()` fails, nothing gets written to KV. The register payload includes the real PID.
- **Scoped cost query.** `rimuru.costs.summary` is called with `since = started_at` — global totals from yesterday don't trip your guard today.
- **Stderr diagnostics.** The wrapper banner, warnings, and summary go to `stderr` so they don't corrupt the wrapped process's `stdout`.
- **Atomic completion.** History is written *before* the active guard is deleted. A half-failed write leaves the ledger consistent.
- **Typed flags.** `--action` is a `clap::ValueEnum` (`kill` or `warn`). `--limit` rejects NaN, infinity, and non-positive values at parse time.

<br/>

<h2 id="output-compression"><picture><source media="(prefers-color-scheme: dark)" srcset="docs/assets/tags/light/section-compression.svg"><img src="docs/assets/tags/section-compression.svg" alt="Output compression" height="44" /></picture></h2>

The MCP proxy compresses tool responses over 2000 tokens before they reach the agent. `Auto` mode inspects the payload shape and routes to the strategy that fits.

| Strategy     | Picked when                                           | What it does                                    |
|--------------|-------------------------------------------------------|--------------------------------------------------|
| `Auto`       | default — inspects content                           | Routes to one of the strategies below            |
| `JsonPaths`  | object or array with > 50 keys or > 5K tokens        | Shallow keys kept; deep nodes → `{__depth, __keys}`; long arrays → head + tail + truncation marker |
| `ErrorsOnly` | string with error / warning / fail / panic lines     | Keeps matched lines plus 3 lines of prior context |
| `TreeView`   | string that looks like a file listing                | Paths rendered as an indented tree               |
| `Summarize`  | plain text                                            | First 10 + last 5 lines, removes the middle      |
| `Truncate`   | fallback                                              | Char-safe truncation at `~max_tokens × 3` chars  |

All string truncation uses `char_indices().nth()` — multi-byte characters can't split mid-codepoint. Max-char arithmetic uses `saturating_mul` so extreme `max_tokens` can't wrap. If a smart strategy doesn't drop below the cap, the fallback is `Truncate`.

```bash
rimuru mcp stats
# github::issues_list    calls=42   saved=18,924 tokens   compressed=12
# filesystem::read_file  calls=117  saved=61,302 tokens   compressed=48
```

<br/>

<h2 id="interfaces"><picture><source media="(prefers-color-scheme: dark)" srcset="docs/assets/tags/light/section-interfaces.svg"><img src="docs/assets/tags/section-interfaces.svg" alt="Four interfaces" height="44" /></picture></h2>

Every feature is reachable from four front-ends. Pick the one that fits the task.

| Interface | Binary | What it's for |
|-----------|--------|--------------|
| **Web UI** | embedded in `rimuru-worker` on `:3100` | 13 pages including Dashboard, Agents, Sessions, Costs, Models, Advisor, Context, MCP Proxy, City, Plugins, Hooks, Settings, Terminal. Single-file React build, ~1MB, served directly by the worker. |
| **CLI** | `rimuru` | 13 command groups, `--format table\|json\|yaml`, direct iii trigger calls (no HTTP middleman). |
| **TUI** | `rimuru-tui` | Ratatui 0.29 with 12 tabs, 15 themed color schemes, keyboard nav (`Tab`, `j`/`k`, `1-9`, `/`, `t`, `?`). |
| **Desktop** | `rimuru-desktop` | Tauri v2 native app with embedded worker, 46 IPC commands across 10 modules, system tray, global shortcut, window state persistence. |

<p align="center">
  <img src="docs/assets/rimuru-tui.gif" alt="Rimuru TUI" width="720" />
</p>

<br/>

<h2 id="hardware-advisor"><picture><source media="(prefers-color-scheme: dark)" srcset="docs/assets/tags/light/section-advisor.svg"><img src="docs/assets/tags/section-advisor.svg" alt="Hardware advisor" height="44" /></picture></h2>

The advisor detects CPU, RAM, and GPU (Metal / CUDA / ROCm) and maps every API model you use to a local equivalent from a 50+ model catalog. Fit is scored `Perfect`, `Good`, `Marginal`, or `Too Tight`, with tok/s estimates and projected monthly savings.

```bash
rimuru models advisor
#  Claude Sonnet 4   -> Qwen2.5-14B (Q4_K_M)   fit: Perfect     savings: $34/mo
#  Claude Opus 4.5   -> Llama 3.3 70B (Q4_0)   fit: Marginal    savings: $128/mo
#  GPT-4o            -> Qwen2.5-32B (Q5_K_M)   fit: Good        savings: $41/mo
```

<br/>

<h2 id="architecture"><picture><source media="(prefers-color-scheme: dark)" srcset="docs/assets/tags/light/section-architecture.svg"><img src="docs/assets/tags/section-architecture.svg" alt="Architecture" height="44" /></picture></h2>

```
                iii Engine (WS :49134)
                        │
         ┌──────────────┼──────────────┐
         │              │              │
   rimuru-worker   rimuru-cli    rimuru-desktop
   (core + HTTP)   (iii client)  (Tauri v2 + embedded worker)
         │
   ┌─────┴─────┐
   │           │
  Web UI    rimuru-tui
  (:3100)   (HTTP client)
```

| Crate | Binary | What it is |
|-------|--------|------------|
| `rimuru-core` | `rimuru-worker` | iii worker with 60+ registered functions, 45+ HTTP endpoints via `iii-http`, budget / runaway / guard / compression / advisor modules, agent adapters, hook dispatcher, embedded web UI. |
| `rimuru-cli` | `rimuru` | Clap CLI. Connects to the engine via `register_worker`, invokes functions directly through `iii.trigger()`. No HTTP hop. |
| `rimuru-tui` | `rimuru-tui` | Ratatui 0.29 terminal UI. HTTP client against the worker. |
| `rimuru-desktop` | `rimuru-desktop` | Tauri v2 app with the worker embedded. 46 IPC commands that forward to the in-process engine. |

State lives in the engine's in-memory KV under scoped namespaces: `agents`, `sessions`, `cost_records`, `cost_daily`, `cost_agent`, `budgets`, `budget_alerts`, `guards`, `guard_history`, `mcp_servers`, `mcp_metrics`, `hooks`, `plugins`, `config`, `context_breakdowns`.

<br/>

<h2 id="api"><picture><source media="(prefers-color-scheme: dark)" srcset="docs/assets/tags/light/section-api.svg"><img src="docs/assets/tags/section-api.svg" alt="API reference" height="44" /></picture></h2>

Two call paths, same functions:

- **HTTP** via `iii-http` on `:3111` — `POST /api/budget/check`, `GET /api/agents`, etc. Payloads are normalized by `extract_input()` so path params, query params, and bodies merge into one `Value`.
- **Direct trigger** via the iii WebSocket — `iii.trigger(TriggerRequest { function_id: "rimuru.budget.check", ... })`. This is what the CLI uses.

Function namespaces:

```
rimuru.agents.*       list, get, create, update, delete, status, detect, connect, disconnect, sync
rimuru.sessions.*     list, get, active, history, cleanup
rimuru.costs.*        record, summary, daily, by_agent, daily_rollup
rimuru.budget.*       check, status, set, alerts
rimuru.runaway.*      analyze, scan, configure
rimuru.guard.*        register, complete, list, history
rimuru.mcp.proxy.*    connect, tools, call, search, stats, disconnect
rimuru.context.*      breakdown, breakdown_by_session, utilization, waste
rimuru.models.*       list, get, sync, advisor, catalog
rimuru.hooks.*        register, list, dispatch
rimuru.config.*       get, set
```

Full HTTP endpoint list is in [`docs/api.md`](docs/api.md).

<br/>

<h2 id="development"><picture><source media="(prefers-color-scheme: dark)" srcset="docs/assets/tags/light/section-development.svg"><img src="docs/assets/tags/section-development.svg" alt="Development" height="44" /></picture></h2>

```bash
git clone https://github.com/rohitg00/rimuru.git
cd rimuru

cd ui && npm ci && npm run build && cd ..    # single-file production UI
cargo build --release                         # all four crates

cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test --workspace
```

Binaries land in `target/release/`: `rimuru-worker`, `rimuru`, `rimuru-tui`, `rimuru-desktop`.

README SVG tag set is generated from `scripts/generate-readme-tags.py` — don't hand-edit the SVGs under `docs/assets/tags/`.

Pull requests welcome. Keep the no-emoji rule in commits, README, and section headers.

<br/>

<h2 id="license"><picture><source media="(prefers-color-scheme: dark)" srcset="docs/assets/tags/light/section-license.svg"><img src="docs/assets/tags/section-license.svg" alt="License" height="44" /></picture></h2>

Apache License 2.0. See [LICENSE](LICENSE).

Built on [iii-engine](https://github.com/iii-hq/iii) (Worker / Function / Trigger primitives). Terminal UI powered by [Ratatui](https://ratatui.rs). Desktop app powered by [Tauri](https://tauri.app).

<p align="center">
  Made by <a href="https://github.com/rohitg00">Rohit Ghumare</a>
</p>
