// SPDX-License-Identifier: MIT
// Copyright (c) 2024 Skillberry Contributors

//! Configuration types for the VMCP manager filter.

use serde::Deserialize;

/// Configuration for the VMCP manager filter.
#[derive(Debug, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub(super) struct VmcpManagerConfig {
    /// Base URL of the skillberry-store service.
    pub store_base_url: String,

    /// Template for VMCP server names. Use {env_id} as placeholder.
    #[serde(default = "default_vmcp_name_template")]
    pub vmcp_name_template: String,

    /// Always create a new VMCP server.
    #[serde(default = "default_always_create")]
    pub always_create: bool,

    /// HTTP request timeout in milliseconds.
    #[serde(default = "default_timeout_ms")]
    pub timeout_ms: u64,

    /// Delete VMCP server if request processing fails.
    #[serde(default = "default_cleanup_on_error")]
    pub cleanup_on_error: bool,
}

fn default_vmcp_name_template() -> String {
    "vmcp-{env_id}".to_string()
}

fn default_always_create() -> bool {
    true
}

fn default_timeout_ms() -> u64 {
    10000
}

fn default_cleanup_on_error() -> bool {
    true
}
