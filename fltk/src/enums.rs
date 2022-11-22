use crate::app::{font_index, FONTS};
use crate::prelude::{FltkError, FltkErrorKind};
use crate::utils::{self, FlString};
use fltk_sys::fl;
use std::{
    ffi::{CStr, CString},
    mem, path,
};

/// Defines label types
#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
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
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
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

/// Implements some convenience methods for ColorDepth
impl ColorDepth {
    /// Create a ColorDepth from an u8 value
    pub fn from_u8(val: u8) -> Result<ColorDepth, FltkError> {
        if !(1..=4).contains(&val) {
            Err(FltkError::Internal(FltkErrorKind::FailedOperation))
        } else {
            Ok(unsafe { mem::transmute(val) })
        }
    }
}

/// Defines the frame types which can be set using the `set_frame()` and `set_down_frame()` methods
#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
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
    /// Alias OFlatFrame as OFlatBox
    pub const OFlatBox: FrameType = FrameType::OFlatFrame;
    /// Alias GtkRoundDownFrame as GtkRoundDownBox
    pub const GtkRoundDownBox: FrameType = FrameType::GtkRoundDownFrame;
    /// Gets the Frame type by index
    pub fn by_index(idx: usize) -> FrameType {
        let idx = if idx > 56 { 56 } else { idx };
        unsafe { mem::transmute(idx as i32) }
    }

    /// Get the frame's x offset
    pub fn dx(self) -> i32 {
        unsafe { fl::Fl_box_dx(self as i32) }
    }

    /// Get the frame's y offset
    pub fn dy(self) -> i32 {
        unsafe { fl::Fl_box_dy(self as i32) }
    }

    /// Get the frame's width offset
    pub fn dw(self) -> i32 {
        unsafe { fl::Fl_box_dw(self as i32) }
    }

    /// Get the frame's height offset
    pub fn dh(self) -> i32 {
        unsafe { fl::Fl_box_dh(self as i32) }
    }

    /// Swap frames
    pub fn swap_frames(old_frame: FrameType, new_frame: FrameType) {
        unsafe {
            let new_frame = new_frame as i32;
            let old_frame = old_frame as i32;
            fl::Fl_set_box_type(old_frame, new_frame);
        }
    }
}

bitflags::bitflags! {
    /// Defines alignment rules used by FLTK for labels
    pub struct Align: i32 {
        /// Center
        const Center = 0x0000;
        /// Top
        const Top = 0x0001;
        /// Bottom
        const Bottom = 0x0002;
        /// Left
        const Left = 0x0004;
        /// Right
        const Right = 0x0008;
        /// Inside
        const Inside = 0x0010;
        /// Text Over Image
        const TextOverImage = 0x0020;
        /// Clip
        const Clip = 0x0040;
        /// Wrap
        const Wrap = 0x0080;
        /// Image Next To Text
        const ImageNextToText = 0x0100;
        /// Text Next To Image
        const TextNextToImage = 0x0120;
        /// Image Backdrop
        const ImageBackdrop = 0x0200;
        /// Top Left
        const TopLeft = 0x0001 | 0x0004;
        /// Top Right
        const TopRight = 0x0001 | 0x0008;
        /// Bottom Left
        const BottomLeft = 0x0002 | 0x0004;
        /// Bottom Right
        const BottomRight = 0x0002 | 0x0008;
        /// Left Top
        const LeftTop = 0x0007;
        /// Right Top
        const RightTop = 0x000B;
        /// Left Bottom
        const LeftBottom = 0x000D;
        /// Right Bottom
        const RightBottom = 0x000E;
        /// Position Mask
        const PositionMask = 0x000F;
        /// Image Mask
        const ImageMask = 0x0320;
    }
}

