pub use crate::prelude::*;
use std::mem;

#[repr(i32)]
#[derive(Debug, Copy, Clone)]
pub enum Align {
    AlignCenter = 0,
    AlignTop = 1,
    AlignBottom = 2,
    AlignLeft = 4,
    AlignRight = 8,
}

#[repr(i32)]
#[derive(Debug, Copy, Clone)]
pub enum Font {
    Helvetica = 0,
    HelveticaBold = 1,
    HelveticaItalic = 2,
    HelveticaBoldItalic = 3,
    Courrier = 4,
    CourrierBold = 5,
    CourrierItalic = 6,
    CourrierBoldItalic = 7,
    Times = 8,
    TimesBold = 9,
    TimesItalic = 10,
    TimesBoldItalic = 11,
    Symbol = 12,
    Screen = 13,
    ScreenBold = 14,
    Zapfdingbats = 15,
    Freefont = 16,
}

#[repr(i32)]
#[derive(Debug, Copy, Clone)]
pub enum Color {
    ForeGround = 0,
    BackGround = 7,
    Inactive = 8,
    Selection = 15,
    Gray0 = 32,
    Dark3 = 39,
    Dark2 = 45,
    Dark1 = 47,
    FrameDefault = 49,
    Light1 = 50,
    Light2 = 52,
    Light3 = 54,
    Black = 56,
    Red = 88,
    Green = 63,
    Yellow = 95,
    Blue = 216,
    Magenta = 248,
    Cyan = 223,
    DarkRed = 72,
    DarkGreen = 60,
    DarkYellow = 76,
    DarkBlue = 136,
    DarkMagenta = 152,
    DarkCyan = 140,
    White = 255,
}

#[repr(i32)]
#[derive(Debug, Copy, Clone)]
pub enum Event {
    NoEvent = 0,
    Push,
    Released,
    Enter,
    Leave,
    Drag,
    Focus,
    Unfocus,
    KeyDown,
    KeyUp,
    Close,
    Move,
    Shortcut,
    Deactivate,
    Activate,
    Hide,
    Show,
    Paste,
    SelectionClear,
    MouseWheel,
}

#[repr(i32)]
#[derive(WidgetType, Debug, Copy, Clone)]
pub enum LabelType {
    NormalLabel = 0,
    NoLabel,
    ShadowLabel,
    EngravedLabel,
    EmbossedLabel,
    MultiLabel,
    IconLabel,
    ImageLabel,
    FreeLabelType,
}

#[repr(i32)]
#[derive(Debug, Copy, Clone)]
pub enum Shortcut {
    Shift = 0x00010000,
    CapsLock = 0x00020000,
    Ctrl = 0x00040000,
    Alt = 0x00080000,
}

impl std::ops::Add<char> for Shortcut {
    type Output = i32;
    fn add(self, other: char) -> i32 {
        self as i32 + other as i32
    }
}
