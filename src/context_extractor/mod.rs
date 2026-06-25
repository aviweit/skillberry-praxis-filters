// SPDX-License-Identifier: MIT
// Copyright (c) 2024 Skillberry Contributors

//! Context extractor filter: maps request headers into `filter_metadata`
//! for consumption by downstream filters (`skill_resolver`, `vmcp_manager`).

mod config;
mod filter;

#[cfg(test)]
mod tests;

pub use filter::ContextExtractorFilter;