bitflags::bitflags! {
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
    /// Returns a font by index, can be queried via the [`app::get_font_names()`](`crate::app::get_font_names()`)
    pub fn by_index(idx: usize) -> Font {
        if idx < (FONTS.lock().unwrap()).len() {
            unsafe { mem::transmute(idx as i32) }
        } else {
            Font::Helvetica
        }
    }

    /// Gets the font by its name, can be queried via the [`app::get_font_names()`](`crate::app::get_font_names`)
    pub fn by_name(name: &str) -> Font {
        match font_index(name) {
            Some(val) => Font::by_index(val),
            None => Font::Helvetica,
        }
    }

    /**
        Replace a current font with a loaded font
        ```rust,no_run
        use fltk::enums::Font;
        let font = Font::load_font("font.ttf").unwrap();
        Font::set_font(Font::Helvetica, &font);
        ```
    */
    pub fn set_font(old: Font, new: &str) {
        let new = CString::safe_new(new);
        unsafe {
            fl::Fl_set_font2(old.bits, new.into_raw() as _);
        }
    }

    /**
        Load font from file.
        ```rust,no_run
        use fltk::enums::Font;
        let font = Font::load_font("font.ttf").unwrap();
        Font::set_font(Font::Helvetica, &font);
        ```
    */
    pub fn load_font<P: AsRef<path::Path>>(path: P) -> Result<String, FltkError> {
        Font::load_font_(path.as_ref())
    }

    fn load_font_(path: &path::Path) -> Result<String, FltkError> {
        unsafe {
            if !path.exists() {
                return Err::<String, FltkError>(FltkError::Internal(
                    FltkErrorKind::ResourceNotFound,
                ));
            }
            if let Some(p) = path.to_str() {
                let font_data = std::fs::read(path)?;
                let face = match ttf_parser::Face::parse(&font_data, 0) {
                    Ok(f) => f,
                    Err(_) => {
                        return Err(FltkError::Internal(FltkErrorKind::FailedOperation));
                    }
                };
                let family_name = face
                    .names()
                    .into_iter()
                    .find(|name| {
                        name.name_id == ttf_parser::name_id::FULL_NAME && name.is_unicode()
                    })
                    .and_then(|name| name.to_string());
                let path = CString::safe_new(p);
                let ret = fl::Fl_load_font(path.as_ptr());
                if let Some(family_name) = family_name {
                    if ret > 0 {
                        Ok(family_name)
                    } else {
                        Err(FltkError::Internal(FltkErrorKind::FailedOperation))
                    }
                } else {
                    Err(FltkError::Internal(FltkErrorKind::FailedOperation))
                }
            } else {
                Err(FltkError::Internal(FltkErrorKind::FailedOperation))
            }
        }
    }

    /// Get the font's real name
    pub fn get_name(&self) -> String {
        unsafe {
            CStr::from_ptr(fl::Fl_get_font_name(self.bits as i32))
                .to_string_lossy()
                .to_string()
        }
    }
}

