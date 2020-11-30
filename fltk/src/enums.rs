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
        let idx = if idx > 56 { 56 } else { idx };
        unsafe { std::mem::transmute(idx as i32) }
    }
}

/// Defines alignment rules used by FLTK for labels
#[repr(i32)]
#[derive(Copy, Clone, PartialEq)]
pub enum Align {
    Center = 0,
    Top = 1,
    Bottom = 2,
    Left = 4,
    Right = 8,
    Inside = 16,
    TextOverImage = 20,
    Clip = 40,
    Wrap = 80,
    ImageNextToText = 100,
    TextNextToImage = 120,
    ImageBackdrop = 200,
    TopLeft = 1 | 4,
    TopRight = 1 | 8,
    BottomLeft = 2 | 4,
    BottomRight = 2 | 8,
    LeftTop = 7,
    RightTop = 11,
    LeftBottom = 13,
    RightBottom = 14,
    PositionMask = 15,
    ImageMask = 320,
}

#[allow(unreachable_patterns)]
impl std::fmt::Debug for Align {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use Align::*;
        match *self {
            Center => write!(f, "Align::Center"),
            Top => write!(f, "Align::Top"),
            Bottom => write!(f, "Align::Bottom"),
            Left => write!(f, "Align::Left"),
            Right => write!(f, "Align::Right"),
            Inside => write!(f, "Align::Inside"),
            TextOverImage => write!(f, "Align::TextOverImage"),
            Clip => write!(f, "Align::Clip"),
            Wrap => write!(f, "Align::Wrap"),
            ImageNextToText => write!(f, "Align::ImageNextToText"),
            TextNextToImage => write!(f, "Align::TextNextToImage"),
            ImageBackdrop => write!(f, "Align::ImageBackdrop"),
            TopLeft => write!(f, "Align::TopLeft"),
            TopRight => write!(f, "Align::TopRight"),
            BottomLeft => write!(f, "Align::BottomLeft"),
            BottomRight => write!(f, "Align::BottomRight"),
            LeftTop => write!(f, "Align::LeftTop"),
            RightTop => write!(f, "Align::RightTop"),
            LeftBottom => write!(f, "Align::LeftBottom"),
            RightBottom => write!(f, "Align::RightBottom"),
            PositionMask => write!(f, "Align::PositionMask"),
            ImageMask => write!(f, "Align::ImageMask"),
            _ => write!(f, "0x{:02x}", *self as i32),
        }
    }
}

/// Defines fonts used by FLTK
#[repr(i32)]
#[derive(Copy, Clone, PartialEq)]
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
}

