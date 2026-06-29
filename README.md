# skillberry-praxis-filters

> ⚠️ **Work in Progress** — This repository is actively evolving. Features, APIs, and configuration may change at any time.

External [Praxis](https://github.com/praxis-proxy/praxis) filters for the Skillberry ecosystem, plus the **Skillberry Worker** — the thin Python agentic service that sits behind Praxis.

## Architecture

```
Client
  │  POST /v1/chat/completions
  ▼
Praxis port 8080 — client-ingress
  │  context_extractor, headers, router
  │  (headers injects agent config from Praxis env vars)
  ▼
Skillberry Worker (port 8001)   ← Python / FastAPI — this repo: worker/
  │  LangGraph ReAct loop
  │  LLM calls loop back via Praxis port 8081
  ▼
Praxis port 8081 — llm-egress (loopback only)
  │  model_to_header, openai_responses_model_rewrite
  │  router, credential_injection, token_usage_headers
  ▼
LLM provider (RITS / OpenAI / Anthropic / Ollama)
```

See [`plan-docs/praxis-as-agent-gateway.md`](plan-docs/praxis-as-agent-gateway.md) for the full design.

---

## Praxis Filters (Rust)

Used in [`pipeline/skillberry-agent-proxy.yaml.tmpl`](pipeline/skillberry-agent-proxy.yaml.tmpl):

| Filter | Listener | Description |
|--------|----------|-------------|
| `context_extractor` | client-ingress | Extracts `x-skillberry-context-*` request headers into filter metadata |
| `headers` | client-ingress | Injects agent config env vars as `x-skillberry-*` headers into worker requests |
| `router` | both | Routes requests to the appropriate cluster |
| `load_balancer` | both | Forwards to upstream endpoints |
| `model_to_header` | llm-egress | Copies the `model` field to `X-Model` header for routing |
| `openai_responses_model_rewrite` | llm-egress | Rewrites LiteLLM SDK model name prefixes to provider-specific format |
| `credential_injection` | llm-egress | Injects LLM provider credentials from Praxis env vars |
| `token_usage_headers` | llm-egress | Injects `Praxis-Token-*` headers on responses for token usage tracking |

## Skillberry Worker (Python)

See [`worker/README.md`](worker/README.md) for quick-start instructions.

---

## Quickstart

### 1. Build the Praxis filters

Check out Praxis at the pinned commit:

```console
cd ~
git clone https://github.com/praxis-proxy/praxis.git praxis
cd praxis && git checkout 0bc9534e922a8be313331dd9f317356e5097d109
```

Add this crate as a dependency — three edits inside the Praxis checkout:

**`Cargo.toml`** (workspace root):

```toml
[workspace.dependencies]
skillberry-praxis-filters = { git = "https://github.com/skillberry-ai/skillberry-praxis-filters.git", branch = "main" }

[patch."https://github.com/praxis-proxy/praxis.git"]
praxis-proxy-filter = { path = "filter" }
```

**`server/Cargo.toml`**:

```toml
[dependencies]
skillberry-praxis-filters = { workspace = true }
```

Build:

```console
cargo update && cargo build --package praxis-proxy
```

### 2. Start the Skillberry Worker

```console
cd worker
pip install -e .
uvicorn worker.main:app --host 0.0.0.0 --port 8001
```

### 3. Start Praxis

```console
export SKILL_NAME="my-skill"          # or SKILL_UUID=<uuid>
export RITS_API_KEY="<your-key>"      # or OPENAI_API_KEY / LITELLM_MASTER_KEY
./scripts/start.sh
```

`scripts/start.sh` expands [`pipeline/skillberry-agent-proxy.yaml.tmpl`](pipeline/skillberry-agent-proxy.yaml.tmpl)
via `envsubst` and starts Praxis with the generated config.

### 4. Verify

```console
curl http://localhost:8080/health    # Praxis
curl http://localhost:8001/health    # Worker
```

---

## Repository Layout

```
Cargo.toml                              Rust crate (Praxis filters)
src/                                    Filter implementations (Rust)
  context_extractor/
worker/                                 Skillberry Worker (Python)
  pyproject.toml
  main.py / agentic_graph.py / llm_client.py
  README.md
pipeline/
  skillberry.yaml                       Legacy single-listener config
  skillberry-agent-proxy.yaml.tmpl      Two-listener template (current)
scripts/
  start.sh                              envsubst + praxis launcher
plan-docs/
  praxis-as-agent-gateway.md            Full architecture plan
```

## Environment Variables

**Praxis-owned** (set on the Praxis process; injected as headers into worker requests):

| Variable | Default | Description |
|----------|---------|-------------|
| `SKILL_UUID` | — | Direct skill UUID (highest priority) |
| `SKILL_NAME` | — | Skill name resolved via store API |
| `ENABLE_THINK_LOGS` | `false` | Include `<think>` block in response |
| `USE_AGENT_TOOLS` | `true` | Pass client-supplied tools to ReAct loop |
| `USE_AGENT_PROMPTS` | `true` | Pass system messages to ReAct loop |
| `MCP_PROMPTS_POSITION` | `postfix` | MCP prompt injection position |
| `REACT_RECURSION_LIMIT` | `20` | LangGraph ReAct max iterations |
| `SKILLBERRY_TOOLS_URL` | `http://127.0.0.1:8000` | Skillberry Tools Service URL |
| `RITS_API_KEY` / `OPENAI_API_KEY` / etc. | — | LLM provider credentials |

**Worker-owned** (set on the worker process):

| Variable | Default | Description |
|----------|---------|-------------|
| `LLM_BASE_URL` | `http://127.0.0.1:8081/v1` | Praxis llm-egress URL |
| `WORKER_LOG_LEVEL` | `INFO` | Log level |
| `WORKER_LOG_FILE` | `/tmp/worker.log` | Log file path |
| `WORKER_PORT` | `8001` | HTTP listen port |
