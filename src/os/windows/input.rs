use super::get_stdin_handle;
use crate::input::{Event, KeyEvent};

use std::os::windows::raw::HANDLE;
use std::{io, mem, time::Duration};

#[repr(C)]
#[derive(Copy, Clone)]
struct InputRecord {
    event_type: u16,
    event: EventRecord,
}

#[repr(C)]
#[derive(Copy, Clone)]
union EventRecord {
    key: KeyEventRecord,
    focus: FocusEventRecord,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct KeyEventRecord {
    key_down: i32,
    repeat_count: u16,
    virtual_key_code: u16,
    virtual_scan_code: u16,
    u_char: CharUnion,
    control_key_state: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct FocusEventRecord {
    set_focus: i32,
}

#[repr(C)]
#[derive(Copy, Clone)]
union CharUnion {
    unicode_char: u16,
    ascii_char: u8,
}

unsafe extern "system" {
    fn ReadConsoleInputW(
        console_input: HANDLE,
        buffer: *mut InputRecord,
        length: u32,
        number_of_events_read: *mut u32,
    ) -> i32;
    fn WaitForSingleObject(handle: HANDLE, wait_time_ms: u32) -> u32;
}

/// Attempts to fetch input from stdin
///
/// # Errors
/// If the timeout has expired or
/// there was an error getting the data
pub fn poll_input(timeout: Duration) -> io::Result<Event> {
    let handle = get_stdin_handle()?;
    let mut record: InputRecord = unsafe { mem::zeroed() };
    let mut read = 0;

    // shut up clippy no reasonable person would expect to be able to have a poll longer than a
    // month
    #[allow(clippy::cast_possible_truncation)]
    let wait_time_millis = timeout.as_millis() as u32;
    let result = unsafe { WaitForSingleObject(handle, wait_time_millis) };

    // The function timed out
    if result != 0 {
        return Err(io::ErrorKind::TimedOut.into());
    }

    let result = unsafe { ReadConsoleInputW(handle, &mut record, 1, &mut read) };

    if result == 0 {
        Err(io::Error::last_os_error())?;
    }
    match record.event_type {
        0x10 => {
            // Focus Event
            Err(io::ErrorKind::InvalidData.into())
        }
        0x1 => {
            // Key Event
            let key_event: KeyEventRecord = unsafe { record.event.key };
            if key_event.key_down == 0 {
                // return Ok(Event::Key(KeyEvent::Null));
                // I don't quite know why but this seems to happen a lot, until I investigate
                // more this will have to do
                return Err(io::ErrorKind::Other.into());
            }
            Ok(Event::Key(parse_key_event(&key_event)))
        }
        _ => {
            //TODO Make this better
            Err(io::ErrorKind::InvalidData.into())
        }
    }
}

fn parse_key_event(event: &KeyEventRecord) -> KeyEvent {
    let ctrl = event.control_key_state & (0x0008 | 0x0004) != 0; // LEFT_CTRL_PRESSED | RIGHT_CTRL_PRESSED
    let shift = event.control_key_state & 0x0010 != 0; // SHIFT_PRESSED

    match event.virtual_key_code {
        0x08 => KeyEvent::Backspace, // VK_BACK
        0x09 => {
            if shift {
                KeyEvent::ShiftTab
            } else {
                KeyEvent::Tab
            }
        }
        0x0D => KeyEvent::Char('\n'),
        0x1B => KeyEvent::Escape,
        0x21 => KeyEvent::PageUp,
        0x22 => KeyEvent::PageDown,
        0x23 => KeyEvent::End,
        0x24 => KeyEvent::Home,
        0x25 => KeyEvent::Left,
        0x26 => KeyEvent::Up,
        0x27 => KeyEvent::Right,
        0x28 => KeyEvent::Down,
        0x2D => KeyEvent::Insert,
        0x2E => KeyEvent::Delete,
        // I don't think anybody is going to try to press F256 clippy
        #[allow(clippy::cast_possible_truncation)]
        0x70..=0x87 => KeyEvent::F((event.virtual_key_code - 0x6F) as u8), // F1-F24
        _ => {
            let c = char::from_u32(u32::from(unsafe { event.u_char.unicode_char })).unwrap_or(' ');
            if ctrl && c.is_ascii_alphabetic() {
                KeyEvent::Ctrl(c)
            } else {
                KeyEvent::Char(c)
            }
        }
    }
}