bitflags::bitflags! {
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
        /// Foreground, label colors
        const Foreground = 0;
        /// BackGround2, Is the color inside input, output and text display widgets
        const BackGround2 = 7;
        /// Background2, Is the color inside input, output and text display widgets
        const Background2 = 7;
        /// Inactive
        const Inactive = 8;
        /// Selection
        const Selection = 15;
        /// Free
        const Free = 16;
        /// Gray0
        const Gray0 = 32;
        /// GrayRamp
        const GrayRamp = 32;
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
        /// Background
        const Background = 49;
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
    pub const fn from_rgb(r: u8, g: u8, b: u8) -> Color {
        let r = r as u32;
        let g = g as u32;
        let b = b as u32;
        let val: u32 = ((r & 0xff) << 24) + ((g & 0xff) << 16) + ((b & 0xff) << 8);
        Color::from_rgbi(val)
    }

    /// Get a color from an RGBI value, the I stands for the Fltk colormap index.
    pub const fn from_rgbi(val: u32) -> Color {
        let mut c = Color::Black;
        c.bits = val;
        c
    }

    /// Create color from RGBA using alpha compositing. Works for non-group types.
    pub fn from_rgba_tuple(tup: (u8, u8, u8, u8)) -> Color {
        if tup.3 != 255 {
            let bg_col = if let Some(grp) = crate::group::Group::try_current() {
                use crate::prelude::WidgetExt;
                grp.color()
            } else {
                Color::BackGround
            };
            let bg_col = bg_col.to_rgb();
            let alpha = tup.3 as f32 / 255.0;
            let r = alpha * tup.0 as f32 + (1.0 - alpha) * bg_col.0 as f32;
            let r = r as u8;
            let g = alpha * tup.1 as f32 + (1.0 - alpha) * bg_col.1 as f32;
            let g = g as u8;
            let b = alpha * tup.2 as f32 + (1.0 - alpha) * bg_col.2 as f32;
            let b = b as u8;
            Color::from_rgb(r, g, b)
        } else {
            Color::from_rgb(tup.0, tup.1, tup.2)
        }
    }

    /// Returns a color from hex or decimal
    pub const fn from_u32(val: u32) -> Color {
        let (r, g, b) = utils::hex2rgb(val);
        Color::from_rgb(r, g, b)
    }

    /// Returns a color from hex or decimal
    pub const fn from_hex(val: u32) -> Color {
        let (r, g, b) = utils::hex2rgb(val);
        Color::from_rgb(r, g, b)
    }

    /// Return a Color from a hex color format (`#xxxxxx`)
    pub fn from_hex_str(col: &str) -> Result<Color, FltkError> {
        if !col.starts_with('#') || col.len() != 7 {
            Err(FltkError::Internal(FltkErrorKind::InvalidColor))
        } else {
            Ok(Color::from_hex(u32::from_str_radix(&col[1..7], 16)?))
        }
    }

    /// Returns the color in hex string format
    pub fn to_hex_str(&self) -> String {
        let (r, g, b) = self.to_rgb();
        format!("#{:02x}{:02x}{:02x}", r, g, b)
    }

    /// Returns a color by index of RGBI
    pub fn by_index(idx: u8) -> Color {
        unsafe { mem::transmute(idx as u32) }
    }

    /// Returns an inactive form of the color
    pub fn inactive(&self) -> Color {
        unsafe { mem::transmute(fl::Fl_inactive(self.bits)) }
    }

    /// Returns an darker form of the color
    pub fn darker(&self) -> Color {
        unsafe { mem::transmute(fl::Fl_darker(self.bits)) }
    }

    /// Returns an lighter form of the color
    pub fn lighter(&self) -> Color {
        unsafe { mem::transmute(fl::Fl_lighter(self.bits)) }
    }

    /// Returns a gray color value from black (i == 0) to white (i == FL_NUM_GRAY - 1)
    pub fn gray_ramp(val: i32) -> Color {
        unsafe { mem::transmute(fl::Fl_gray_ramp(val)) }
    }

    /// Returns a gray color value from black (i == 0) to white (i == FL_NUM_GRAY - 1)
    pub fn color_average(c1: Color, c2: Color, weight: f32) -> Color {
        unsafe { mem::transmute(fl::Fl_color_average(c1.bits, c2.bits, weight)) }
    }

    /// Returns a color that contrasts with the background color.
    pub fn contrast(fg: Color, bg: Color) -> Color {
        unsafe { mem::transmute(fl::Fl_contrast(fg.bits, bg.bits)) }
    }

    /// Returns the color closest to the passed grayscale value
    pub fn gray_scale(g: u8) -> Color {
        unsafe { mem::transmute(fl::Fl_rgb_color2(g)) }
    }

    /// Returns the color closest to the passed rgb value
    pub fn rgb_color(r: u8, g: u8, b: u8) -> Color {
        unsafe { mem::transmute(fl::Fl_rgb_color(r, g, b)) }
    }

    /// Get the RGB value of the color
    pub fn to_rgb(&self) -> (u8, u8, u8) {
        unsafe {
            let val = self.bits;
            let r = ((val >> 24) & 0xff) as u8;
            let g = ((val >> 16) & 0xff) as u8;
            let b = ((val >> 8) & 0xff) as u8;
            let i = (val & 0xff) as u8;
            if i == 0 {
                (r, g, b)
            } else {
                let val = fl::Fl_cmap(val);
                let r = ((val >> 24) & 0xff) as u8;
                let g = ((val >> 16) & 0xff) as u8;
                let b = ((val >> 8) & 0xff) as u8;
                (r, g, b)
            }
        }
    }
}

