use serde::{Deserialize, Serialize};

use crate::input::AsModifier;
use crate::Modifier;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum KeyboardInput {
    Pressed(Key),
    Released(Key),
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum Key {
    AltLeft,
    AltRight,
    Backspace,
    CapsLock,
    ControlLeft,
    ControlRight,
    Delete,
    ArrowDown,
    End,
    Escape,
    F1,
    F10,
    F11,
    F12,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    Home,
    ArrowLeft,
    MetaLeft,
    MetaRight,
    PageDown,
    PageUp,
    Return,
    ArrowRight,
    ShiftLeft,
    ShiftRight,
    Space,
    Tab,
    ArrowUp,
    PrintScreen,
    ScrollLock,
    Pause,
    NumLock,
    BackQuote,
    Num1,
    Num2,
    Num3,
    Num4,
    Num5,
    Num6,
    Num7,
    Num8,
    Num9,
    Num0,
    Minus,
    Equal,
    Q,
    W,
    E,
    R,
    T,
    Y,
    U,
    I,
    O,
    P,
    BracketLeft,
    BracketRight,
    A,
    S,
    D,
    F,
    G,
    H,
    J,
    K,
    L,
    SemiColon,
    Quote,
    BackSlash,
    IntlBackslash,
    Z,
    X,
    C,
    V,
    B,
    N,
    M,
    Comma,
    Dot,
    Slash,
    Insert,
    KpReturn,
    KpMinus,
    KpPlus,
    KpMultiply,
    KpDivide,
    Kp0,
    Kp1,
    Kp2,
    Kp3,
    Kp4,
    Kp5,
    Kp6,
    Kp7,
    Kp8,
    Kp9,
    KpDelete,
    Function,
    Unknown(u32),
}

impl AsModifier for Key {
    fn as_modifier(&self) -> Modifier {
        Modifier::Key(*self)
    }
}

impl From<&rdev::Key> for Key {
    fn from(value: &rdev::Key) -> Self {
        match value {
            rdev::Key::Alt => Key::AltLeft,
            rdev::Key::AltGr => Key::AltRight,
            rdev::Key::Backspace => Key::Backspace,
            rdev::Key::CapsLock => Key::CapsLock,
            rdev::Key::ControlLeft => Key::ControlLeft,
            rdev::Key::ControlRight => Key::ControlRight,
            rdev::Key::Delete => Key::Delete,
            rdev::Key::DownArrow => Key::ArrowDown,
            rdev::Key::End => Key::End,
            rdev::Key::Escape => Key::Escape,
            rdev::Key::F1 => Key::F1,
            rdev::Key::F10 => Key::F10,
            rdev::Key::F11 => Key::F11,
            rdev::Key::F12 => Key::F12,
            rdev::Key::F2 => Key::F2,
            rdev::Key::F3 => Key::F3,
            rdev::Key::F4 => Key::F4,
            rdev::Key::F5 => Key::F5,
            rdev::Key::F6 => Key::F6,
            rdev::Key::F7 => Key::F7,
            rdev::Key::F8 => Key::F8,
            rdev::Key::F9 => Key::F9,
            rdev::Key::Home => Key::Home,
            rdev::Key::LeftArrow => Key::ArrowLeft,
            rdev::Key::MetaLeft => Key::MetaLeft,
            rdev::Key::MetaRight => Key::MetaRight,
            rdev::Key::PageDown => Key::PageDown,
            rdev::Key::PageUp => Key::PageUp,
            rdev::Key::Return => Key::Return,
            rdev::Key::RightArrow => Key::ArrowRight,
            rdev::Key::ShiftLeft => Key::ShiftLeft,
            rdev::Key::ShiftRight => Key::ShiftRight,
            rdev::Key::Space => Key::Space,
            rdev::Key::Tab => Key::Tab,
            rdev::Key::UpArrow => Key::ArrowUp,
            rdev::Key::PrintScreen => Key::PrintScreen,
            rdev::Key::ScrollLock => Key::ScrollLock,
            rdev::Key::Pause => Key::Pause,
            rdev::Key::NumLock => Key::NumLock,
            rdev::Key::BackQuote => Key::BackQuote,
            rdev::Key::Num1 => Key::Num1,
            rdev::Key::Num2 => Key::Num2,
            rdev::Key::Num3 => Key::Num3,
            rdev::Key::Num4 => Key::Num4,
            rdev::Key::Num5 => Key::Num5,
            rdev::Key::Num6 => Key::Num6,
            rdev::Key::Num7 => Key::Num7,
            rdev::Key::Num8 => Key::Num8,
            rdev::Key::Num9 => Key::Num9,
            rdev::Key::Num0 => Key::Num0,
            rdev::Key::Minus => Key::Minus,
            rdev::Key::Equal => Key::Equal,
            rdev::Key::KeyQ => Key::Q,
            rdev::Key::KeyW => Key::W,
            rdev::Key::KeyE => Key::E,
            rdev::Key::KeyR => Key::R,
            rdev::Key::KeyT => Key::T,
            rdev::Key::KeyY => Key::Y,
            rdev::Key::KeyU => Key::U,
            rdev::Key::KeyI => Key::I,
            rdev::Key::KeyO => Key::O,
            rdev::Key::KeyP => Key::P,
            rdev::Key::LeftBracket => Key::BracketLeft,
            rdev::Key::RightBracket => Key::BracketRight,
            rdev::Key::KeyA => Key::A,
            rdev::Key::KeyS => Key::S,
            rdev::Key::KeyD => Key::D,
            rdev::Key::KeyF => Key::F,
            rdev::Key::KeyG => Key::G,
            rdev::Key::KeyH => Key::H,
            rdev::Key::KeyJ => Key::J,
            rdev::Key::KeyK => Key::K,
            rdev::Key::KeyL => Key::L,
            rdev::Key::SemiColon => Key::SemiColon,
            rdev::Key::Quote => Key::Quote,
            rdev::Key::BackSlash => Key::BackSlash,
            rdev::Key::IntlBackslash => Key::IntlBackslash,
            rdev::Key::KeyZ => Key::Z,
            rdev::Key::KeyX => Key::X,
            rdev::Key::KeyC => Key::C,
            rdev::Key::KeyV => Key::V,
            rdev::Key::KeyB => Key::B,
            rdev::Key::KeyN => Key::N,
            rdev::Key::KeyM => Key::M,
            rdev::Key::Comma => Key::Comma,
            rdev::Key::Dot => Key::Dot,
            rdev::Key::Slash => Key::Slash,
            rdev::Key::Insert => Key::Insert,
            rdev::Key::KpReturn => Key::KpReturn,
            rdev::Key::KpMinus => Key::KpMinus,
            rdev::Key::KpPlus => Key::KpPlus,
            rdev::Key::KpMultiply => Key::KpMultiply,
            rdev::Key::KpDivide => Key::KpDivide,
            rdev::Key::Kp0 => Key::Kp0,
            rdev::Key::Kp1 => Key::Kp1,
            rdev::Key::Kp2 => Key::Kp2,
            rdev::Key::Kp3 => Key::Kp3,
            rdev::Key::Kp4 => Key::Kp4,
            rdev::Key::Kp5 => Key::Kp5,
            rdev::Key::Kp6 => Key::Kp6,
            rdev::Key::Kp7 => Key::Kp7,
            rdev::Key::Kp8 => Key::Kp8,
            rdev::Key::Kp9 => Key::Kp9,
            rdev::Key::KpDelete => Key::KpDelete,
            rdev::Key::Function => Key::Function,
            rdev::Key::Unknown(key) => Key::Unknown(*key),
        }
    }
}