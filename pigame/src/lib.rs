//! # pigame
//! low dependency pi zero game engine (linux framebuffer)

#![warn(
    clippy::unwrap_used,
    clippy::pedantic,
    clippy::nursery,
    missing_docs,
    missing_debug_implementations,
    rust_2018_idioms,
    unreachable_pub,
    unused_qualifications
)]

/// Context and global state
#[doc(hidden)]
pub mod context;
/// Error types
pub mod error;
/// Framebuffer abstractions, drawing functions, text, and colours
pub mod graphics;
/// Input handling
pub mod input;
/// Mathematical types
pub mod maths;
pub use rand;
