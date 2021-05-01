use crate::app::{font_index, FONTS};
use fltk_sys::fl::Fl_get_rgb_color;

/// Defines label types
#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum LabelType {
    /// Normal
    Normal = 0,
    /// None
    None,
    /// Shadow
    Shadow,
    /// Engraved
    Engraved,
    /// Embossed
    Embossed,
}

/// Defines the color depth for drawing and rgb images
#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ColorDepth {
    /// Luminance/grayscale
    L8 = 1,
    /// Luminance with Alpha channel
    La8 = 2,
    /// RGB888
    Rgb8 = 3,
    /// RGBA8888 with Alpha channel
    Rgba8 = 4,
}

/// Defines the frame types which can be set using the `set_frame()` and `set_down_frame()` methods
#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum FrameType {
    /// No Box
    NoBox = 0,
    /// Flat Box
    FlatBox,
    /// Up Box
    UpBox,
    /// Down Box
    DownBox,
    /// Up Frame
    UpFrame,
    /// Down Frame
    DownFrame,
    /// Thin Up Box
    ThinUpBox,
    /// Thin Down Box
    ThinDownBox,
    /// Thin Up Frame
    ThinUpFrame,
    /// Thin Down Frame
    ThinDownFrame,
    /// Engraved Box
    EngravedBox,
    /// Embossed Box
    EmbossedBox,
    /// Engraved Frame
    EngravedFrame,
    /// Embossed Frame
    EmbossedFrame,
    /// Border Box
    BorderBox,
    /// Shadow Box
    ShadowBox,
    /// Border Frame
    BorderFrame,
    /// Shadow Frame
    ShadowFrame,
    /// Rounded Box
    RoundedBox,
    /// Rounded Shadow Box
    RShadowBox,
    /// Rounded Frame
    RoundedFrame,
    /// Rounded Flat Box
    RFlatBox,
    /// Round Up Box
    RoundUpBox,
    /// Round Down Box
    RoundDownBox,
    /// Diamond Up Box
    DiamondUpBox,
    /// Diamond Down Box
    DiamondDownBox,
    /// Oval Box
    OvalBox,
    /// Oval Shadow Box
    OShadowBox,
    /// Oval Frame
    OvalFrame,
    /// Oval Flat Frame
    OFlatFrame,
    /// Plastic Up Box
    PlasticUpBox,
    /// Plastic Down Box
    PlasticDownBox,
    /// Plastic Up Frame
    PlasticUpFrame,
    /// Plastic Down Frame
    PlasticDownFrame,
    /// Plastic Thin Up Box
    PlasticThinUpBox,
    /// Plastic Thin Down Box
    PlasticThinDownBox,
    /// Plastic Round Up Box
    PlasticRoundUpBox,
    /// Plastic Round Down Box
    PlasticRoundDownBox,
    /// Gtk Up Box
    GtkUpBox,
    /// Gtk Down Box
    GtkDownBox,
    /// Gtk Up Frame
    GtkUpFrame,
    /// Gtk Down Frame
    GtkDownFrame,
    /// Gtk Thin Up Box
    GtkThinUpBox,
    /// Gtk Thin Down Box
    GtkThinDownBox,
    /// Gtk Thin Up Frame
    GtkThinUpFrame,
    /// Gtk Thin Down Frame
    GtkThinDownFrame,
    /// Gtk Round Up Frame
    GtkRoundUpFrame,
    /// Gtk Round Down Frame
    GtkRoundDownFrame,
    /// Gleam Up Box
    GleamUpBox,
    /// Gleam Down Box
    GleamDownBox,
    /// Gleam Up Frame
    GleamUpFrame,
    /// Gleam Down Frame
    GleamDownFrame,
    /// Gleam Thin Up Box
    GleamThinUpBox,
    /// Gleam Thin Down Box
    GleamThinDownBox,
    /// Gleam Round Up Box
    GleamRoundUpBox,
    /// Gleam Round Down Box
    GleamRoundDownBox,
    /// Free BoxType
    FreeBoxType,
}

