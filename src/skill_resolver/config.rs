// SPDX-License-Identifier: MIT
// Copyright (c) 2024 Skillberry Contributors

//! Configuration types for the skill resolver filter.

use serde::Deserialize;

/// Configuration for the skill resolver filter.
#[derive(Debug, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub(super) struct SkillResolverConfig {
    /// Base URL of the skillberry-store service.
    pub store_base_url: String,

    /// Environment variable name containing the skill UUID.
    #[serde(default = "default_skill_uuid_env")]
    pub skill_uuid_env: String,

    /// Environment variable name containing the skill name.
    #[serde(default = "default_skill_name_env")]
    pub skill_name_env: String,

    /// HTTP request timeout in milliseconds.
    #[serde(default = "default_timeout_ms")]
    pub timeout_ms: u64,
}

fn default_skill_uuid_env() -> String {
    "SKILL_UUID".to_string()
}

fn default_skill_name_env() -> String {
    "SKILL_NAME".to_string()
}

fn default_timeout_ms() -> u64 {
    5000
}
