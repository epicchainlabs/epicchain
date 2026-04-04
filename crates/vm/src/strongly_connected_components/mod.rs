//! Strongly connected components module for the Neo Virtual Machine.
//!
//! This module provides algorithms for finding strongly connected components in a graph.

pub mod tarjan;

pub use tarjan::Tarjan;