impl FrameType {
    /// Gets the Frame type by index
    pub fn by_index(idx: usize) -> FrameType {
        let idx = if idx > 56 { 56 } else { idx };
        unsafe { std::mem::transmute(idx as i32) }
    }

    /// Get the frame's x offset
    pub fn dx(self) -> i32 {
        unsafe { fltk_sys::fl::Fl_box_dx(self as i32) }
    }

    /// Get the frame's y offset
    pub fn dy(self) -> i32 {
        unsafe { fltk_sys::fl::Fl_box_dy(self as i32) }
    }

    /// Get the frame's width offset
    pub fn dw(self) -> i32 {
        unsafe { fltk_sys::fl::Fl_box_dw(self as i32) }
    }

    /// Get the frame's height offset
    pub fn dh(self) -> i32 {
        unsafe { fltk_sys::fl::Fl_box_dh(self as i32) }
    }
}

bitflags! {
    /// Defines alignment rules used by FLTK for labels
    pub struct Align: i32 {
        /// Center
        const Center = 0;
        /// Top
        const Top = 1;
        /// Bottom
        const Bottom = 2;
        /// Left
        const Left = 4;
        /// Right
        const Right = 8;
        /// Inside
        const Inside = 16;
        /// Text Over Image
        const TextOverImage = 20;
        /// Clip
        const Clip = 40;
        /// Wrap
        const Wrap = 80;
        /// Image Next To Text
        const ImageNextToText = 100;
        /// Text Next To Image
        const TextNextToImage = 120;
        /// Image Backdrop
        const ImageBackdrop = 200;
        /// Top Left
        const TopLeft = 1 | 4;
        /// Top Right
        const TopRight = 1 | 8;
        /// Bottom Left
        const BottomLeft = 2 | 4;
        /// Bottom Right
        const BottomRight = 2 | 8;
        /// Left Top
        const LeftTop = 7;
        /// Right Top
        const RightTop = 11;
        /// Left Bottom
        const LeftBottom = 13;
        /// Right Bottom
        const RightBottom = 14;
        /// Position Mask
        const PositionMask = 15;
        /// Image Mask
        const ImageMask = 320;
    }
}

bitflags! {
    /// Defines fonts used by FLTK
    pub struct Font: i32 {
        /// Helvetica
        const Helvetica = 0;
        /// Helvetica Bold
        const HelveticaBold = 1;
        /// Helvetica Italic
        const HelveticaItalic = 2;
        /// Helvetica Bold Italic
        const HelveticaBoldItalic = 3;
        /// Courier
        const Courier = 4;
        /// Courier Bold
        const CourierBold = 5;
        /// Courier Italic
        const CourierItalic = 6;
        /// Courier Bold Italic
        const CourierBoldItalic = 7;
        /// Times
        const Times = 8;
        /// Times Bold
        const TimesBold = 9;
        /// Times Italic
        const TimesItalic = 10;
        /// Times Bold Italic
        const TimesBoldItalic = 11;
        /// Symbol
        const Symbol = 12;
        /// Screen
        const Screen = 13;
        /// Screen Bold
        const ScreenBold = 14;
        /// Zapfdingbats
        const Zapfdingbats = 15;
    }
}

impl Font {
    /// Returns a font by index, can be queried via the `app::get_font_names()`
    pub fn by_index(idx: usize) -> Font {
        unsafe {
            if idx < (*FONTS.lock().unwrap()).len() {
                std::mem::transmute(idx as i32)
            } else {
                Font::Helvetica
            }
        }
    }

    /// Gets the font by its name, can be queried via the `app::get_font_names()`
    pub fn by_name(name: &str) -> Font {
        match font_index(name) {
            Some(val) => Font::by_index(val),
            None => Font::Helvetica,
        }
    }
}

