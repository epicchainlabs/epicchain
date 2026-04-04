//! Neo Extensions Framework
//!
//! This crate provides extension points and utilities for the Neo blockchain implementation.

pub mod collections;
pub mod encoding;
pub mod error;
pub mod plugin;
pub mod utilities;

// Re-export commonly used types
pub use error::{ExtensionError, ExtensionResult};
pub use plugin::{Plugin, PluginCategory, PluginContext, PluginEvent, PluginInfo};
