//! Various input functions, structs, etc.
//!
//! Very incomplete currently

#[cfg(unix)]
#[path = "unix_input.rs"]
mod unix_input;

#[cfg(unix)]
pub use unix_input::*;

#[cfg(windows)]
#[path = "windows_input.rs"]
mod windows_input;

#[cfg(windows)]
pub use windows_input::*;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Event {
    Key(KeyEvent),
    Mouse(MouseEvent),
    FocusGained,
    FocusLost,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum KeyEvent {
    Backspace,
    Up,
    Down,
    Left,
    Right,
    Home,
    End,
    PageUp,
    PageDown,
    Tab,
    BackTab,
    Delete,
    Insert,
    F(u8),
    Char(char),
    Ctrl(char),
    Escape,
    Null,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum MouseEvent {
    Press(MouseButton, u16, u16),
    Release(u16, u16),
    Hold(u16, u16),
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
    WheelUp,
    WheelDown,
    WheelLeft,
    WheelRight,
}