impl Font {
    /// Returns a font by index, can be queried via the app::get_font_names()
    pub fn by_index(idx: usize) -> Font {
        unsafe {
            if idx < (*FONTS.lock().unwrap()).len() {
                std::mem::transmute(idx as i32)
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


#[allow(unreachable_patterns)]
impl std::fmt::Debug for Font {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use Font::*;
        match *self {
            Helvetica => write!(f, "Font::Helvetica"),
            HelveticaBold => write!(f, "Font::HelveticaBold"),
            HelveticaItalic => write!(f, "Font::HelveticaItalic"),
            HelveticaBoldItalic => write!(f, "Font::HelveticaBoldItalic"),
            Courier => write!(f, "Font::Courier"),
            CourierBold => write!(f, "Font::CourierBold"),
            CourierItalic => write!(f, "Font::CourierItalic"),
            CourierBoldItalic => write!(f, "Font::CourierBoldItalic"),
            Times => write!(f, "Font::Times"),
            TimesBold => write!(f, "Font::TimesBold"),
            TimesItalic => write!(f, "Font::TimesItalic"),
            TimesBoldItalic => write!(f, "Font::TimesBoldItalic"),
            Symbol => write!(f, "Font::Symbol"),
            Screen => write!(f, "Font::Screen"),
            ScreenBold => write!(f, "Font::ScreenBold"),
            Zapfdingbats => write!(f, "Font::Zapfdingbats"),
            _ => write!(f, "0x{:02x}", *self as i32),
        }
    }
}

/// Defines colors used by FLTK
/// Colors are stored as RGBI values, the last being the index for FLTK colors in this enum. 
/// Colors in this enum don't have an RGB stored. However, custom colors have an RGB, and don't have an index.
/// The RGBI can be acquired by casting the color to u32 and formatting it to ```0x{08x}```.
/// The last 2 digits are the hexadecimal representation of the color in this enum.
/// For example, Color::White, has a hex of 0x000000ff, ff being the 255 value of this enum. 
/// A custom color like Color::from_u32(0x646464), will have an representation as 0x64646400,
/// of which the final 00 indicates that it is not stored in this enum.
#[repr(u32)]
#[derive(Copy, Clone, PartialEq)]
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
        let (r, g, b) = crate::utils::hex2rgb(val);
        Color::from_rgb(r, g, b)
    }
}

#[allow(unreachable_patterns)]
impl std::fmt::Debug for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use Color::*;
        match *self {
            ForeGround => write!(f, "Color::ForeGround"),
            BackGround => write!(f, "Color::BackGround"),
            Inactive => write!(f, "Color::Inactive"),
            Selection => write!(f, "Color::Selection"),
            Gray0 => write!(f, "Color::Gray0"),
            Dark3 => write!(f, "Color::Dark3"),
            Dark2 => write!(f, "Color::Dark2"),
            Dark1 => write!(f, "Color::Dark1"),
            FrameDefault => write!(f, "Color::FrameDefault"),
            Light1 => write!(f, "Color::Light1"),
            Light2 => write!(f, "Color::Light2"),
            Light3 => write!(f, "Color::Light3"),
            Black => write!(f, "Color::Black"),
            Red => write!(f, "Color::Red"),
            Green => write!(f, "Color::Green"),
            Yellow => write!(f, "Color::Yellow"),
            Blue => write!(f, "Color::Blue"),
            Magenta => write!(f, "Color::Magenta"),
            Cyan => write!(f, "Color::Cyan"),
            DarkRed => write!(f, "Color::DarkRed"),
            DarkGreen => write!(f, "Color::DarkGreen"),
            DarkYellow => write!(f, "Color::DarkYellow"),
            DarkBlue => write!(f, "Color::DarkBlue"),
            DarkMagenta => write!(f, "Color::DarkMagenta"),
            DarkCyan => write!(f, "Color::DarkCyan"),
            White => write!(f, "Color::White"),
            _ => write!(f, "{:06x}", *self as u32),
        }
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
            Key::None => write!(f, "Key::None"),
            Key::Button => write!(f, "Key::Button"),
            Key::BackSpace => write!(f, "Key::BackSpace"),
            Key::Tab => write!(f, "Key::Tab"),
            Key::IsoKey => write!(f, "Key::IsoKey"),
            Key::Enter => write!(f, "Key::Enter"),
            Key::Pause => write!(f, "Key::Pause"),
            Key::ScrollLock => write!(f, "Key::ScrollLock"),
            Key::Escape => write!(f, "Key::Escape"),
            Key::Kana => write!(f, "Key::Kana"),
            Key::Eisu => write!(f, "Key::Eisu"),
            Key::Yen => write!(f, "Key::Yen"),
            Key::JISUnderscore => write!(f, "Key::JISUnderscore"),
            Key::Home => write!(f, "Key::Home"),
            Key::Left => write!(f, "Key::Left"),
            Key::Up => write!(f, "Key::Up"),
            Key::Right => write!(f, "Key::Right"),
            Key::Down => write!(f, "Key::Down"),
            Key::PageUp => write!(f, "Key::PageUp"),
            Key::PageDown => write!(f, "Key::PageDown"),
            Key::End => write!(f, "Key::End"),
            Key::Print => write!(f, "Key::Print"),
            Key::Insert => write!(f, "Key::Insert"),
            Key::Menu => write!(f, "Key::Menu"),
            Key::Help => write!(f, "Key::Help"),
            Key::NumLock => write!(f, "Key::NumLock"),
            Key::KP => write!(f, "Key::KP"),
            Key::KPEnter => write!(f, "Key::KPEnter"),
            Key::KPLast => write!(f, "Key::KPLast"),
            Key::FLast => write!(f, "Key::FLast"),
            Key::ShiftL => write!(f, "Key::ShiftL"),
            Key::ShiftR => write!(f, "Key::ShiftR"),
            Key::ControlL => write!(f, "Key::ControlL"),
            Key::ControlR => write!(f, "Key::ControlR"),
            Key::CapsLock => write!(f, "Key::CapsLock"),
            Key::MetaL => write!(f, "Key::MetaL"),
            Key::MetaR => write!(f, "Key::MetaR"),
            Key::AltL => write!(f, "Key::AltL"),
            Key::AltR => write!(f, "Key::AltR"),
            Key::Delete => write!(f, "Key::Delete"),
            _ => write!(f, "0x{:02x}", *self as i32),
        }
    }
}

