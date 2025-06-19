//! Core music theory data structures and algorithms for Composer
//!
//! This crate provides the fundamental data structures and algorithms for chord theory,
//! Roman numeral notation, and musical transformation as defined in the specification.

pub mod chord;
pub mod constants;
pub mod error;
pub mod roman;
pub mod scale;
pub mod theory;

pub use chord::*;
pub use constants::*;
pub use error::*;
pub use roman::*;
pub use scale::*;
pub use theory::*;
