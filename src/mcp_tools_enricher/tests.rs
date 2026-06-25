// SPDX-License-Identifier: MIT
// Copyright (c) 2024 Skillberry Contributors

//! Unit tests for the MCP tools enricher filter.

use super::McpToolsEnricherFilter;

#[test]
fn test_filter_creation() {
    let yaml = serde_yaml::from_str(
        r#"
timeout_ms: 5000
tool_choice: auto
"#,
    )
    .unwrap();

    let filter = McpToolsEnricherFilter::from_config(&yaml).unwrap();
    assert_eq!(filter.name(), "mcp_tools_enricher");
}

#[test]
fn test_invalid_tool_choice_fails() {
    let yaml = serde_yaml::from_str(
        r#"
tool_choice: "invalid"
"#,
    )
    .unwrap();

    let err = McpToolsEnricherFilter::from_config(&yaml)
        .err()
        .expect("should fail on invalid tool_choice");
    assert!(err.to_string().contains("tool_choice"));
}

#[test]
fn test_zero_max_body_bytes_fails() {
    let yaml = serde_yaml::from_str(
        r#"
max_body_bytes: 0
"#,
    )
    .unwrap();

    let err = McpToolsEnricherFilter::from_config(&yaml)
        .err()
        .expect("should fail on zero max_body_bytes");
    assert!(err.to_string().contains("max_body_bytes"));
}
