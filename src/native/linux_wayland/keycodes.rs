//! There are quite a few different notions for keycodes. And most of them are `u32` so it does get
//! very confusing...
//! Basically
//!   - Wayland server sends a scancode `key` of type `c_uint`
//!   - `key + 8` becomes a `xkb` scancode of type `xkb_keycode_t`
//!   - We feed this to `xkb` to get a `keysym` of type `xkb_keysym_t`
//!     - The `keysym` can be modifier-dependent: `Shift + Key1` can be translated to either `Key1`
//!       (without modifier) or `Exclam` (with modifier)
//!   - We then feed the `keysym` to `translate_keysym` to get a Miniquad `Keycode`
//!
//! Note that the default Miniquad behavior is without modifier; there is not even a Keycode for
//! `Exclam`. So we must provide the unmodified `keysym` or we will get a `Keycode::Unknown`.
//!
//! On the other hand, the modified `keysym` is useful when we want to translate it into the
//! underlying character.
use crate::event::KeyCode;
use crate::native::linux_wayland::libxkbcommon::xkb_keysym_t;

pub fn translate_keysym(keysym: xkb_keysym_t) -> KeyCode {
    // See xkbcommon/xkbcommon-keysyms.h
    match keysym {
        65307 => KeyCode::Escape,
        65056 => KeyCode::Tab, // LeftTab
        65289 => KeyCode::Tab,
        65505 => KeyCode::LeftShift,
        65506 => KeyCode::RightShift,
        65507 => KeyCode::LeftControl,
        65508 => KeyCode::RightControl,
        65511 | 65513 => KeyCode::LeftAlt,
        65406 | 65027 | 65512 | 65514 => KeyCode::RightAlt,
        65515 => KeyCode::LeftSuper,
        65516 => KeyCode::RightSuper,
        65383 => KeyCode::Menu,
        65407 => KeyCode::NumLock,
        65509 => KeyCode::CapsLock,
        65377 => KeyCode::PrintScreen,
        65300 => KeyCode::ScrollLock,
        65299 => KeyCode::Pause,
        65535 => KeyCode::Delete,
        65288 => KeyCode::Backspace,
        65293 => KeyCode::Enter,
        65360 => KeyCode::Home,
        65367 => KeyCode::End,
        65365 => KeyCode::PageUp,
        65366 => KeyCode::PageDown,
        65379 => KeyCode::Insert,
        65361 => KeyCode::Left,
        65363 => KeyCode::Right,
        65364 => KeyCode::Down,
        65362 => KeyCode::Up,
        65470 => KeyCode::F1,
        65471 => KeyCode::F2,
        65472 => KeyCode::F3,
        65473 => KeyCode::F4,
        65474 => KeyCode::F5,
        65475 => KeyCode::F6,
        65476 => KeyCode::F7,
        65477 => KeyCode::F8,
        65478 => KeyCode::F9,
        65479 => KeyCode::F10,
        65480 => KeyCode::F11,
        65481 => KeyCode::F12,
        65482 => KeyCode::F13,
        65483 => KeyCode::F14,
        65484 => KeyCode::F15,
        65485 => KeyCode::F16,
        65486 => KeyCode::F17,
        65487 => KeyCode::F18,
        65488 => KeyCode::F19,
        65489 => KeyCode::F20,
        65490 => KeyCode::F21,
        65491 => KeyCode::F22,
        65492 => KeyCode::F23,
        65493 => KeyCode::F24,
        65494 => KeyCode::F25,
        65455 => KeyCode::KpDivide,
        65450 => KeyCode::KpMultiply,
        65453 => KeyCode::KpSubtract,
        65451 => KeyCode::KpAdd,
        65438 => KeyCode::Kp0,
        65436 => KeyCode::Kp1,
        65433 => KeyCode::Kp2,
        65435 => KeyCode::Kp3,
        65430 => KeyCode::Kp4,
        65437 => KeyCode::Kp5,
        65432 => KeyCode::Kp6,
        65429 => KeyCode::Kp7,
        65431 => KeyCode::Kp8,
        65434 => KeyCode::Kp9,
        65439 => KeyCode::KpDecimal,
        65469 => KeyCode::KpEqual,
        65421 => KeyCode::KpEnter,
        65 | 97 => KeyCode::A,
        66 | 98 => KeyCode::B,
        67 | 99 => KeyCode::C,
        68 | 100 => KeyCode::D,
        69 | 101 => KeyCode::E,
        70 | 102 => KeyCode::F,
        71 | 103 => KeyCode::G,
        72 | 104 => KeyCode::H,
        73 | 105 => KeyCode::I,
        74 | 106 => KeyCode::J,
        75 | 107 => KeyCode::K,
        76 | 108 => KeyCode::L,
        77 | 109 => KeyCode::M,
        78 | 110 => KeyCode::N,
        79 | 111 => KeyCode::O,
        80 | 112 => KeyCode::P,
        81 | 113 => KeyCode::Q,
        82 | 114 => KeyCode::R,
        83 | 115 => KeyCode::S,
        84 | 116 => KeyCode::T,
        85 | 117 => KeyCode::U,
        86 | 118 => KeyCode::V,
        87 | 119 => KeyCode::W,
        88 | 120 => KeyCode::X,
        89 | 121 => KeyCode::Y,
        90 | 122 => KeyCode::Z,
        49 => KeyCode::Key1,
        50 => KeyCode::Key2,
        51 => KeyCode::Key3,
        52 => KeyCode::Key4,
        53 => KeyCode::Key5,
        54 => KeyCode::Key6,
        55 => KeyCode::Key7,
        56 => KeyCode::Key8,
        57 => KeyCode::Key9,
        48 => KeyCode::Key0,
        32 => KeyCode::Space,
        45 => KeyCode::Minus,
        61 => KeyCode::Equal,
        91 => KeyCode::LeftBracket,
        93 => KeyCode::RightBracket,
        92 => KeyCode::Backslash,
        59 => KeyCode::Semicolon,
        39 => KeyCode::Apostrophe,
        96 => KeyCode::GraveAccent,
        44 => KeyCode::Comma,
        46 => KeyCode::Period,
        47 => KeyCode::Slash,
        60 => KeyCode::World1,
        _ => KeyCode::Unknown,
    }
}