bitflags! {
    /// Defines colors used by FLTK.
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
        /// ForeGround, label colors
        const ForeGround = 0;
        /// BackGround2, Is the color inside input, output and text display widgets
        const BackGround2 = 7;
        /// Inactive
        const Inactive = 8;
        /// Selection
        const Selection = 15;
        /// Gray0
        const Gray0 = 32;
        /// Dark3
        const Dark3 = 39;
        /// Dark2
        const Dark2 = 45;
        /// Dark1
        const Dark1 = 47;
        /// FrameDefault
        const FrameDefault = 49;
        /// BackGround
        const BackGround = 49;
        /// Light1
        const Light1 = 50;
        /// Light2
        const Light2 = 52;
        /// Light3
        const Light3 = 54;
        /// Black
        const Black = 56;
        /// Red
        const Red = 88;
        /// Green
        const Green = 63;
        /// Yellow
        const Yellow = 95;
        /// Blue
        const Blue = 216;
        /// Magenta
        const Magenta = 248;
        /// Cyan
        const Cyan = 223;
        /// DarkRed
        const DarkRed = 72;
        /// DarkGreen
        const DarkGreen = 60;
        /// DarkYellow
        const DarkYellow = 76;
        /// DarkBlue
        const DarkBlue = 136;
        /// DarkMagenta
        const DarkMagenta = 152;
        /// DarkCyan
        const DarkCyan = 140;
        /// White
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

    /// Returns a color from hex or decimal
    pub fn from_hex(val: u32) -> Color {
        let (r, g, b) = crate::utils::hex2rgb(val);
        Color::from_rgb(r, g, b)
    }

    /// Returns a color by index of RGBI
    pub fn by_index(idx: u8) -> Color {
        unsafe { std::mem::transmute(idx as u32) }
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
            }
        }
    }
}

/// Defines event types captured by FLTK
#[repr(i32)]
#[derive(Copy, Clone, PartialEq, PartialOrd)]
#[non_exhaustive]
pub enum Event {
    /// No Event
    NoEvent = 0,
    /// Push
    Push,
    /// Released
    Released,
    /// Enter
    Enter,
    /// Leave
    Leave,
    /// Drag
    Drag,
    /// Focus
    Focus,
    /// Unfocus
    Unfocus,
    /// KeyDown
    KeyDown,
    /// KeyUp
    KeyUp,
    /// Close
    Close,
    /// Move
    Move,
    /// Shortcut
    Shortcut,
    /// Deactivate
    Deactivate,
    /// Activate
    Activate,
    /// Hide
    Hide,
    /// Show
    Show,
    /// Paste
    Paste,
    /// Selection Clear
    SelectionClear,
    /// MouseWheel
    MouseWheel,
    /// DndEnter
    DndEnter,
    /// Drag n Drop: Drag
    DndDrag,
    /// Drag n Drop: Leave
    DndLeave,
    /// Drag n Drop: Release
    DndRelease,
    /// Screen Config Changed
    ScreenConfigChanged,
    /// Fullscreen
    Fullscreen,
    /// Zoom Gesture
    ZoomGesture,
    /// Zoom Event
    ZoomEvent,
    /// Window Resize Event.
    /// Avoid resizing the widget during a resize event
    /// to avoid infinite recursion
    Resize,
}

impl Event {
    /// Creates an event from an i32 value
    pub fn from_i32(val: i32) -> Event {
        unsafe { std::mem::transmute(val) }
    }
}

impl Into<i32> for Event {
    fn into(self) -> i32 {
        self as i32
    }
}