#[allow(unreachable_patterns)]
impl std::fmt::Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Color::ForeGround => write!(f, "Color::ForeGround"),
            Color::BackGround => write!(f, "Color::BackGround"),
            Color::BackGround2 => write!(f, "Color::BackGround2"),
            Color::Foreground => write!(f, "Color::Foreground"),
            Color::Background => write!(f, "Color::Background"),
            Color::Background2 => write!(f, "Color::Background2"),
            Color::Inactive => write!(f, "Color::Inactive"),
            Color::Selection => write!(f, "Color::Selection"),
            Color::Gray0 => write!(f, "Color::Gray0"),
            Color::Dark3 => write!(f, "Color::Dark3"),
            Color::Dark2 => write!(f, "Color::Dark2"),
            Color::Dark1 => write!(f, "Color::Dark1"),
            Color::FrameDefault => write!(f, "Color::FrameDefault"),
            Color::Light1 => write!(f, "Color::Light1"),
            Color::Light2 => write!(f, "Color::Light2"),
            Color::Light3 => write!(f, "Color::Light3"),
            Color::Black => write!(f, "Color::Black"),
            Color::Red => write!(f, "Color::Red"),
            Color::Green => write!(f, "Color::Green"),
            Color::Yellow => write!(f, "Color::Yellow"),
            Color::Blue => write!(f, "Color::Blue"),
            Color::Magenta => write!(f, "Color::Magenta"),
            Color::Cyan => write!(f, "Color::Cyan"),
            Color::DarkRed => write!(f, "Color::DarkRed"),
            Color::DarkGreen => write!(f, "Color::DarkGreen"),
            Color::DarkYellow => write!(f, "Color::DarkYellow"),
            Color::DarkBlue => write!(f, "Color::DarkBlue"),
            Color::DarkMagenta => write!(f, "Color::DarkMagenta"),
            Color::DarkCyan => write!(f, "Color::DarkCyan"),
            Color::White => write!(f, "Color::White"),
            _ => {
                let temp = format!("{:08x}", self.bits);
                write!(f, "Color::from_hex(0x{})", &temp[0..6])
            }
        }
    }
}

bitflags::bitflags! {
    /// Defines event types captured by FLTK
    pub struct Event: i32 {
        /// No Event
        const NoEvent = 0;
        /// Push
        const Push = 1;
        /// Released
        const Released = 2;
        /// Enter
        const Enter = 3;
        /// Leave
        const Leave = 4;
        /// Drag
        const Drag = 5;
        /// Focus
        const Focus = 6;
        /// Unfocus
        const Unfocus = 7;
        /// KeyDown
        const KeyDown = 8;
        /// KeyUp
        const KeyUp = 9;
        /// Close
        const Close = 10;
        /// Move
        const Move = 11;
        /// Shortcut
        const Shortcut = 12;
        /// Deactivate
        const Deactivate = 13;
        /// Activate
        const Activate = 14;
        /// Hide
        const Hide = 15;
        /// Show
        const Show = 16;
        /// Paste
        const Paste = 17;
        /// Selection Clear
        const SelectionClear = 18;
        /// MouseWheel
        const MouseWheel = 19;
        /// DndEnter
        const DndEnter = 20;
        /// Drag n Drop: Drag
        const DndDrag = 21;
        /// Drag n Drop: Leave
        const DndLeave = 22;
        /// Drag n Drop: Release
        const DndRelease = 23;
        /// Screen Config Changed
        const ScreenConfigChanged = 24;
        /// Fullscreen
        const Fullscreen = 25;
        /// Zoom Gesture
        const ZoomGesture = 26;
        /// Zoom Event
        const ZoomEvent = 27;
        /// Window Resize Event.
        /// Avoid resizing the parent during a resize event
        /// to avoid infinite recursion
        const Resize = 28;
    }
}

impl Event {
    /// Creates an event from an i32 value
    pub const fn from_i32(val: i32) -> Event {
        let mut ev = Event::NoEvent;
        ev.bits = val;
        ev
    }
}

impl From<i32> for Event {
    fn from(val: i32) -> Event {
        Event::from_i32(val)
    }
}

#[allow(unreachable_patterns)]
impl std::fmt::Display for Event {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Event::NoEvent => write!(f, "Event::NoEvent"),
            Event::Push => write!(f, "Event::Push"),
            Event::Released => write!(f, "Event::Released"),
            Event::Enter => write!(f, "Event::Enter"),
            Event::Leave => write!(f, "Event::Leave"),
            Event::Drag => write!(f, "Event::Drag"),
            Event::Focus => write!(f, "Event::Focus"),
            Event::Unfocus => write!(f, "Event::Unfocus"),
            Event::KeyDown => write!(f, "Event::KeyDown"),
            Event::KeyUp => write!(f, "Event::KeyUp"),
            Event::Close => write!(f, "Event::Close"),
            Event::Move => write!(f, "Event::Move"),
            Event::Shortcut => write!(f, "Event::Shortcut"),
            Event::Deactivate => write!(f, "Event::Deactivate"),
            Event::Activate => write!(f, "Event::Activate"),
            Event::Hide => write!(f, "Event::Hide"),
            Event::Show => write!(f, "Event::Show"),
            Event::Paste => write!(f, "Event::Paste"),
            Event::SelectionClear => write!(f, "Event::SelectionClear"),
            Event::MouseWheel => write!(f, "Event::MouseWheel"),
            Event::DndEnter => write!(f, "Event::DndEnter"),
            Event::DndDrag => write!(f, "Event::DndDrag"),
            Event::DndLeave => write!(f, "Event::DndLeave"),
            Event::DndRelease => write!(f, "Event::DndRelease"),
            Event::ScreenConfigChanged => write!(f, "Event::ScreenConfigChanged"),
            Event::Fullscreen => write!(f, "Event::Fullscreen"),
            Event::ZoomGesture => write!(f, "Event::ZoomGesture"),
            Event::ZoomEvent => write!(f, "Event::ZoomEvent"),
            Event::Resize => write!(f, "Event::Resize"),
            _ => {
                write!(f, "Event::from_i32({})", self.bits)
            }
        }
    }
}

