//! Lumenza is a cross-platform multimedia manager. It handles core tasks, such as
//! storing data in a database, tagging, and album management.
//!
//! An inspiration for this library was Windows Media Player Legacy. This crate
//! is meant to fill in the area of multimedia management. Although WMP Legacy
//! has many features that are outside the scope of this library, this library
//! replicates the core functionality of WMP Legacy: organizing multimedia into
//! albums, tagging them, etc.
//!
//! The subsystems, such as the database engine and the configuration system, 
//! are managed implicitly. Thumbnailiing can be done by the client, or by 
//! Lumenza itself. This is useful when the target platform has a specific 
//! thumbnailing library that is faster, or supports more specialized formats.
//! 
//! Another note is that Lumenza does not interact with thumbnails beyound 
//! storing the path to the thumbnail, unless requested explicitly by the client.
//! This means that you can use any type of file as a thumbnail. 

/// Detailed error representation
pub mod error;

// Reexport as struct.
mod library;
pub use crate::library::Library;

// Reeexport as struct.
mod picture;
pub use crate::picture::Picture;

// Provide access to subsystems only in this crate.
pub(crate) mod systems;
