//! Binary serialization and data processing for Composer
//!
//! This crate provides binary formats for musical chord and note data serialization,
//! including the 5-byte chord format, trie serialization, tokenization for ML,
//! and hash functions for data integrity.

pub mod chord_binary;
pub mod constants;
pub mod error;
pub mod hash;
pub mod tokenization;
pub mod trie_binary;

pub use chord_binary::*;
pub use constants::*;
pub use error::*;
pub use hash::*;
pub use tokenization::*;
pub use trie_binary::*;
