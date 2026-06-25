// SPDX-License-Identifier: MIT
// Copyright (c) 2024 Skillberry Contributors

//! Configuration types for the context extractor filter.

use serde::Deserialize;

/// Deserialized YAML config for the context extractor filter.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct ContextExtractorConfig {
    /// List of headers to extract.
    pub headers: Vec<HeaderExtractionRule>,

    /// Optional validation rules applied to all extracted values.
    #[serde(default)]
    pub validation: Option<ValidationRules>,
}

/// Configuration for extracting a single header.
#[derive(Debug, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub(super) struct HeaderExtractionRule {
    /// HTTP header name to extract (case-insensitive).
    pub name: String,

    /// Metadata key to store the extracted value under in `filter_metadata`.
    pub metadata_key: String,

    /// Default value if the header is absent.
    #[serde(default)]
    pub default: Option<String>,

    /// Whether this header is required. Returns 400 if missing and no default.
    #[serde(default)]
    pub required: bool,

    /// Optional per-header regex validation pattern.
    #[serde(default)]
    pub pattern: Option<String>,

    /// Optional per-header maximum value length.
    #[serde(default)]
    pub max_length: Option<usize>,
}

/// Global validation rules applied to all extracted header values.
#[derive(Debug, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub(super) struct ValidationRules {
    /// Maximum length for any extracted value.
    #[serde(default)]
    pub max_length: Option<usize>,

    /// Regex pattern that all values must match.
    #[serde(default)]
    pub pattern: Option<String>,
}
