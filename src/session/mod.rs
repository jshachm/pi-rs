//! Session management - JSONL tree structure
//!
//! Sessions are stored as JSONL files with a tree structure.
//! Each entry has an id and parentId, enabling in-place branching.

pub mod manager;
pub mod entry;

pub use manager::SessionManager;
pub use entry::*;