#[allow(unreachable_patterns)]
impl std::fmt::Debug for Event {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Event::NoEvent => write!(f, "NoEvent"),
            Event::Push => write!(f, "Push"),
            Event::Released => write!(f, "Released"),
            Event::Enter => write!(f, "Enter"),
            Event::Leave => write!(f, "Leave"),
            Event::Drag => write!(f, "Drag"),
            Event::Focus => write!(f, "Focus"),
            Event::Unfocus => write!(f, "Unfocus"),
            Event::KeyDown => write!(f, "KeyDown"),
            Event::KeyUp => write!(f, "KeyUp"),
            Event::Close => write!(f, "Close"),
            Event::Move => write!(f, "Move"),
            Event::Shortcut => write!(f, "Shortcut"),
            Event::Deactivate => write!(f, "Deactivate"),
            Event::Activate => write!(f, "Activate"),
            Event::Hide => write!(f, "Hide"),
            Event::Show => write!(f, "Show"),
            Event::Paste => write!(f, "Paste"),
            Event::SelectionClear => write!(f, "SelectionClear"),
            Event::MouseWheel => write!(f, "MouseWheel"),
            Event::DndEnter => write!(f, "DndEnter"),
            Event::DndDrag => write!(f, "DndDrag"),
            Event::DndLeave => write!(f, "DndLeave"),
            Event::DndRelease => write!(f, "DndRelease"),
            Event::ScreenConfigChanged => write!(f, "ScreenConfigChanged"),
            Event::Fullscreen => write!(f, "Fullscreen"),
            Event::ZoomGesture => write!(f, "ZoomGesture"),
            Event::ZoomEvent => write!(f, "ZoomEvent"),
            Event::Resize => write!(f, "Resize"),
            _ => {
                write!(f, "Event::from_i32({})", *self as i32)
            }
        }
    }
}

bitflags! {
    /// Defines the inputted virtual keycode
    pub struct Key: i32 {
        /// None
        const None = 0;
        /// Button
        const Button = 0xfee8;
        /// BackSpace
        const BackSpace = 0xff08;
        /// Tab
        const Tab = 0xff09;
        /// IsoKey
        const IsoKey = 0xff0c;
        /// Enter
        const Enter = 0xff0d;
        /// Pause
        const Pause = 0xff13;
        /// ScrollLock
        const ScrollLock = 0xff14;
        /// Escape
        const Escape = 0xff1b;
        /// Kana
        const Kana = 0xff2e;
        /// Eisu
        const Eisu = 0xff2f;
        /// Yen
        const Yen = 0xff30;
        /// JISUnderscore
        const JISUnderscore = 0xff31;
        /// Home
        const Home = 0xff50;
        /// Left
        const Left = 0xff51;
        /// Up
        const Up = 0xff52;
        /// Right
        const Right = 0xff53;
        /// Down
        const Down = 0xff54;
        /// PageUp
        const PageUp = 0xff55;
        /// PageDown
        const PageDown = 0xff56;
        /// End
        const End = 0xff57;
        /// Print
        const Print = 0xff61;
        /// Insert
        const Insert = 0xff63;
        /// Menu
        const Menu = 0xff67;
        /// Help
        const Help = 0xff68;
        /// NumLock
        const NumLock = 0xff7f;
        /// Keypad
        const KP = 0xff80;
        /// Keypad Enter
        const KPEnter = 0xff8d;
        /// Keypad Last
        const KPLast = 0xffbd;
        /// FLast
        const FLast = 0xffe0;
        /// Shift Left
        const ShiftL = 0xffe1;
        /// Shift Right
        const ShiftR = 0xffe2;
        /// Control Left
        const ControlL = 0xffe3;
        /// Control Right
        const ControlR = 0xffe4;
        /// Caps Lock
        const CapsLock = 0xffe5;
        /// Meta Left
        const MetaL = 0xffe7;
        /// Meta Right
        const MetaR = 0xffe8;
        /// Alt Left
        const AltL = 0xffe9;
        /// Alt Right
        const AltR = 0xffea;
        /// Delete
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
        /// None
        const None = 0;
        /// Shift
        const Shift = 0x0001_0000;
        /// Caps Lock
        const CapsLock = 0x0002_0000;
        /// Ctrl
        const Ctrl = 0x0004_0000;
        /// Alt
        const Alt = 0x0008_0000;
        /// Meta
        const Meta = 0x0040_0000;
        /// Command (Meta for macOS, Ctrl for other systems)
        const Command = if cfg!(target_os = "macos") {
            Shortcut::Meta.bits
        } else {
            Shortcut::Ctrl.bits
        };
        /// Control (Ctrl for macOS, Meta for other systems)
        const Control = if cfg!(target_os = "macos") {
            Shortcut::Ctrl.bits
        } else {
            Shortcut::Meta.bits
        };
        /// Mouse button 1 is pushed
        const Button1 = 0x0100_0000;
        /// Mouse button 2 is pushed
        const Button2 = 0x0200_0000;
        /// Mouse button 3 is pushed
        const Button3 = 0x0400_0000;
        /// Any mouse button is pushed
        const Buttons = 0x7f00_0000;
    }
}

