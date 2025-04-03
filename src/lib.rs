#![warn(clippy::all, clippy::pedantic)]

use std::io;

pub mod ansi;
pub mod input;
mod os;

generate_os_function!(pub fn enable_ansi() -> io::Result<()>);
generate_os_function!(pub fn get_terminal_size() -> io::Result<(u16, u16)>);

generate_os_struct!(pub struct RawTerminal);
