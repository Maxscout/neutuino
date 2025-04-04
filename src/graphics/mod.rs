pub const ANSI_GRAPHICS_PROTOCOL_ESCAPE_START: &str = "\x1b_G";
pub const ANSI_GRAPHICS_PROTOCOL_ESCAPE_END: &str = "\x1b\\";

mod base64;

#[cfg(feature = "graphics-protocol-image")]
pub mod image;
