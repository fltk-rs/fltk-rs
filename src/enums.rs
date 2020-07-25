use crate::app::*;
use fltk_sys::fl::Fl_get_rgb_color;

/// Defines label types
#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum LabelType {
    Normal = 0,
    None,
    Shadow,
    Engraved,
    Embossed,
    Multi,
    Icon,
    Image,
    FreeType,
}

/// Defines the frame type, which can be set using the set_type() method
#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum FrameType {
    NoBox = 0,
    FlatBox,
    UpBox,
    DownBox,
    UpFrame,
    DownFrame,
    ThinUpBox,
    ThinDownBox,
    ThinUpFrame,
    ThinDownFrame,
    EngraveBox,
    EmbossedBox,
    EngravedFrame,
    EmbossedFrame,
    BorderBox,
    ShadowBox,
    BorderFrame,
    ShadowFrame,
    RoundedBox,
    RShadowBox,
    RoundedFrame,
    RFlatBox,
    RoundUpBox,
    RoundDownBox,
    DiamondUpBox,
    DiamondDownBox,
    OvalBox,
    OShadowBox,
    OvalFrame,
    OFlatFrame,
    PlasticUpBox,
    PlasticDownBox,
    PlasticUpFrame,
    PlasticDownFrame,
    PlasticThinUpBox,
    PlasticThinDownBox,
    PlasticRoundUpBox,
    PlasticRoundDownBox,
    GtkUpBox,
    GtkDownBox,
    GtkUpFrame,
    GtkDownFrame,
    GtkThinUpBox,
    GtkThinDownBox,
    GtkThinUpFrame,
    GtkThinDownFrame,
    GtkRoundUpFrame,
    GtkRoundDownFrame,
    GleamUpBox,
    GleamDownBox,
    GleamUpFrame,
    GleamDownFrame,
    GleamThinUpBox,
    GleamThinDownBox,
    GleamRoundUpBox,
    GleamRoundDownBox,
    FreeBoxType,
}

impl FrameType {
    /// Gets the Frame type by index
    pub fn by_index(idx: usize) -> FrameType {
        assert!(idx < 57);
        unsafe { std::mem::transmute(idx as i32) }
    }
}

/// Defines alignment rules used by FLTK for labels
#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Align {
    Center = 0,
    Top = 1,
    Bottom = 2,
    Left = 4,
    Right = 8,
}

/// Defines fonts used by FLTK
#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Font {
    Helvetica = 0,
    HelveticaBold = 1,
    HelveticaItalic = 2,
    HelveticaBoldItalic = 3,
    Courier = 4,
    CourierBold = 5,
    CourierItalic = 6,
    CourierBoldItalic = 7,
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

impl Font {
    /// Returns a font by index, can be queried via the app::get_font_names()
    pub fn by_index(idx: usize) -> Font {
        unsafe {
            if let Some(f) = &FONTS {
                if idx < f.len() {
                    std::mem::transmute(idx as i32)
                } else {
                    Font::Helvetica
                }
            } else {
                Font::Helvetica
            }
        }
    }

    /// Gets the font by its name, can be queried via the app::get_font_names()
    pub fn by_name(name: &str) -> Font {
        match font_index(name) {
            Some(val) => Font::by_index(val),
            None => Font::Helvetica,
        }
    }
}

/// Defines colors used by FLTK
#[repr(u32)]
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

impl Color {
    /// Returns a color from RGB
    pub fn from_rgb(r: u8, g: u8, b: u8) -> Color {
        unsafe { std::mem::transmute(Fl_get_rgb_color(r, g, b)) }
    }

    /// Returns a color from hex or decimal
    pub fn from_u32(val: u32) -> Color {
        let hex = format!("{:06x}", val);
        let r = u8::from_str_radix(&hex[0..2], 16).unwrap();
        let g = u8::from_str_radix(&hex[2..4], 16).unwrap();
        let b = u8::from_str_radix(&hex[4..6], 16).unwrap();
        Color::from_rgb(r, g, b)
    }

    /// Returns the hex/u32 value of a color
    pub fn to_u32(&self) -> u32 {
        *self as u32
    }

    /// Returns the rgb values of a color
    pub fn to_rgb(&self) -> (u8, u8, u8) {
        let x = *self as u32;
        let hex = format!("{:06x}", x);
        let r = u8::from_str_radix(&hex[0..2], 16).unwrap();
        let g = u8::from_str_radix(&hex[2..4], 16).unwrap();
        let b = u8::from_str_radix(&hex[4..6], 16).unwrap();
        (r, g, b)
    }
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
    DndEnter,
    DndDrag,
    DndLeave,
    DndRelease,
    ScreenConfigChanged,
    Fullscreen,
    ZoomGesture,
    ZoomEvent,
}

