use std::io;
use std::os::windows::raw::HANDLE;

#[link(name = "kernel32")]
unsafe extern "system" {
    fn GetStdHandle(std_handle: i32) -> HANDLE;
    fn GetConsoleMode(console_handle: HANDLE, mode: *mut u32) -> u32;
    fn SetConsoleMode(console_handle: HANDLE, mode: u32) -> u32;
    fn GetConsoleScreenBufferInfo(
        console_output: HANDLE,
        console_screen_buffer_info: *mut ConsoleScreenBufferInfo,
    ) -> u32;
}

const STD_INPUT_HANDLE: i32 = -10;
const STD_OUTPUT_HANDLE: i32 = -11;
const ENABLE_VIRTUAL_TERMINAL_PROCESSING: u32 = 4;
const ENABLE_ECHO_INPUT: u32 = 4;
const ENABLE_LINE_INPUT: u32 = 2;
const ENABLE_PROCESSED_INPUT: u32 = 1;
const INVALID_HANDLE_VALUE: HANDLE = -1isize as HANDLE;

#[repr(C)]
#[derive(Default)]
struct ConsoleScreenBufferInfo {
    x: u16,
    y: u16,
    _unused: [u16; 9],
}

fn get_stdin_handle() -> io::Result<HANDLE> {
    let handle = unsafe { GetStdHandle(STD_INPUT_HANDLE) };
    if handle == INVALID_HANDLE_VALUE {
        Err(io::Error::last_os_error())
    } else {
        Ok(handle)
    }
}

fn get_stdout_handle() -> io::Result<HANDLE> {
    let handle = unsafe { GetStdHandle(STD_OUTPUT_HANDLE) };
    if handle == INVALID_HANDLE_VALUE {
        Err(io::Error::last_os_error())
    } else {
        Ok(handle)
    }
}

fn set_console_mode(handle: HANDLE, mode: u32) -> io::Result<()> {
    if unsafe { SetConsoleMode(handle, mode) == 0 } {
        Err(io::Error::last_os_error())
    } else {
        Ok(())
    }
}

fn get_console_mode(handle: HANDLE, mode: &mut u32) -> io::Result<()> {
    if unsafe { GetConsoleMode(handle, mode) == 0 } {
        Err(io::Error::last_os_error())
    } else {
        Ok(())
    }
}

/// Enables raw mode, which disables line buffering, input echoing, and output canonicalization
///
/// # Errors
///
/// If there is no stdin,
/// stdin is not a tty,
/// or it fails to change terminal settings
pub fn enable_raw_mode() -> io::Result<()> {
    let handle = get_stdin_handle()?;
    let mut mode = 0;
    get_console_mode(handle, &mut mode)?;
    mode &= !(ENABLE_ECHO_INPUT | ENABLE_LINE_INPUT | ENABLE_PROCESSED_INPUT);
    set_console_mode(handle, mode)?;
    Ok(())
}

/// Disables raw mode, which enables line buffering, input echoing, and output canonicalization
///
/// # Errors
///
/// If there is no stdin,
/// stdin is not a tty,
/// or it fails to change terminal settings
pub fn disable_raw_mode() -> io::Result<()> {
    let handle = get_stdin_handle()?;
    let mut mode = 0;
    get_console_mode(handle, &mut mode)?;
    mode |= ENABLE_ECHO_INPUT | ENABLE_LINE_INPUT | ENABLE_PROCESSED_INPUT;
    set_console_mode(handle, mode)?;
    Ok(())
}

/// Enables ANSI support on Windows terminals
///
/// ANSI is on by default on *nix machines but still exists on them for simpler usage
///
/// # Errors
///
/// Never on *nix
///
/// On Windows, if There is no stdout,
/// if stdout isn't a TTY, or
/// if it cannot change terminal properties
pub fn enable_ansi() -> io::Result<()> {
    let handle = get_stdout_handle()?;
    let mut mode = 0;
    get_console_mode(handle, &mut mode)?;
    mode |= ENABLE_VIRTUAL_TERMINAL_PROCESSING;
    set_console_mode(handle, mode)?;
    Ok(())
}

/// Gets the size of the terminal
///
/// Returns in (width, height) format
///
/// # Errors
///
/// If there is no stdout,
/// if stdout isn't a TTY, or
/// if it fails to retrieve the terminal size
pub fn get_terminal_size() -> io::Result<(u16, u16)> {
    let handle = get_stdout_handle()?;
    let mut csbi = ConsoleScreenBufferInfo::default();
    if unsafe { GetConsoleScreenBufferInfo(handle, &mut csbi) != 0 } {
        let width = csbi.x;
        let height = csbi.y;
        return Ok((width, height));
    }
    Err(io::Error::last_os_error())
}

pub mod input;
