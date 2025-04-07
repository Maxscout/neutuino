//! Collection of functions that help control the terminal
//!
//! These are built to work at least on these platforms:
//! Windows, Linux, and Mac, but are likely to work on more

#[allow(unused)]
pub(crate) mod unix;

#[allow(unused)]
pub(crate) mod windows;

#[cfg(unix)]
pub use unix::*;

#[cfg(windows)]
pub use windows::*;