/// Alias reflecting FLTK's name
pub type EventState = Shortcut;

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

    /// get key mask
    pub fn key(&self) -> Key {
        let mut temp = self.bits;
        temp &= 0x0000_ffff;
        Key::from_i32(temp)
    }

    /// Get the button number
    pub fn button(button_num: i32) -> Shortcut {
        unsafe { std::mem::transmute(0x0080_0000 << button_num) }
    }
}

bitflags! {
    /// Defines the types of triggers for widget callback functions
    pub struct CallbackTrigger: i32 {
        /// Never
        const Never = 0;
        /// Changed
        const Changed = 1;
        /// Not Changed
        const NotChanged = 2;
        /// Release
        const Release = 4;
        /// Release Always
        const ReleaseAlways = 6;
        /// Enter Key
        const EnterKey = 8;
        /// Enter Key Always
        const EnterKeyAlways = 10;
        /// Enter Key and Changed
        const EnterKeyChanged = 11;
    }
}

/// Defines the cursor styles supported by fltk
#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Cursor {
    /// Default
    Default = 0,
    /// Arrow
    Arrow = 35,
    /// Cross
    Cross = 66,
    /// Wait
    Wait = 76,
    /// Insert
    Insert = 77,
    /// Hand
    Hand = 31,
    /// Help
    Help = 47,
    /// Move
    Move = 27,
    /// North South
    NS = 78,
    /// West East
    WE = 79,
    /// North West - South East
    NWSE = 80,
    /// North East - South West
    NESW = 81,
    /// North
    N = 70,
    /// North East
    NE = 69,
    /// East
    E = 49,
    /// South East
    SE = 8,
    /// South
    S = 9,
    /// South West
    SW = 7,
    /// West
    W = 36,
    /// North West
    NW = 68,
    /// None
    None = 255,
}

bitflags! {
    /// Defines visual mode types (capabilites of the window).
    /// Rgb and Single have a value of zero, so they
    /// are "on" unless you give Index or Double.
    pub struct Mode: i32 {
        /// Rgb color (not indexed)
        const Rgb = 0;
        /// Single buffered
        const Single = 0;
        /// Indexed mode
        const Index = 1;
        /// Double buffered
        const Double = 2;
        /// Accumulation buffer
        const Accum = 4;
        /// Alpha channel in color
        const Alpha = 8;
        /// Depth buffer
        const Depth = 16;
        /// Stencil buffer
        const Stencil = 32;
        /// Rgb8 color with at least 8 bits of each color
        const Rgb8 = 64;
        /// MultiSample antialiasing
        const MultiSample = 128;
        /// Stereoscopic rendering
        const Stereo = 256;
        /// Fake single buffered windows using double-buffer
        const FakeSingle = 512; //
        /// Use OpenGL version 3.0 or more
        const Opengl3 = 1024;
    }
}

bitflags! {
    /// Damage masks
    pub struct Damage: u8 {
        /// No damage
        const  None = 0x00;
        /// A child needs to be redrawn.
        const  Child    = 0x01;
        /// The window was exposed.
        const Expose   = 0x02;
        /// The Fl_Scroll widget was scrolled.
        const Scroll   = 0x04;
        /// The overlay planes need to be redrawn.
        const Overlay  = 0x08;
        /// First user-defined damage bit.
        const User1    = 0x10;
        /// Second user-defined damage bit.
        const User2    = 0x20;
        /// Everything needs to be redrawn.
        const All     = 0x80;
    }
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
