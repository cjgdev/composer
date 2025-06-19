//! AI-Powered Musical Intelligence for Composer
//!
//! This crate provides machine learning-driven musical analysis, chord progression
//! suggestions, bass line harmonization, and difficulty assessment using statistical
//! models and trie-based pattern matching.

pub mod analysis;
pub mod engine;
pub mod error;
pub mod suggestions;
pub mod trie;

pub use analysis::*;
pub use engine::*;
pub use error::*;
pub use suggestions::*;
pub use trie::*;
