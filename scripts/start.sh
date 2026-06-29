#!/usr/bin/env bash
# Start the Skillberry Agent Proxy (Praxis).
#
# Usage:
#   ./scripts/start.sh [--config-only]
#
# --config-only  Expand the template and exit without starting Praxis.
#                Useful for inspecting the generated config before launch.
#
# Required environment variables (Praxis owns these):
#   SKILL_UUID / SKILL_NAME           — which skill to activate
#   ENABLE_THINK_LOGS                 — default: false
#   USE_AGENT_TOOLS                   — default: true
#   USE_AGENT_PROMPTS                 — default: true
#   MCP_PROMPTS_POSITION              — default: postfix
#   REACT_RECURSION_LIMIT             — default: 20
#   SKILLBERRY_TOOLS_URL              — default: http://127.0.0.1:8000
#   RITS_API_KEY / OPENAI_API_KEY / LITELLM_MASTER_KEY  — provider credentials
#
# Worker environment variables (the worker process reads these itself):
#   LLM_BASE_URL        — default: http://127.0.0.1:8081/v1
#   WORKER_LOG_LEVEL    — default: INFO
#   WORKER_LOG_FILE     — default: /tmp/worker.log
#   WORKER_PORT         — default: 8001

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "${SCRIPT_DIR}/.." && pwd)"

TMPL="${REPO_ROOT}/pipeline/skillberry-agent-proxy.yaml.tmpl"
CONF="${REPO_ROOT}/pipeline/skillberry-agent-proxy.yaml"

echo "Expanding pipeline template..."
envsubst < "${TMPL}" > "${CONF}"
echo "Generated: ${CONF}"

if [[ "${1:-}" == "--config-only" ]]; then
    echo "Config-only mode — exiting without starting Praxis."
    exit 0
fi

exec praxis --config "${CONF}"
