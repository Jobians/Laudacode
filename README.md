<div align="center">

# Laudacode

**A fast, lightweight AI coding agent for your terminal.**
Pure Rust, no Node.js, tiny binary, built for Termux.

<img src="./img/laudacode.jpg" alt="Laudacode" width="100%"/>

[![Rust](https://img.shields.io/badge/Rust-1.75%2B-orange)](https://rust-lang.org)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Platform](https://img.shields.io/badge/platform-Termux%20%7C%20Linux%20%7C%20macOS-green)]()

</div>

---

Laudacode is an AI agent that lives in your terminal. It reads your project,
edits files, runs commands and fetches web docs — all under your approval —
using **any OpenAI-compatible API**: OpenAI, OpenRouter, Groq, DeepSeek,
Together, Ollama, LM Studio, llama.cpp server, vLLM…

## Features

- ⚡ **Pure Rust + Tokio + reqwest (rustls)** — one small static-ish binary, for Android/Termux
- 🔌 **Any OpenAI-compatible endpoint** — custom `base_url` / `api_key` / `model`
- 🧠 **Agentic tool loop** — `list_dir`, `read_file`, `write_file`, `edit_file`, `apply_patch`, `run_command`, `fetch_url`, `web_search`, `grep`, `glob`, `git`, `update_plan`
- 🔎 **Regex search** — `grep` is literal by default, pass `"regex": true` for a pattern; honors `ignore_case`, `context`, and has a compiled-size cap
- 📊 **Read-only git** — one `git` tool, fixed allowlist (`status`, `log`, `diff`, `show`, `blame`), direct argv, no shell
- 🪝 **Post-edit hooks** — `[hooks] post_edit` runs formatters/linters/tests after every file mutation; exit status and output go back to the model
- 💸 **Token & cost guardrails** — `[limits] max_tokens` / `max_cost_usd`; `/status` shows what's left
- 🧾 **Structured logs** — optional JSONL trail of tool calls, usage, hook results, budget blocks; never prompts or file contents
- 🌐 **Proxy & custom CAs** — `[network] proxy` (http/https/socks5), `ca_bundle`, explicit `insecure`
- 🚦 **Patient about flaky networks** — up to 5 retries by default on dropped connections and 408/409/429/5xx, jittered exponential backoff honoring `Retry-After`. Waits show in the status line, `Esc` cancels instantly, tune with `[limits] max_retries`
- ⚙️ **Managed processes** — `start_process` / `poll_process` / `write_process` / `stop_process` for dev servers, watchers, REPLs; 16 per agent, 8 KiB per stream, process-group cleanup
- 🌐 **Web built in** — `fetch_url` plus `web_search` (DuckDuckGo)
- 🛡️ **Approval modes** — `suggest`, `auto-edit`, `full-auto`, with confirmation for dangerous commands
- 🖼️ **Image input** — screenshots and photos for vision models (`-i`, `/image`)
- 📡 **Streaming responses** with reasoning-model support
- 🎨 **13 color themes** — lauda, cherry, midnight, nord, dracula, monokai, solarized, gruvbox, tokyo, everforest, ember, ice, hacker (`/theme`)
- ✨ **Ambient effects** — petals, rain, snow, matrix rain, lightning, stars, fireflies, bubbles, embers, confetti, comets, aurora (`/effect`, banner only)
- 🌈 **Syntax highlighting** — code blocks and diffs, per language (rust, python, js/ts, go, c/cpp, java, sh, toml, yaml, json)
- ⌨️ **Prompt history** — ↑/↓ recall with draft restore, persisted
- 💬 **Slash commands & input sugar** — `/provider`, `/model`, `/diff`, `/review`, `/undo`, `/compact`, `/init`, `/status`, `/export`, `/resume`, `/retry`…, plus `@file`, `#note` and `!<cmd>`
- ↩️ **Multi-turn undo** — `/undo N` reverts the last N agent turns
- 🔎 **Session cost tracking** — cumulative tokens and estimated USD in `/status`
- 📱 **Adaptive mobile UI** — banner art collapses to a one-line header on small screens
- 👆 **Touch-native** — swipe to scroll, tap picker rows, tap the composer to move the caret
- 🛠️ **Live activity feedback** — footer shows `reasoning` / `streaming` / `running <tool>` plus elapsed time
- 📄 **AGENTS.md support** — project instructions auto-loaded (`/init` generates one)
- 👤 **Profiles** — named presets in `[profiles.<name>]`, via `--profile`
- 💾 **Session persistence** — `--continue`, `--resume <id>`, `/resume`; ids, unique prefixes and names all work
- 🌿 **Checkpoints & branching** — `/checkpoint [label]`, `/checkpoints`, `/branch <n|id>`
- 🖥️ **One-shot mode** — `laudacode exec "fix the failing test"`
- 📦 **JSON output** — `--json` emits machine-readable event lines

## Install

### One-liner (Termux / Linux / macOS)

```sh
curl -fsSL https://raw.githubusercontent.com/Anon4You/Laudacode/main/install.sh | sh
```

Installs the **latest GitHub release** (auto-detected), builds it on-device and
puts `laudacode` in `$PREFIX/bin` — no sudo inside Termux. Needs `curl`, `tar`,
`rust` (Termux: `pkg install curl tar rust`). Pin an older release with
`LAUDACODE_VERSION=v0.2.0`.

### Termux / Android (manual)

```sh
pkg update && pkg install rust git -y
git clone https://github.com/Anon4You/Laudacode.git
cd Laudacode
cargo build --release
cp target/release/laudacode $PREFIX/bin/
```

> Low-RAM phone? `CARGO_PROFILE_RELEASE_LTO=off cargo build --release`

### Linux / macOS

```sh
cargo install --locked --git https://github.com/Anon4You/Laudacode
# or from a clone:
cargo install --locked --path .
```

## Quick start

```sh
export OPENAI_API_KEY="sk-or-v1-..."          # any OpenAI-compatible key
export OPENAI_BASE_URL="https://openrouter.ai/api/v1"
export OPENAI_MODEL="stealth/ox-alpha"

laudacode                                     # interactive REPL
```

Or skip env vars and configure from inside the TUI: first run has no wizard,
just type `/provider`. It opens an interactive menu (**add · use · edit · list**):
pick **add**, choose a preset (openrouter, tokenrouter, openai, groq, deepseek,
together, ollama, ollamacloud, lmstudio…), paste your key, pick a model from the
live catalog. Nothing is saved until a live test request proves key and model
work — same rule for the CLI flow.

**Keyless free providers** — `powerbrain` and `aitopia` need no API key and no
model picker: picking one saves immediately with a built-in default model.
`powerbrain` is the built-in default, so with nothing configured Laudacode works
out of the box, chat-first.

```sh
laudacode provider add                        # guided setup (name, url, key, model)
laudacode provider list
laudacode provider use tokenrouter
laudacode
```

One-shot tasks:

```sh
laudacode exec "explain what this repo does"
laudacode exec "add input validation to src/main.rs" --mode full-auto
```

## Configuration

Precedence: **CLI flags > profile (`--profile`) > environment variables > config file**.

| Variable         | Meaning                    |
|------------------|----------------------------|
| `OPENAI_API_KEY` | API key                    |
| `OPENAI_BASE_URL`| e.g. `https://api.groq.com/openai/v1` |
| `OPENAI_MODEL`   | model name                 |

Config file at `~/.config/laudacode/config.toml`
(or `.json`; override location with `LAUDACODE_CONFIG`):

```toml
active_provider = "openrouter"
approval_mode   = "suggest"

[providers.openrouter]
base_url = "https://openrouter.ai/api/v1"
api_key  = "sk-or-v1-..."
model    = "stealth/ox-alpha"

[providers.openrouter.headers]        # optional custom headers
"HTTP-Referer" = "https://github.com/Anon4You/Laudacode"
"X-Title"      = "Laudacode"

[profiles.fast]                       # optional presets → laudacode --profile fast
provider = "groq"
model    = "llama-3.3-70b-versatile"

[limits]                              # stop runaway loops
max_tokens   = 500000
max_cost_usd = 5.0

[hooks]                               # run after every file mutation
post_edit = ["cargo fmt", "cargo clippy --quiet -- -D warnings"]
post_edit_timeout_secs = 30

[logging]                             # JSONL trail, no prompts or file contents
enabled = true                        # no `file` → stderr
file  = "~/.local/share/laudacode/session.jsonl"

[network]                             # applies to every outgoing request
proxy     = "http://127.0.0.1:8080"  # http / https / socks5
ca_bundle = "~/.config/laudacode/corp.pem"
# insecure = true                     # skip TLS verification (risky)
```

Post-edit hooks get the touched paths in `$LAUDACODE_CHANGED_FILES`
(space-separated). Each hook's exit status and first output line go back to the
model, so a failing test run is something it can see and fix. Only
file-mutating tools trigger them.

Provider presets (OpenAI, OpenRouter, Groq, DeepSeek, Ollama, Ollama Cloud,
LM Studio) and annotated versions of every block above:
[`config.example.toml`](config.example.toml).

## Approval modes

Default is **BUILD** (`auto-edit`).

| `--mode` value          | TUI label  | File edits | Shell commands | Dangerous commands |
|--------------------------|------------|------------|----------------|--------------------|
| `suggest` (alias `ask`)  | PLAN       | ask        | ask            | ask                |
| `auto-edit` *(default)*  | BUILD      | ✅ auto    | ask            | ask                |
| `full-auto` (alias `yolo`)| FULL AUTO | ✅ auto    | ✅ auto        | ask (always)       |

Answer `[a]always` on any prompt to auto-approve the rest of the session. In the
TUI, switch modes any time with `/approvals` or **Tab**.

## CLI reference

```
laudacode                          # interactive session
laudacode "quick question"         # one-shot prompt
laudacode exec "<task>"            # same as above
laudacode exec "<task>" --json     # emit JSON event lines instead of prose
laudacode -P groq -m llama-3.3-70b-versatile
laudacode --profile fast           # activate [profiles.fast] from your config
laudacode -i screenshot.png "what's wrong with this UI?"
laudacode --base-url http://localhost:11434/v1 --api-key ollama --model qwen2.5-coder:7b
laudacode -c                       # continue last session
laudacode -y                       # shorthand for --mode full-auto
laudacode provider add|list|use|edit|remove <name>
laudacode session checkpoints <id>            # list a session's checkpoints
laudacode session branch <id> <checkpoint>     # fork a new session from one
```

## Slash commands

| Command              | Description                                  |
|----------------------|----------------------------------------------|
| `/help`              | command overview                             |
| `/model`             | pick a model from the provider's live list   |
| `/approvals`         | switch approval mode (plan/build/full-auto)  |
| `/provider …`        | manage providers (`add` `list` `show` `use <name>`) |
| `/theme`             | switch color theme (live preview)            |
| `/effect`            | ambient effects (petals · rain · lightning…) |
| `/status`            | provider/model/session + token, cost, budget, hook, log & network state |
| `/session …`         | `rename` · `search` · `list` · `delete` sessions |
| `/checkpoint [label]`| snapshot the conversation as a branch point    |
| `/checkpoints`       | list this session's checkpoints                |
| `/branch <n\|id>`    | fork a new session from a checkpoint           |
| `/skills`            | searchable picker — pick a skill to stage it in the composer |
| `/diff`              | git diff of working tree                     |
| `/review`            | AI review of the current git diff            |
| `/undo [N]`          | revert file changes from the last N turns    |
| `/init`              | generate AGENTS.md for this project          |
| `/compact`           | summarize history to free context window     |
| `/clear`             | fresh conversation                           |
| `/retry`             | re-run the previous task                     |
| `/resume`            | restore a previous session by id             |
| `/image <path>`      | attach an image to your next message         |
| `/export`            | save transcript as markdown                  |
| `/quit`              | exit                                         |

Input prefixes:

| Prefix      | Effect                                        |
|-------------|-----------------------------------------------|
| `@file`     | attach a file — its contents are inlined into the prompt |
| `#note`     | save a memory into AGENTS.md                  |
| `!<command>`| run a shell command locally (no agent)        |

Keys: type `/` for autocomplete, `↑/↓` + `Tab`/`Enter` to complete,
`Ctrl+O` expands recent tool output, `Esc` interrupts the agent.

**Touch / mouse** — swipe or wheel to scroll the transcript, **tap a picker row**
to choose it (models, themes, sessions, providers…), and **tap inside the
composer** to move the caret to that position. `Ctrl+B` toggles the banner, and
the hint strip under the composer adapts to narrow windows automatically.

## License

MIT — see [LICENSE](LICENSE).

<div align="center"><sub>Built with ⚡ by Anon4You</sub></div>
