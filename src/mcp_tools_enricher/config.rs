// SPDX-License-Identifier: MIT
// Copyright (c) 2024 Skillberry Contributors

//! Configuration types for the MCP tools enricher filter.

use serde::Deserialize;

use praxis_filter::FilterError;

/// Default maximum request body size to buffer (10 MiB).
const DEFAULT_JSON_BODY_MAX_BYTES: usize = 10_485_760;

/// Absolute maximum request body size (64 MiB).
const MAX_JSON_BODY_BYTES: usize = 67_108_864;

/// Default timeout for MCP server connections in milliseconds.
const DEFAULT_TIMEOUT_MS: u64 = 5000;

/// Default tool choice value.
const DEFAULT_TOOL_CHOICE: &str = "auto";

/// Deserialized YAML config for the MCP tools enricher filter.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct McpToolsEnricherConfig {
    /// Maximum request body size to buffer before parsing.
    #[serde(default = "default_max_body_bytes")]
    pub max_body_bytes: usize,

    /// Behavior when the body is not valid JSON or cannot be enriched.
    #[serde(default)]
    pub on_invalid: InvalidBodyBehavior,

    /// Timeout for MCP server connection in milliseconds.
    #[serde(default = "default_timeout_ms")]
    #[allow(dead_code)]
    pub timeout_ms: u64,

    /// Tool choice value to set in the enriched request.
    #[serde(default = "default_tool_choice")]
    pub tool_choice: String,
}

fn default_max_body_bytes() -> usize {
    DEFAULT_JSON_BODY_MAX_BYTES
}

fn default_timeout_ms() -> u64 {
    DEFAULT_TIMEOUT_MS
}

fn default_tool_choice() -> String {
    DEFAULT_TOOL_CHOICE.to_owned()
}

/// Behavior when the request body cannot be enriched.
#[derive(Debug, Clone, Copy, Default, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub(super) enum InvalidBodyBehavior {
    /// Pass the original body through unchanged.
    #[default]
    Continue,
    /// Return HTTP 400.
    Reject,
}

/// Validate a parsed config.
pub(super) fn validate_config(cfg: &McpToolsEnricherConfig) -> Result<(), FilterError> {
    if cfg.max_body_bytes == 0 {
        return Err("mcp_tools_enricher: 'max_body_bytes' must be greater than zero".into());
    }
    if cfg.max_body_bytes > MAX_JSON_BODY_BYTES {
        return Err(format!(
            "mcp_tools_enricher: max_body_bytes ({}) exceeds maximum ({MAX_JSON_BODY_BYTES})",
            cfg.max_body_bytes
        )
        .into());
    }
    if cfg.timeout_ms == 0 {
        return Err("mcp_tools_enricher: 'timeout_ms' must be greater than zero".into());
    }
    match cfg.tool_choice.as_str() {
        "auto" | "required" | "none" => Ok(()),
        _ => Err(format!(
            "mcp_tools_enricher: 'tool_choice' must be one of: 'auto', 'required', 'none', got '{}'",
            cfg.tool_choice
        )
        .into()),
    }
}
