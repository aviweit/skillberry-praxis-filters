// SPDX-License-Identifier: MIT
// Copyright (c) 2024 Skillberry Contributors

//! Unit tests for the VMCP manager filter.

use super::VmcpManagerFilter;

#[test]
fn test_filter_creation() {
    let yaml = serde_yaml::from_str(
        r#"
store_base_url: "http://localhost:8000"
vmcp_name_template: "vmcp-{env_id}"
always_create: true
timeout_ms: 10000
"#,
    )
    .unwrap();

    let filter = VmcpManagerFilter::from_config(&yaml).unwrap();
    assert_eq!(filter.name(), "vmcp_manager");
}

#[test]
fn test_empty_store_url_fails() {
    let yaml = serde_yaml::from_str(
        r#"
store_base_url: ""
"#,
    )
    .unwrap();

    let err = VmcpManagerFilter::from_config(&yaml)
        .err()
        .expect("should fail on empty store_url");
    assert!(err.to_string().contains("must not be empty"));
}
