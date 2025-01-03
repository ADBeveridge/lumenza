//! Lumenza is a cross-platform picture manager. 
//! It handles core tasks, such as storing data in a database,
//! tagging, and album management.

/// Lumenza specific errors
pub mod error;

/// Library level functions
pub mod library;

/// Picture level functions
pub mod picture;

pub(crate) mod systems;