bitflags::bitflags! {
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
        /// F1
        const F1 = 0xffbd + 1;
        /// F2
        const F2 = 0xffbd + 2;
        /// F3
        const F3 = 0xffbd + 3;
        /// F4
        const F4 = 0xffbd + 4;
        /// F5
        const F5 = 0xffbd + 5;
        /// F6
        const F6 = 0xffbd + 6;
        /// F7
        const F7 = 0xffbd + 7;
        /// F8
        const F8 = 0xffbd + 8;
        /// F9
        const F9 = 0xffbd + 9;
        /// F10
        const F10 = 0xffbd + 10;
        /// F11
        const F11 = 0xffbd + 11;
        /// F12
        const F12 = 0xffbd + 12;
        /// Function key last
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
    pub const fn from_i32(val: i32) -> Key {
        let mut k = Key::None;
        k.bits = val;
        k
    }

    /// Gets a Key from a char
    pub const fn from_char(val: char) -> Key {
        let mut k = Key::None;
        k.bits = val as i32;
        k
    }

    /// Get the char representation of a Key.
    pub const fn to_char(&self) -> Option<char> {
        let bits = self.bits;
        if bits >= 0xD800 && bits <= 0xDFFF {
            None
        } else {
            Some(bits as u8 as char)
        }
    }

    /// Returns whether a key is a function key
    pub const fn is_fn_key(key: Key) -> bool {
        key.bits() >= Key::F1.bits() && key.bits() < Key::FLast.bits()
    }

    /// Return the corresponding function key
    pub const fn fn_key(val: i32) -> Key {
        Key::from_i32(Key::F1.bits() - 1 + val)
    }
}

bitflags::bitflags! {
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
    pub const fn from_char(c: char) -> Shortcut {
        let mut s = Shortcut::None;
        s.bits = c as _;
        s
    }

    /// Create a shortcut from a key
    pub const fn from_key(k: Key) -> Shortcut {
        let mut s = Shortcut::None;
        s.bits = k.bits();
        s
    }

    /// Create a shortcut from an i32
    pub const fn from_i32(v: i32) -> Shortcut {
        let mut s = Shortcut::None;
        s.bits = v;
        s
    }

    /// get key mask
    pub const fn key(&self) -> Key {
        let mut temp = self.bits;
        temp &= 0x0000_ffff;
        Key::from_i32(temp)
    }

    /// Get the button number
    pub const fn button(button_num: i32) -> Shortcut {
        let mut s = Shortcut::None;
        s.bits = 0x0080_0000 << button_num;
        s
    }
}

bitflags::bitflags! {
    /// Defines the types of triggers for widget callback functions. Equivalent to FL_WHEN
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
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
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

bitflags::bitflags! {
    /// Defines visual mode types (capabilities of the window).
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
        /// MultiSample anti-aliasing
        const MultiSample = 128;
        /// Stereoscopic rendering
        const Stereo = 256;
        /// Fake single buffered windows using double-buffer
        const FakeSingle = 512; //
        /// Use OpenGL version 3.0 or more
        const Opengl3 = 1024;
    }
}

bitflags::bitflags! {
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
        unsafe { mem::transmute(self.bits as i32 | other as i32) }
    }
}

impl std::ops::BitOr<Key> for Shortcut {
    type Output = Shortcut;
    fn bitor(self, other: Key) -> Self::Output {
        unsafe { mem::transmute(self.bits as i32 | other.bits() as i32) }
    }
}

impl std::ops::BitOr<i32> for Align {
    type Output = Align;
    fn bitor(self, rhs: i32) -> Self::Output {
        unsafe { mem::transmute(self.bits | rhs as i32) }
    }
}