/// Defines the modifiers of virtual keycodes
#[repr(i32)]
#[derive(Copy, Clone, PartialEq)]
pub enum Shortcut {
    None = 0,
    Shift = 0x0001_0000,
    CapsLock = 0x0002_0000,
    Ctrl = 0x0004_0000,
    Alt = 0x0008_0000,
}

impl Shortcut {
    /// Create a shortcut from a char
    pub fn from_char(c: char) -> Shortcut {
        Shortcut::None | c
    }

    /// Create a shortcut from a key
    pub fn from_key(k: Key) -> Shortcut {
        Shortcut::None | k
    }

    /// Create a shortcut from an i32
    pub fn from_i32(v: i32) -> Shortcut {
        Shortcut::None | Key::from_i32(v)
    }
}

#[allow(unreachable_patterns)]
impl std::fmt::Debug for Shortcut {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use Shortcut::*;
        match *self {
            None => write!(f, "Shortcut::None"),
            Shift => write!(f, "Shortcut::Shift"),
            CapsLock => write!(f, "Shortcut::CapsLock"),
            Ctrl => write!(f, "Shortcut::Ctrl"),
            Alt => write!(f, "Shortcut::Alt"),
            _ => write!(f, "0x{:08x}", *self as i32),
        }
    }
}

/// Defines the types of triggers for widget callback functions
#[repr(i32)]
#[derive(Copy, Clone, PartialEq)]
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


#[allow(unreachable_patterns)]
impl std::fmt::Debug for CallbackTrigger {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use CallbackTrigger::*;
        match *self {
            Never => write!(f, "CallbackTrigger::Never"),
            Changed => write!(f, "CallbackTrigger::Changed"),
            NotChanged => write!(f, "CallbackTrigger::NotChanged"),
            Release => write!(f, "CallbackTrigger::Release"),
            ReleaseAlways => write!(f, "CallbackTrigger::ReleaseAlways"),
            EnterKey => write!(f, "CallbackTrigger::EnterKey"),
            EnterKeyAlways => write!(f, "CallbackTrigger::EnterKeyAlways"),
            EnterKeyChanged => write!(f, "CallbackTrigger::EnterKeyChanged"),
            _ => write!(f, "0x{:02x}", *self as i32),
        }
    }
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
#[derive(Copy, Clone, PartialEq)]
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


#[allow(unreachable_patterns)]
impl std::fmt::Debug for Mode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use Mode::*;
        match *self {
            Rgb => write!(f, "Mode::Rgb"),
            Index => write!(f, "Mode::Index"),
            Double => write!(f, "Mode::Double"),
            Accum => write!(f, "Mode::Accum"),
            Alpha => write!(f, "Mode::Alpha"),
            Depth => write!(f, "Mode::Depth"),
            Stencil => write!(f, "Mode::Stencil"),
            Rgb8 => write!(f, "Mode::Rgb8"),
            MultiSample => write!(f, "Mode::MultiSample"),
            Stereo => write!(f, "Mode::Stereo"),
            FakeSingle => write!(f, "Mode::FakeSingle"),
            Opengl3 => write!(f, "Mode::Opengl3"),
            _ => write!(f, "0x{:04x}", *self as i32),
        }
    }
}

pub trait WidgetType {
    fn to_int(self) -> i32;
    fn from_i32(val: i32) -> Self;
}

impl std::ops::BitOr<char> for Shortcut {
    type Output = Shortcut;
    fn bitor(self, other: char) -> Self::Output {
        unsafe { std::mem::transmute(self as i32 | other as i32) }
    }
}

impl std::ops::BitOr<Key> for Shortcut {
    type Output = Shortcut;
    fn bitor(self, other: Key) -> Self::Output {
        unsafe { std::mem::transmute(self as i32 | other as i32) }
    }
}

impl std::ops::BitOr<Shortcut> for Shortcut {
    type Output = Shortcut;
    fn bitor(self, other: Shortcut) -> Self::Output {
        unsafe { std::mem::transmute(self as i32 | other as i32) }
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

impl std::ops::BitOr<i32> for Align {
    type Output = Align;
    fn bitor(self, rhs: i32) -> Self::Output {
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
