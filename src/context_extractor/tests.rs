// SPDX-License-Identifier: MIT
// Copyright (c) 2024 Skillberry Contributors

//! Unit tests for the context extractor filter.

use super::ContextExtractorFilter;

#[test]
fn test_filter_creation() {
    let yaml = serde_yaml::from_str(
        r#"
headers:
  - name: x-skillberry-env-id
    metadata_key: env_id
    default: "default-env"
    required: true
    pattern: "^[a-zA-Z0-9_-]+$"
    max_length: 64
"#,
    )
    .unwrap();

    let filter = ContextExtractorFilter::from_config(&yaml).unwrap();
    assert_eq!(filter.name(), "context_extractor");
}

#[test]
fn test_empty_headers_fails() {
    let yaml = serde_yaml::from_str("headers: []").unwrap();
    let err = ContextExtractorFilter::from_config(&yaml)
        .err()
        .expect("should fail on empty headers");
    assert!(err.to_string().contains("must not be empty"));
}

#[test]
fn test_invalid_regex_fails() {
    let yaml = serde_yaml::from_str(
        r#"
headers:
  - name: x-env-id
    metadata_key: env_id
    pattern: "^[invalid"
"#,
    )
    .unwrap();
    let err = ContextExtractorFilter::from_config(&yaml)
        .err()
        .expect("should fail on invalid regex");
    assert!(err.to_string().contains("invalid regex pattern"));
}

#[test]
fn test_invalid_header_name_fails() {
    let yaml = serde_yaml::from_str(
        r#"
headers:
  - name: "invalid header name!"
    metadata_key: env_id
"#,
    )
    .unwrap();
    let err = ContextExtractorFilter::from_config(&yaml)
        .err()
        .expect("should fail on invalid header name");
    assert!(err.to_string().contains("invalid header name"));
}
