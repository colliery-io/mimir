//! System and utility command handlers.
//!
//! Contains commands for application info, logging, development tools,
//! and window management.

pub mod app_info;
pub mod dev_tools;
pub mod logs;
pub mod window_manager;

pub use app_info::*;
pub use dev_tools::*;
pub use logs::*;
pub use window_manager::*;
