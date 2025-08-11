use keycode::{KeyMappingId, KeyModifiers};

pub const fn keycode_to_enigo_key(keycode: keycode::KeyMap) -> Option<enigo::Key> {
    Some(match keycode.id {
        KeyMappingId::Escape => enigo::Key::Escape,
        KeyMappingId::F1 => enigo::Key::F1,
        KeyMappingId::F2 => enigo::Key::F2,
        KeyMappingId::F3 => enigo::Key::F3,
        KeyMappingId::F4 => enigo::Key::F4,
        KeyMappingId::F5 => enigo::Key::F5,
        KeyMappingId::F6 => enigo::Key::F6,
        KeyMappingId::F7 => enigo::Key::F7,
        KeyMappingId::F8 => enigo::Key::F8,
        KeyMappingId::F9 => enigo::Key::F9,
        KeyMappingId::F10 => enigo::Key::F10,
        KeyMappingId::F11 => enigo::Key::F11,
        KeyMappingId::F12 => enigo::Key::F12,
        KeyMappingId::PrintScreen => enigo::Key::PrintScr,
        KeyMappingId::ScrollLock => enigo::Key::ScrollLock,
        KeyMappingId::Pause => enigo::Key::Pause,

        KeyMappingId::Backquote => enigo::Key::Unicode('`'),
        KeyMappingId::Tab => enigo::Key::Tab,
        KeyMappingId::CapsLock => enigo::Key::CapsLock,
        KeyMappingId::ShiftLeft => enigo::Key::LShift,
        KeyMappingId::ControlLeft => enigo::Key::LControl,
        KeyMappingId::Super => enigo::Key::Meta,
        KeyMappingId::AltLeft => enigo::Key::Alt,
        KeyMappingId::Space => enigo::Key::Space,

        KeyMappingId::Minus => enigo::Key::Unicode('-'),
        KeyMappingId::Equal => enigo::Key::Unicode('='),
        KeyMappingId::Backspace => enigo::Key::Backspace,
        KeyMappingId::BracketLeft => enigo::Key::Unicode('['),
        KeyMappingId::BracketRight => enigo::Key::Unicode(']'),
        KeyMappingId::Backslash => enigo::Key::Unicode('\\'),
        KeyMappingId::Semicolon => enigo::Key::Unicode(';'),
        KeyMappingId::Quote => enigo::Key::Unicode('\''),
        KeyMappingId::Enter => enigo::Key::Return,
        KeyMappingId::Comma => enigo::Key::Unicode(','),
        KeyMappingId::Period => enigo::Key::Unicode('.'),
        KeyMappingId::Slash => enigo::Key::Unicode('/'),
        KeyMappingId::ShiftRight => enigo::Key::RShift,
        KeyMappingId::ControlRight => enigo::Key::RControl,
        KeyMappingId::ContextMenu => enigo::Key::LMenu,
        KeyMappingId::AltRight => enigo::Key::Alt,

        KeyMappingId::Digit1 => enigo::Key::Unicode('1'),
        KeyMappingId::Digit2 => enigo::Key::Unicode('2'),
        KeyMappingId::Digit3 => enigo::Key::Unicode('3'),
        KeyMappingId::Digit4 => enigo::Key::Unicode('4'),
        KeyMappingId::Digit5 => enigo::Key::Unicode('5'),
        KeyMappingId::Digit6 => enigo::Key::Unicode('6'),
        KeyMappingId::Digit7 => enigo::Key::Unicode('7'),
        KeyMappingId::Digit8 => enigo::Key::Unicode('8'),
        KeyMappingId::Digit9 => enigo::Key::Unicode('9'),
        KeyMappingId::Digit0 => enigo::Key::Unicode('0'),

        KeyMappingId::UsA => enigo::Key::Unicode('a'),
        KeyMappingId::UsB => enigo::Key::Unicode('b'),
        KeyMappingId::UsC => enigo::Key::Unicode('c'),
        KeyMappingId::UsD => enigo::Key::Unicode('d'),
        KeyMappingId::UsE => enigo::Key::Unicode('e'),
        KeyMappingId::UsF => enigo::Key::Unicode('f'),
        KeyMappingId::UsG => enigo::Key::Unicode('g'),
        KeyMappingId::UsH => enigo::Key::Unicode('h'),
        KeyMappingId::UsI => enigo::Key::Unicode('i'),
        KeyMappingId::UsJ => enigo::Key::Unicode('j'),
        KeyMappingId::UsK => enigo::Key::Unicode('k'),
        KeyMappingId::UsL => enigo::Key::Unicode('l'),
        KeyMappingId::UsM => enigo::Key::Unicode('m'),
        KeyMappingId::UsN => enigo::Key::Unicode('n'),
        KeyMappingId::UsO => enigo::Key::Unicode('o'),
        KeyMappingId::UsP => enigo::Key::Unicode('p'),
        KeyMappingId::UsQ => enigo::Key::Unicode('q'),
        KeyMappingId::UsR => enigo::Key::Unicode('r'),
        KeyMappingId::UsS => enigo::Key::Unicode('s'),
        KeyMappingId::UsT => enigo::Key::Unicode('t'),
        KeyMappingId::UsU => enigo::Key::Unicode('u'),
        KeyMappingId::UsV => enigo::Key::Unicode('v'),
        KeyMappingId::UsW => enigo::Key::Unicode('w'),
        KeyMappingId::UsX => enigo::Key::Unicode('x'),
        KeyMappingId::UsY => enigo::Key::Unicode('y'),
        KeyMappingId::UsZ => enigo::Key::Unicode('z'),

        KeyMappingId::Insert => enigo::Key::Insert,
        KeyMappingId::Del => enigo::Key::Delete,
        KeyMappingId::Home => enigo::Key::Home,
        KeyMappingId::End => enigo::Key::End,
        KeyMappingId::PageUp => enigo::Key::PageUp,
        KeyMappingId::PageDown => enigo::Key::PageDown,

        KeyMappingId::ArrowDown => enigo::Key::DownArrow,
        KeyMappingId::ArrowLeft => enigo::Key::LeftArrow,
        KeyMappingId::ArrowRight => enigo::Key::RightArrow,
        KeyMappingId::ArrowUp => enigo::Key::UpArrow,

        KeyMappingId::NumLock => enigo::Key::Numlock,

        KeyMappingId::Numpad0 => enigo::Key::Numpad0,
        KeyMappingId::Numpad1 => enigo::Key::Numpad1,
        KeyMappingId::Numpad2 => enigo::Key::Numpad2,
        KeyMappingId::Numpad3 => enigo::Key::Numpad3,
        KeyMappingId::Numpad4 => enigo::Key::Numpad4,
        KeyMappingId::Numpad5 => enigo::Key::Numpad5,
        KeyMappingId::Numpad6 => enigo::Key::Numpad6,
        KeyMappingId::Numpad7 => enigo::Key::Numpad7,
        KeyMappingId::Numpad8 => enigo::Key::Numpad8,
        KeyMappingId::Numpad9 => enigo::Key::Numpad9,

        KeyMappingId::NumpadDivide => enigo::Key::Divide,
        KeyMappingId::NumpadMultiply => enigo::Key::Multiply,
        KeyMappingId::NumpadSubtract => enigo::Key::Subtract,
        KeyMappingId::NumpadAdd => enigo::Key::Add,
        KeyMappingId::NumpadDecimal => enigo::Key::Decimal,

        _ => return None,
    })
}

pub fn key_modifier_to_enigo_key(key_modifier: KeyModifiers) -> Option<enigo::Key> {
    if key_modifier.is_empty() { None }
    else if key_modifier.contains(KeyModifiers::ControlLeft)  { Some(enigo::Key::LControl) }
    else if key_modifier.contains(KeyModifiers::ControlRight) { Some(enigo::Key::RControl) }
    else if key_modifier.contains(KeyModifiers::ShiftLeft)  { Some(enigo::Key::LShift) }
    else if key_modifier.contains(KeyModifiers::ShiftRight) { Some(enigo::Key::RShift) }
    else if key_modifier.intersects(KeyModifiers::AltLeft | KeyModifiers::AltRight) { Some(enigo::Key::Alt) }
    else if key_modifier.intersects(KeyModifiers::MetaLeft | KeyModifiers::MetaRight) { Some(enigo::Key::Meta) }
    else { unreachable!("exhaustive") }
}
