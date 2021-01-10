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

bitflags! {
    /// Defines alignment rules used by FLTK for labels
    pub struct Align: i32 {
        const Center = 0;
        const Top = 1;
        const Bottom = 2;
        const Left = 4;
        const Right = 8;
        const Inside = 16;
        const TextOverImage = 20;
        const Clip = 40;
        const Wrap = 80;
        const ImageNextToText = 100;
        const TextNextToImage = 120;
        const ImageBackdrop = 200;
        const TopLeft = 1 | 4;
        const TopRight = 1 | 8;
        const BottomLeft = 2 | 4;
        const BottomRight = 2 | 8;
        const LeftTop = 7;
        const RightTop = 11;
        const LeftBottom = 13;
        const RightBottom = 14;
        const PositionMask = 15;
        const ImageMask = 320;
    }
}

bitflags! {
    /// Defines fonts used by FLTK
    pub struct Font: i32 {
        const Helvetica = 0;
        const HelveticaBold = 1;
        const HelveticaItalic = 2;
        const HelveticaBoldItalic = 3;
        const Courier = 4;
        const CourierBold = 5;
        const CourierItalic = 6;
        const CourierBoldItalic = 7;
        const Times = 8;
        const TimesBold = 9;
        const TimesItalic = 10;
        const TimesBoldItalic = 11;
        const Symbol = 12;
        const Screen = 13;
        const ScreenBold = 14;
        const Zapfdingbats = 15;
    }
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

bitflags! {
    /// Defines colors used by FLTK
    /// Colors are stored as RGBI values, the last being the index for FLTK colors in this enum. 
    /// Colors in this enum don't have an RGB stored. However, custom colors have an RGB, and don't have an index.
    /// The RGBI can be acquired by casting the color to u32 and formatting it to ```0x{08x}```.
    /// The last 2 digits are the hexadecimal representation of the color in this enum.
    /// For example, Color::White, has a hex of 0x000000ff, ff being the 255 value of this enum. 
    /// A custom color like Color::from_u32(0x646464), will have an representation as 0x64646400,
    /// of which the final 00 indicates that it is not stored in this enum.
    /// For convenience, the fmt::Display trait is implemented so that the name of the Color is shown
    /// when there is one, otherwise the RGB value is given.
    pub struct Color: u32 {
        const ForeGround = 0;
        const BackGround2 = 7;
        const Inactive = 8;
        const Selection = 15;
        const Gray0 = 32;
        const Dark3 = 39;
        const Dark2 = 45;
        const Dark1 = 47;
        const FrameDefault = 49;
        const BackGround = 49;
        const Light1 = 50;
        const Light2 = 52;
        const Light3 = 54;
        const Black = 56;
        const Red = 88;
        const Green = 63;
        const Yellow = 95;
        const Blue = 216;
        const Magenta = 248;
        const Cyan = 223;
        const DarkRed = 72;
        const DarkGreen = 60;
        const DarkYellow = 76;
        const DarkBlue = 136;
        const DarkMagenta = 152;
        const DarkCyan = 140;
        const White = 255;
    }
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

    /// Returns a color by index of RGBI
    pub fn by_index(idx: u8) -> Color {
        unsafe {
            std::mem::transmute(idx as u32)
        }
    }
}

#[allow(unreachable_patterns)]
impl std::fmt::Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Color::ForeGround => write!(f, "ForeGround"),
            Color::BackGround => write!(f, "BackGround"),
            Color::Inactive => write!(f, "Inactive"),
            Color::Selection => write!(f, "Selection"),
            Color::Gray0 => write!(f, "Gray0"),
            Color::Dark3 => write!(f, "Dark3"),
            Color::Dark2 => write!(f, "Dark2"),
            Color::Dark1 => write!(f, "Dark1"),
            Color::FrameDefault => write!(f, "FrameDefault"),
            Color::Light1 => write!(f, "Light1"),
            Color::Light2 => write!(f, "Light2"),
            Color::Light3 => write!(f, "Light3"),
            Color::Black => write!(f, "Black"),
            Color::Red => write!(f, "Red"),
            Color::Green => write!(f, "Green"),
            Color::Yellow => write!(f, "Yellow"),
            Color::Blue => write!(f, "Blue"),
            Color::Magenta => write!(f, "Magenta"),
            Color::Cyan => write!(f, "Cyan"),
            Color::DarkRed => write!(f, "DarkRed"),
            Color::DarkGreen => write!(f, "DarkGreen"),
            Color::DarkYellow => write!(f, "DarkYellow"),
            Color::DarkBlue => write!(f, "DarkBlue"),
            Color::DarkMagenta => write!(f, "DarkMagenta"),
            Color::DarkCyan => write!(f, "DarkCyan"),
            Color::White => write!(f, "White"),
            _ => { 
                let temp = format!("{:08x}", self.bits);
                write!(f, "0x{}", &temp[0..6])
            },
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

bitflags! {
    /// Defines the inputted virtual keycode
    pub struct Key: i32 {
        const None = 0;
        const Button = 0xfee8;
        const BackSpace = 0xff08;
        const Tab = 0xff09;
        const IsoKey = 0xff0c;
        const Enter = 0xff0d;
        const Pause = 0xff13;
        const ScrollLock = 0xff14;
        const Escape = 0xff1b;
        const Kana = 0xff2e;
        const Eisu = 0xff2f;
        const Yen = 0xff30;
        const JISUnderscore = 0xff31;
        const Home = 0xff50;
        const Left = 0xff51;
        const Up = 0xff52;
        const Right = 0xff53;
        const Down = 0xff54;
        const PageUp = 0xff55;
        const PageDown = 0xff56;
        const End = 0xff57;
        const Print = 0xff61;
        const Insert = 0xff63;
        const Menu = 0xff67;
        const Help = 0xff68;
        const NumLock = 0xff7f;
        const KP = 0xff80;
        const KPEnter = 0xff8d;
        const KPLast = 0xffbd;
        const FLast = 0xffe0;
        const ShiftL = 0xffe1;
        const ShiftR = 0xffe2;
        const ControlL = 0xffe3;
        const ControlR = 0xffe4;
        const CapsLock = 0xffe5;
        const MetaL = 0xffe7;
        const MetaR = 0xffe8;
        const AltL = 0xffe9;
        const AltR = 0xffea;
        const Delete = 0xffff;
    }
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

bitflags! {
    /// Defines the modifiers of virtual keycodes
    pub struct Shortcut: i32 {
        const None = 0;
        const Shift = 0x0001_0000;
        const CapsLock = 0x0002_0000;
        const Ctrl = 0x0004_0000;
        const Alt = 0x0008_0000;
    }
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

bitflags! {
    /// Defines the types of triggers for widget callback functions
    pub struct CallbackTrigger: i32 {
        const Never = 0;
        const Changed = 1;
        const NotChanged = 2;
        const Release = 4;
        const ReleaseAlways = 6;
        const EnterKey = 8;
        const EnterKeyAlways = 10;
        const EnterKeyChanged = 11;
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

bitflags! {
    /// Defines Fl_Mode types
    pub struct Mode: i32 {
        const Rgb = 0;
        const Index = 1;
        const Double = 2;
        const Accum = 4;
        const Alpha = 8;
        const Depth = 16;
        const Stencil = 32;
        const Rgb8 = 64;
        const MultiSample = 128;
        const Stereo = 256;
        const FakeSingle = 512; // Fake single buffered windows using double-buffer
        const Opengl3 = 1024;
    }
}

pub trait WidgetType {
    fn to_int(self) -> i32;
    fn from_i32(val: i32) -> Self;
}

impl std::ops::BitOr<char> for Shortcut {
    type Output = Shortcut;
    fn bitor(self, other: char) -> Self::Output {
        unsafe { std::mem::transmute(self.bits as i32 | other as i32) }
    }
}

impl std::ops::BitOr<Key> for Shortcut {
    type Output = Shortcut;
    fn bitor(self, other: Key) -> Self::Output {
        unsafe { std::mem::transmute(self.bits as i32 | other.bits() as i32) }
    }
}

impl std::ops::BitOr<i32> for Align {
    type Output = Align;
    fn bitor(self, rhs: i32) -> Self::Output {
        unsafe { std::mem::transmute(self.bits | rhs as i32) }
    }
}
