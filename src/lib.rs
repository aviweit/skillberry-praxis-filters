// SPDX-License-Identifier: MIT
// Copyright (c) 2024 Skillberry Contributors

//! External Praxis filters for the Skillberry ecosystem.
//!
//! Provides four filters:
//! - `context_extractor`: Extracts request headers into filter metadata
//! - `skill_resolver`: Resolves skill UUIDs from environment variables
//! - `vmcp_manager`: Creates and manages Virtual MCP servers
//! - `mcp_tools_enricher`: Injects MCP tools into OpenAI-compatible request bodies

pub mod context_extractor;
pub mod mcp_tools_enricher;
pub mod skill_resolver;
pub mod vmcp_manager;

use praxis_filter::export_filters;

export_filters! {
    http "context_extractor" => context_extractor::ContextExtractorFilter::from_config,
    http "skill_resolver" => skill_resolver::SkillResolverFilter::from_config,
    http "vmcp_manager" => vmcp_manager::VmcpManagerFilter::from_config,
    http "mcp_tools_enricher" => mcp_tools_enricher::McpToolsEnricherFilter::from_config,
}
