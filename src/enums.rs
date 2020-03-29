/// Defines alignment rules used by FLTK for labels
#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Align {
    AlignCenter = 0,
    AlignTop = 1,
    AlignBottom = 2,
    AlignLeft = 4,
    AlignRight = 8,
}

/// Defines fonts used by FLTK
#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq)]
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

/// Defines colors used by FLTK
#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq)]
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

/// Defines event types captured by FLTK
#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq)]
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
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Key {
    Button = 0xfee8,
    BackSpace = 0xff08,
    Tab = 0xff09,
    IsoKey = 0xff0c,
    Enter = 0xff0d,
    Pause = 0xff13,
    ScrollLock = 0xff14,
    Escape = 0xff1b,
    Kana = 0xff2e,
    Eisu = 0xff2f,
    Yen = 0xff30,
    JISUnderscore = 0xff31,
    Home = 0xff50,
    Left = 0xff51,
    Up = 0xff52,
    Right = 0xff53,
    Down = 0xff54,
    PageUp = 0xff55,
    PageDown = 0xff56,
    End = 0xff57,
    Print = 0xff61,
    Insert = 0xff63,
    Menu = 0xff67,
    Help = 0xff68,
    NumLock = 0xff7f,
    KP = 0xff80,
    KPEnter = 0xff8d,
    KPLast = 0xffbd,
    FLast = 0xffe0,
    ShiftL = 0xffe1,
    ShiftR = 0xffe2,
    ControlL = 0xffe3,
    ControlR = 0xffe4,
    CapsLock = 0xffe5,
    MetaL = 0xffe7,
    MetaR = 0xffe8,
    AltL = 0xffe9,
    AltR = 0xffea,
    Delete = 0xffff,
}

#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Shortcut {
    None = 0,
    Shift = 0x00010000,
    CapsLock = 0x00020000,
    Ctrl = 0x00040000,
    Alt = 0x00080000,
}

#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum CallbackTrigger {
    Never = 0,
    Changed = 1,
    NotChanged = 2,
    Release = 4,
    ReleaseAlways = 6,
    EnterKey = 8,
    EnterKeyAlways = 10,
    EnterKeyChanged = 11,
}

impl std::ops::Add<char> for Shortcut {
    type Output = Shortcut;
    fn add(self, other: char) -> Self::Output {
        unsafe { std::mem::transmute(self as i32 + other as i32) }
    }
}

impl std::ops::BitOr<CallbackTrigger> for CallbackTrigger {
    type Output = CallbackTrigger;
    fn bitor(self, rhs: CallbackTrigger) -> Self::Output {
        unsafe { std::mem::transmute(self as i32 | rhs as i32) }
    }
}

impl std::ops::BitOr<Align> for Align {
    type Output = Align;
    fn bitor(self, rhs: Align) -> Self::Output {
        unsafe { std::mem::transmute(self as i32 | rhs as i32) }
    }
}

impl std::ops::BitOr<Color> for Color {
    type Output = Color;
    fn bitor(self, rhs: Color) -> Self::Output {
        unsafe { std::mem::transmute(self as i32 | rhs as i32) }
    }
}

impl std::ops::BitOr<Font> for Font {
    type Output = Font;
    fn bitor(self, rhs: Font) -> Self::Output {
        unsafe { std::mem::transmute(self as i32 | rhs as i32) }
    }
}

impl std::ops::BitOr<Event> for Event {
    type Output = Event;
    fn bitor(self, rhs: Event) -> Self::Output {
        unsafe { std::mem::transmute(self as i32 | rhs as i32) }
    }
}

impl std::ops::BitOr<Key> for Key {
    type Output = Key;
    fn bitor(self, rhs: Key) -> Self::Output {
        unsafe { std::mem::transmute(self as i32 | rhs as i32) }
    }
}


