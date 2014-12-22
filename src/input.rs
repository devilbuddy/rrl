extern crate tcod;

use tcod::{Console, KeyState};

pub enum KeyCode {
    // Arrow keys
    Up,
    Down,
    Left,
    Right,

    // Special
    Escape,

    // Default
    None
}

pub fn wait_for_keypress()-> KeyCode {
    let key_state = Console::wait_for_keypress(true);

    match key_state.key {
        self::tcod::Key::Special(tcod::KeyCode::Up)     => KeyCode::Up,
        self::tcod::Key::Special(tcod::KeyCode::Down)   => KeyCode::Down,
        self::tcod::Key::Special(tcod::KeyCode::Left)   => KeyCode::Left,
        self::tcod::Key::Special(tcod::KeyCode::Right)  => KeyCode::Right,
        self::tcod::Key::Special(tcod::KeyCode::Escape) => KeyCode::Escape,
        _                                               => KeyCode::None
    }
}
