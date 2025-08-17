//! Embedded core D&D rules bundle
//! 
//! This module embeds the core D&D 5e rules bundle directly in the binary
//! to ensure offline functionality from first run.

/// The embedded core rules bundle (D&D 5e 2014 PHB, DMG, MM)
/// This is approximately 347KB compressed
pub const CORE_RULES_BUNDLE: &[u8] = include_bytes!("../assets/dnd5e-2014-core-v1.210.46.tar.gz");

/// The rule system ID for the core D&D 5e 2014 rules
pub const CORE_RULES_ID: &str = "dnd5e-2014";

/// The bundle ID for the core rules
pub const CORE_BUNDLE_ID: &str = "dnd5e-2014-core";