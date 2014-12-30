extern crate tcod;

use tcod::{Console};

pub enum KeyCode {
    // Arrow keys
    Up,
    Down,
    Left,
    Right,

    Shift,
    ShiftUp,
    ShiftDown,
    ShiftLeft,
    ShiftRight,

    // Special
    Escape,

}

pub fn check_for_keypress() -> Option<KeyCode> {
    let flags = tcod::KEY_PRESSED;
    match Console::check_for_keypress(flags) {
        Some(key_state) => {
            if key_state.shift {
                match key_state.key {
                    self::tcod::Key::Special(tcod::KeyCode::Up)     => Some(KeyCode::ShiftUp),
                    self::tcod::Key::Special(tcod::KeyCode::Down)   => Some(KeyCode::ShiftDown),
                    self::tcod::Key::Special(tcod::KeyCode::Left)   => Some(KeyCode::ShiftLeft),
                    self::tcod::Key::Special(tcod::KeyCode::Right)  => Some(KeyCode::ShiftRight),
                    _                                               => Some(KeyCode::Shift)
                }
            } else {
                match key_state.key {
                    self::tcod::Key::Special(tcod::KeyCode::Up)     => Some(KeyCode::Up),
                    self::tcod::Key::Special(tcod::KeyCode::Down)   => Some(KeyCode::Down),
                    self::tcod::Key::Special(tcod::KeyCode::Left)   => Some(KeyCode::Left),
                    self::tcod::Key::Special(tcod::KeyCode::Right)  => Some(KeyCode::Right),
                    self::tcod::Key::Special(tcod::KeyCode::Escape) => Some(KeyCode::Escape),
                    _                                               => None
                } 
            }
        },
        None => {
             None
        }
    }

}

