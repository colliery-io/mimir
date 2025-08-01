//! Minimal enums for workflow management
//!
//! This module contains only the enums necessary for the board/workflow system.
//! Content-related enums (Genre, Tone, etc.) have been removed.

// Note: CampaignStatus moved to campaign.rs since it's tightly coupled
// Note: ModuleStatus removed in favor of board.rs WorkflowState enums
// Note: Session-related enums removed as they're content-focused, not workflow

// This file intentionally left minimal - workflow states are defined in board.rs
// as they're part of the Three-Board System implementation