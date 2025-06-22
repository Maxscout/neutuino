//! Various input functions, structs, etc.
//!
//! Very incomplete currently

use std::{fmt::Debug, ops::BitOr};

pub use crate::os::input::*;

/// Different events that can happen through the terminal
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Event {
    /// An event that happens upon a key being pressed
    KeyEvent {
        key: Key,
        modifiers: KeyModifiers,
        state: KeyState,
    },
    /// An event that happens upon focus to the terminal window being gained
    FocusGained,
    /// An event that happens upon focus to the terminal window being lost
    FocusLost,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct KeyModifiers {
    flags: u8,
}

impl KeyModifiers {
    pub const NONE: Self = Self { flags: 0b00000000 };
    /// The Control modifier key
    pub const CTRL: Self = Self { flags: 0b00000001 };
    /// The Alt modifier key
    pub const ALT: Self = Self { flags: 0b00000010 };
    /// The Shift modifier key
    pub const SHIFT: Self = Self { flags: 0b00000100 };
    /// The Meta modifier key
    pub const META: Self = Self { flags: 0b00001000 };
    /// The Super modifier key
    pub const SUPER: Self = Self { flags: 0b00010000 };
    /// The Hyper modifier key
    pub const HYPER: Self = Self { flags: 0b00100000 };
    /// The Caps Lock modifier key
    pub const CAPS_LOCK: Self = Self { flags: 0b01000000 };
    /// The Num Lock modifier key
    pub const NUM_LOCK: Self = Self { flags: 0b10000000 };

    pub fn is_modifier_pressed(&self, other: Self) -> bool {
        (other.flags & self.flags) == other.flags
    }
}

// impl Debug for KeyModifiers {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         let modifiers = vec![
//             "NONE",
//             "CTRL",
//             "ALT",
//             "SHIFT",
//             "META",
//             "SUPER",
//             "HYPER",
//             "CAPS_LOCK",
//             "NUM_LOCK",
//         ];

//         Ok(())
//     }
// }

impl BitOr for KeyModifiers {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self {
            flags: self.flags | rhs.flags,
        }
    }
}

/// Determines which side the key is on the keyboard.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum KeySide {
    Left,
    Right,
}

/// Determines if the key is pressed down or released.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum KeyState {
    Pressed,
    Released,
    Repeated,
}

/// An event that happens upon a key being pressed
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Key {
    /// The Backspace key
    Backspace,
    /// The Up arrow key
    Up,
    /// The Down arrow key
    Down,
    /// The Left arrow key
    Left,
    /// The Right arrow key
    Right,
    /// The Home key
    Home,
    /// The End key
    End,
    /// The PageUp key
    PageUp,
    /// The PageDown key
    PageDown,
    /// The Tab key
    Tab,
    /// The delete key
    Delete,
    /// The insert key
    Insert,
    /// The f1-f12 keys
    F(u8),
    /// Any character inputted by the keyboard
    Char(char),
    /// The Escape key
    Escape,
    /// Control key
    Control(KeySide),
    /// Alt key
    Alt(KeySide),
    /// Shift key
    Shift(KeySide),
    /// A null byte sent to the terminal
    ///
    /// Can mean several different things
    Null,
}
