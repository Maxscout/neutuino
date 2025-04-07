#![warn(clippy::all, clippy::pedantic)]

pub mod ansi;
pub mod input;
pub(crate) mod os;

pub use os::*;