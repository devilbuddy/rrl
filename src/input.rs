extern crate tcod;

use tcod::{Console};

pub enum KeyCode {
    // Arrow keys
    Up,
    Down,
    Left,
    Right,

    ToggleAim,
    

    // Special
    Escape,

}

pub fn check_for_keypress() -> Option<KeyCode> {
    let flags = tcod::KEY_PRESSED;
    match Console::check_for_keypress(flags) {
        Some(key_state) => {
            if key_state.shift || key_state.left_ctrl {
                Some(KeyCode::ToggleAim)
                
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