/// Defines the inputted virtual keycode
#[repr(i32)]
#[derive(Copy, Clone, PartialEq)]
pub enum Key {
    None = 0,
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

impl Key {
    /// Gets a Key from an i32
    pub fn from_i32(val: i32) -> Key {
        unsafe { std::mem::transmute(val) }
    }
    
    /// Gets a Key from a char
    pub fn from_char(val: char) -> Key {
        unsafe { std::mem::transmute(val) }
    }
}

#[allow(unreachable_patterns)]
impl std::fmt::Debug for Key {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Key::None => write!(f, "None"),
            Key::Button => write!(f, "Button"),
            Key::BackSpace => write!(f, "BackSpace"),
            Key::Tab => write!(f, "Tab"),
            Key::IsoKey => write!(f, "IsoKey"),
            Key::Enter => write!(f, "Enter"),
            Key::Pause => write!(f, "Pause"),
            Key::ScrollLock => write!(f, "ScrollLock"),
            Key::Escape => write!(f, "Escape"),
            Key::Kana => write!(f, "Kana"),
            Key::Eisu => write!(f, "Eisu"),
            Key::Yen => write!(f, "Yen"),
            Key::JISUnderscore => write!(f, "JISUnderscore"),
            Key::Home => write!(f, "Home"),
            Key::Left => write!(f, "Left"),
            Key::Up => write!(f, "Up"),
            Key::Right => write!(f, "Right"),
            Key::Down => write!(f, "Down"),
            Key::PageUp => write!(f, "PageUp"),
            Key::PageDown => write!(f, "PageDown"),
            Key::End => write!(f, "End"),
            Key::Print => write!(f, "Print"),
            Key::Insert => write!(f, "Insert"),
            Key::Menu => write!(f, "Menu"),
            Key::Help => write!(f, "Help"),
            Key::NumLock => write!(f, "NumLock"),
            Key::KP => write!(f, "KP"),
            Key::KPEnter => write!(f, "KPEnter"),
            Key::KPLast => write!(f, "KPLast"),
            Key::FLast => write!(f, "FLast"),
            Key::ShiftL => write!(f, "ShiftL"),
            Key::ShiftR => write!(f, "ShiftR"),
            Key::ControlL => write!(f, "ControlL"),
            Key::ControlR => write!(f, "ControlR"),
            Key::CapsLock => write!(f, "CapsLock"),
            Key::MetaL => write!(f, "MetaL"),
            Key::MetaR => write!(f, "MetaR"),
            Key::AltL => write!(f, "AltL"),
            Key::AltR => write!(f, "AltR"),
            Key::Delete => write!(f, "Delete"),
            _ => write!(f, "0x{:02x}", *self as i32),
        }
    }
}

/// Defines the modifiers of virtual keycodes
#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Shortcut {
    None = 0,
    Shift = 0x00010000,
    CapsLock = 0x00020000,
    Ctrl = 0x00040000,
    Alt = 0x00080000,
}

impl Shortcut {
    /// Create a shortcut from a char
    pub fn from_char(c: char) -> Shortcut {
        Shortcut::None + c
    }
}

/// Defines the types of triggers for widget callback functions
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

/// Defines the text cursor styles supported by fltk
#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum TextCursor {
    Normal,
    Caret,
    Dim,
    Block,
    Heavy,
    Simple,
}

/// Defines the cursor styles supported by fltk
#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Cursor {
    Default = 0,
    Arrow = 35,
    Cross = 66,
    Wait = 76,
    Insert = 77,
    Hand = 31,
    Help = 47,
    Move = 27,
    NS = 78,
    WE = 79,
    NWSE = 80,
    NESW = 81,
    N = 70,
    NE = 69,
    E = 49,
    SE = 8,
    S = 9,
    SW = 7,
    W = 36,
    NW = 68,
    None = 255,
}

/// Defines Fl_Mode types
#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Mode {
    Rgb = 0,
    Index = 1,
    Double = 2,
    Accum = 4,
    Alpha = 8,
    Depth = 16,
    Stencil = 32,
    Rgb8 = 64,
    MultiSample = 128,
    Stereo = 256,
    FakeSingle = 512, // Fake single buffered windows using double-buffer
    Opengl3 = 1024,
}

pub trait WidgetType {
    fn to_int(self) -> i32;
    fn from_i32(val: i32) -> Self;
}

impl std::ops::Add<char> for Shortcut {
    type Output = Shortcut;
    fn add(self, other: char) -> Self::Output {
        unsafe { std::mem::transmute(self as i32 + other as i32) }
    }
}

impl std::ops::Add<Key> for Shortcut {
    type Output = Shortcut;
    fn add(self, other: Key) -> Self::Output {
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

impl std::ops::BitOr<Mode> for Mode {
    type Output = Mode;
    fn bitor(self, rhs: Mode) -> Self::Output {
        unsafe { std::mem::transmute(self as i32 | rhs as i32) }
    }
}
