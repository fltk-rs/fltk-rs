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

#[doc(hidden)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct UnmappedFrameType {
    bits: i32,
}

impl UnmappedFrameType {
    #[doc(hidden)]
    pub const unsafe fn from_i32(val: i32) -> Self {
        Self { bits: val }
    }
}

/// Defines the frame types which can be set using the `set_frame()` and `set_down_frame()` methods
#[repr(i32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum FrameType {
    /// No Box
    NoBox,
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
    /// User-defined frame types
    UserFrameType(UnmappedFrameType),
}

impl FrameType {
    /// Alias OFlatFrame as OFlatBox
    pub const OFlatBox: FrameType = FrameType::OFlatFrame;
    /// Alias GtkRoundDownFrame as GtkRoundDownBox
    pub const GtkRoundDownBox: FrameType = FrameType::GtkRoundDownFrame;
    /// Get the discriminant value or the user defined frame type
    pub fn as_i32(&self) -> i32 {
        match *self {
            FrameType::UserFrameType(v) => v.bits,
            _ => self.discriminant(),
        }
    }
    /// Construct a FrameType from an i32 value
    /// # Safety
    /// The frametype should be defined using the `app::set_frame_type_cb` function
    #[doc(hidden)]
    pub unsafe fn from_i32(v: i32) -> FrameType {
        if (0..=56).contains(&v) {
            *(&v as *const i32 as *const FrameType)
        } else {
            FrameType::UserFrameType(UnmappedFrameType::from_i32(v))
        }
    }
    #[doc(hidden)]
    fn discriminant(&self) -> i32 {
        unsafe { *(self as *const Self as *const i32) }
    }
    /// Gets the Frame type by index
    pub fn by_index(idx: usize) -> FrameType {
        let idx = if idx > 56 { 56 } else { idx };
        unsafe { FrameType::from_i32(idx as i32) }
    }

    /// Get the frame's x offset
    pub fn dx(self) -> i32 {
        unsafe { fl::Fl_box_dx(self.as_i32()) }
    }

    /// Get the frame's y offset
    pub fn dy(self) -> i32 {
        unsafe { fl::Fl_box_dy(self.as_i32()) }
    }

    /// Get the frame's width offset
    pub fn dw(self) -> i32 {
        unsafe { fl::Fl_box_dw(self.as_i32()) }
    }

    /// Get the frame's height offset
    pub fn dh(self) -> i32 {
        unsafe { fl::Fl_box_dh(self.as_i32()) }
    }

    /// Swap frames
    pub fn swap_frames(old_frame: FrameType, new_frame: FrameType) {
        unsafe {
            let new_frame = new_frame.as_i32();
            let old_frame = old_frame.as_i32();
            fl::Fl_set_box_type(old_frame, new_frame);
        }
    }
}

bitflags::bitflags! {
    /// Defines alignment rules used by FLTK for labels
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
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

/// Defines fonts used by FLTK
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Font {
    bits: i32,
}

impl Font {
    /// Helvetica
    pub const Helvetica: Font = Font { bits: 0 };
    /// Helvetica Bold
    pub const HelveticaBold: Font = Font { bits: 1 };
    /// Helvetica Italic
    pub const HelveticaItalic: Font = Font { bits: 2 };
    /// Helvetica Bold Italic
    pub const HelveticaBoldItalic: Font = Font { bits: 3 };
    /// Courier
    pub const Courier: Font = Font { bits: 4 };
    /// Courier Bold
    pub const CourierBold: Font = Font { bits: 5 };
    /// Courier Italic
    pub const CourierItalic: Font = Font { bits: 6 };
    /// Courier Bold Italic
    pub const CourierBoldItalic: Font = Font { bits: 7 };
    /// Times
    pub const Times: Font = Font { bits: 8 };
    /// Times Bold
    pub const TimesBold: Font = Font { bits: 9 };
    /// Times Italic
    pub const TimesItalic: Font = Font { bits: 10 };
    /// Times Bold Italic
    pub const TimesBoldItalic: Font = Font { bits: 11 };
    /// Symbol
    pub const Symbol: Font = Font { bits: 12 };
    /// Screen
    pub const Screen: Font = Font { bits: 13 };
    /// Screen Bold
    pub const ScreenBold: Font = Font { bits: 14 };
    /// Zapfdingbats
    pub const Zapfdingbats: Font = Font { bits: 15 };
    /// Gets the inner value of the Font
    pub const fn bits(&self) -> i32 {
        self.bits
    }
    /// Returns a font by index. This is the enum representation of the Font. If you change the default font for your app,
    /// which by default is Helvetica, `Font::by_index(0)` will still show Helvetica!
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
            fl::Fl_set_font2(old.bits(), new.into_raw() as _);
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
            CStr::from_ptr(fl::Fl_get_font_name(self.bits()))
                .to_string_lossy()
                .to_string()
        }
    }
}


// todo: implement Display for Font ?
#[allow(unreachable_patterns)]
impl std::fmt::Debug for Font {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Font::Helvetica => write!(f, "Font::Helvetica"),
            Font::HelveticaBold => write!(f, "Font::HelveticaBold"),
            Font::HelveticaItalic => write!(f, "Font::HelveticaItalic"),
            Font::HelveticaBoldItalic => write!(f, "Font::HelveticaBoldItalic"),
            Font::Courier => write!(f, "Font::Courier"),
            Font::CourierBold => write!(f, "Font::CourierBold"),
            Font::CourierItalic => write!(f, "Font::CourierItalic"),
            Font::CourierBoldItalic => write!(f, "Font::CourierBoldItalic"),
            Font::Times => write!(f, "Font::Times"),
            Font::TimesBold => write!(f, "Font::TimesBold"),
            Font::TimesItalic => write!(f, "Font::TimesItalic"),
            Font::TimesBoldItalic => write!(f, "Font::TimesBoldItalic"),
            Font::Symbol => write!(f, "Font::Symbol"),
            Font::Screen => write!(f, "Font::Screen"),
            Font::ScreenBold => write!(f, "Font::ScreenBold"),
            Font::Zapfdingbats => write!(f, "Font::Zapfdingbats"),
            _ => {
                write!(f, "Font::from_i32({})", self.bits())
            }
        }
    }
}

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

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Color {
    bits: u32,
}

impl Color {
    /// ForeGround, label colors
    pub const ForeGround: Color = Color { bits: 0 };
    /// Foreground, label colors
    pub const Foreground: Color = Color { bits: 0 };
    /// BackGround2, Is the color inside input, output and text display widgets
    pub const BackGround2: Color = Color { bits: 7 };
    /// Background2, Is the color inside input, output and text display widgets
    pub const Background2: Color = Color { bits: 7 };
    /// Inactive
    pub const Inactive: Color = Color { bits: 8 };
    /// Selection
    pub const Selection: Color = Color { bits: 15 };
    /// Free
    pub const Free: Color = Color { bits: 16 };
    /// Gray0
    pub const Gray0: Color = Color { bits: 32 };
    /// GrayRamp
    pub const GrayRamp: Color = Color { bits: 32 };
    /// Dark3
    pub const Dark3: Color = Color { bits: 39 };
    /// Dark2
    pub const Dark2: Color = Color { bits: 45 };
    /// Dark1
    pub const Dark1: Color = Color { bits: 47 };
    /// FrameDefault
    pub const FrameDefault: Color = Color { bits: 49 };
    /// BackGround
    pub const BackGround: Color = Color { bits: 49 };
    /// Background
    pub const Background: Color = Color { bits: 49 };
    /// Light1
    pub const Light1: Color = Color { bits: 50 };
    /// Light2
    pub const Light2: Color = Color { bits: 52 };
    /// Light3
    pub const Light3: Color = Color { bits: 54 };
    /// Black
    pub const Black: Color = Color { bits: 56 };
    /// Red
    pub const Red: Color = Color { bits: 88 };
    /// Green
    pub const Green: Color = Color { bits: 63 };
    /// Yellow
    pub const Yellow: Color = Color { bits: 95 };
    /// Blue
    pub const Blue: Color = Color { bits: 216 };
    /// Magenta
    pub const Magenta: Color = Color { bits: 248 };
    /// Cyan
    pub const Cyan: Color = Color { bits: 223 };
    /// DarkRed
    pub const DarkRed: Color = Color { bits: 72 };
    /// DarkGreen
    pub const DarkGreen: Color = Color { bits: 60 };
    /// DarkYellow
    pub const DarkYellow: Color = Color { bits: 76 };
    /// DarkBlue
    pub const DarkBlue: Color = Color { bits: 136 };
    /// DarkMagenta
    pub const DarkMagenta: Color = Color { bits: 152 };
    /// DarkCyan
    pub const DarkCyan: Color = Color { bits: 140 };
    /// White
    pub const White: Color = Color { bits: 255 };

    /// ANSI/xterm Black
    pub const XtermBlack: Color = Color { bits: 0x00000000 };
    /// ANSI/xterm Red
    pub const XtermRed: Color = Color { bits: 0xd0000000 };
    /// ANSI/xterm Green
    pub const XtermGreen: Color = Color { bits: 0x00d00000 };
    /// ANSI/xterm Yellow
    pub const XtermYellow: Color = Color { bits: 0xd0d00000 };
    /// ANSI/xterm Blue
    pub const XtermBlue: Color = Color { bits: 0x0000d000 };
    /// ANSI/xterm Magenta
    pub const XtermMagenta: Color = Color { bits: 0xd000d000 };
    /// ANSI/xterm Cyan
    pub const XtermCyan: Color = Color { bits: 0x00d0d000 };
    /// ANSI/xterm White
    pub const XtermWhite: Color = Color { bits: 0xd0d0d000 };
    /// ANSI/xterm background Red
    pub const XtermBgRed: Color = Color { bits: 0xc0000000 };
    /// ANSI/xterm background Green
    pub const XtermBgGreen: Color = Color { bits: 0x00c00000 };
    /// ANSI/xterm background Yelllow
    pub const XtermBgYellow: Color = Color { bits: 0xc0c00000 };
    /// ANSI/xterm background Blue
    pub const XtermBgBlue: Color = Color { bits: 0x0000c000 };
    /// ANSI/xterm background Magenta
    pub const XtermBgMagenta: Color = Color { bits: 0xd000c000 };
    /// ANSI/xterm background Cyan
    pub const XtermBgCyan: Color = Color { bits: 0x00c0c000 };
    /// ANSI/xterm background White
    pub const XtermBgWhite: Color = Color { bits: 0xc0c0c000 };

    /// Special background color value that lets the Terminal widget's box() color show through behind the text.
    pub const TransparentBg: Color = Color { bits: 0xffffffff };

    /// Gets the inner color representation
    pub const fn bits(&self) -> u32 {
        self.bits
    }
    /// Returns a color from RGB
    pub const fn from_rgb(r: u8, g: u8, b: u8) -> Color {
        let r = r as u32;
        let g = g as u32;
        let b = b as u32;
        let val: u32 = ((r & 0xff) << 24) + ((g & 0xff) << 16) + ((b & 0xff) << 8);
        Color::from_rgbi(val)
    }

    #[cfg(feature = "enable-glwindow")]
    /// Returns a color from RGB
    pub const fn from_rgba(r: u8, g: u8, b: u8, a: u8) -> Color {
        let r = r as u32;
        let g = g as u32;
        let b = b as u32;
        let a = a as u32;
        let val: u32 = ((r & 0xff) << 24) + ((g & 0xff) << 16) + ((b & 0xff) << 8) + (a & 0xff);
        Color::from_rgbi(val)
    }

    /// Returns a color enum from RGBI encoding
    pub const fn from_rgbi(rgbi: u32) -> Color {
        match rgbi {
            // These are all RGB aliases for the defined colors.
            0x00000000 => Color::Black,
            0xff000000 => Color::Red,
            0x00ff0000 => Color::Green,
            0xffff0000 => Color::Yellow,
            0x0000ff00 => Color::Blue,
            0xff00ff00 => Color::Magenta,
            0x00ffff00 => Color::Cyan,
            0x7f000000 => Color::DarkRed,
            0x00910000 => Color::DarkGreen,
            0x7f910000 => Color::DarkYellow,
            0x00007f00 => Color::DarkBlue,
            0x7f007f00 => Color::DarkMagenta,
            0x00917f00 => Color::DarkCyan,
            0xffffff00 => Color::White, // This RGB is also called Gray0
            0xaaaaaa00 => Color::Dark1,
            0x95959500 => Color::Dark2,
            0x55555500 => Color::Dark3,
            0xcbcbcb00 => Color::Light1,
            0xe0e0e000 => Color::Light2,
            0xf5f5f500 => Color::Light3,

            0xd0000000 => Color::XtermRed,
            0x00d00000 => Color::XtermGreen,
            0xd0d00000 => Color::XtermYellow,
            0x0000d000 => Color::XtermBlue,
            0xd000d000 => Color::XtermMagenta,
            0x00d0d000 => Color::XtermCyan,
            0xd0d0d000 => Color::XtermWhite,
            0xc0000000 => Color::XtermBgRed,
            0x00c00000 => Color::XtermBgGreen,
            0xc0c00000 => Color::XtermBgYellow,
            0x0000c000 => Color::XtermBgBlue,
            0xc000c000 => Color::XtermBgMagenta,
            0x00c0c000 => Color::XtermBgCyan,
            0xc0c0c000 => Color::XtermBgWhite,
            // ? todo: define Xterm dim and bold modified colors?

            0xffffffff => Color::TransparentBg, // Special value that lets the widget's box() color show through

            _ => Color { bits: rgbi }, // Just use the provided value
        }
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
        #[cfg(feature = "enable-glwindow")]
        {
            let (r, g, b, a) = utils::hex2rgba(val);
            Color::from_rgba(r, g, b, a)
        }
        #[cfg(not(feature = "enable-glwindow"))]
        {
            let (r, g, b) = utils::hex2rgb(val);
            Color::from_rgb(r, g, b)
        }
    }

    /// Returns a color from hex or decimal
    pub const fn from_hex(val: u32) -> Color {
        Color::from_u32(val)
    }

    /// Return a Color from a hex color format (`#xxxxxx`)
    pub fn from_hex_str(col: &str) -> Result<Color, FltkError> {
        if !col.starts_with('#') || col.len() < 7 {
            return Err(FltkError::Internal(FltkErrorKind::InvalidColor));
        }
        let color: Color;
        #[cfg(not(feature = "enable-glwindow"))]
        {
            color = Color::from_hex(u32::from_str_radix(&col[1..7], 16)?);
        }

        #[cfg(feature = "enable-glwindow")]
        {
            color = Color::from_hex(u32::from_str_radix(&col[1..9], 16)?);
        }
        Ok(color)
    }

    /// Returns the color in hex string format
    pub fn to_hex_str(&self) -> String {
        #[cfg(not(feature = "enable-glwindow"))]
        {
            let (r, g, b) = self.to_rgb();
            format!("#{:02x}{:02x}{:02x}", r, g, b)
        }
        #[cfg(feature = "enable-glwindow")]
        {
            let (r, g, b, a) = self.to_rgba();
            format!("#{:02x}{:02x}{:02x}{:02x}", r, g, b, a)
        }
    }

    /// Returns a color by index of RGBI
    pub fn by_index(idx: u8) -> Color {
        unsafe { mem::transmute(idx as u32) }
    }

    /// Returns an inactive form of the color
    pub fn inactive(&self) -> Color {
        unsafe { mem::transmute(fl::Fl_inactive(self.bits())) }
    }

    /// Returns an darker form of the color
    pub fn darker(&self) -> Color {
        unsafe { mem::transmute(fl::Fl_darker(self.bits())) }
    }

    /// Returns an lighter form of the color
    pub fn lighter(&self) -> Color {
        unsafe { mem::transmute(fl::Fl_lighter(self.bits())) }
    }

    /// Returns a gray color value from black (i == 0) to white (i == FL_NUM_GRAY - 1)
    pub fn gray_ramp(val: i32) -> Color {
        unsafe { mem::transmute(fl::Fl_gray_ramp(val)) }
    }

    /// Returns a gray color value from black (i == 0) to white (i == FL_NUM_GRAY - 1)
    pub fn color_average(c1: Color, c2: Color, weight: f32) -> Color {
        unsafe { mem::transmute(fl::Fl_color_average(c1.bits(), c2.bits(), weight)) }
    }

    /// Returns a color that contrasts with the background color.
    pub fn contrast(fg: Color, bg: Color) -> Color {
        unsafe { mem::transmute(fl::Fl_contrast(fg.bits(), bg.bits())) }
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
            let val = self.bits();
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

    #[cfg(feature = "enable-glwindow")]
    /// Get the RGBA value of the color
    pub fn to_rgba(&self) -> (u8, u8, u8, u8) {
        let val = self.bits();
        let r = ((val >> 24) & 0xff) as u8;
        let g = ((val >> 16) & 0xff) as u8;
        let b = ((val >> 8) & 0xff) as u8;
        let a = (val & 0xff) as u8;
        (r, g, b, a)
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

            Color::XtermRed => write!(f, "Color::XtermRed"),
            Color::XtermGreen => write!(f, "Color::XtermGreen"),
            Color::XtermYellow => write!(f, "Color::XtermYellow"),
            Color::XtermBlue => write!(f, "Color::XtermBlue"),
            Color::XtermMagenta => write!(f, "Color::XtermMagenta"),
            Color::XtermCyan => write!(f, "Color::XtermCyan"),
            Color::XtermWhite => write!(f, "Color::XtermWhite"),
            Color::XtermBgRed => write!(f, "Color::XtermBgRed"),
            Color::XtermBgGreen => write!(f, "Color::XtermBgGreen"),
            Color::XtermBgYellow => write!(f, "Color::XtermBgYellow"),
            Color::XtermBgBlue => write!(f, "Color::XtermBgBlue"),
            Color::XtermBgMagenta => write!(f, "Color::XtermBgMagenta"),
            Color::XtermBgCyan => write!(f, "Color::XtermBgCyan"),
            Color::XtermBgWhite => write!(f, "Color::XtermBgWhite"),

            Color::TransparentBg => write!(f, "Color::TransparentBg"),
            _ => {
                let temp = format!("{:08x}", self.bits());
                write!(f, "Color::from_hex(0x{}_{})", &temp[0..6], &temp[6..8])
            }
        }
    }
}

#[allow(unreachable_patterns)]
impl std::fmt::Debug for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", *self)
    }
}

/// Defines event types captured by FLTK
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Event {
    bits: i32,
}

impl Event {
    /// No Event
    pub const NoEvent: Event = Event { bits: 0 };
    /// Push
    pub const Push: Event = Event { bits: 1 };
    /// Released
    pub const Released: Event = Event { bits: 2 };
    /// Enter
    pub const Enter: Event = Event { bits: 3 };
    /// Leave
    pub const Leave: Event = Event { bits: 4 };
    /// Drag
    pub const Drag: Event = Event { bits: 5 };
    /// Focus
    pub const Focus: Event = Event { bits: 6 };
    /// Unfocus
    pub const Unfocus: Event = Event { bits: 7 };
    /// KeyDown
    pub const KeyDown: Event = Event { bits: 8 };
    /// KeyUp
    pub const KeyUp: Event = Event { bits: 9 };
    /// Close
    pub const Close: Event = Event { bits: 10 };
    /// Move
    pub const Move: Event = Event { bits: 11 };
    /// Shortcut
    pub const Shortcut: Event = Event { bits: 12 };
    /// Deactivate
    pub const Deactivate: Event = Event { bits: 13 };
    /// Activate
    pub const Activate: Event = Event { bits: 14 };
    /// Hide
    pub const Hide: Event = Event { bits: 15 };
    /// Show
    pub const Show: Event = Event { bits: 16 };
    /// Paste
    pub const Paste: Event = Event { bits: 17 };
    /// Selection Clear
    pub const SelectionClear: Event = Event { bits: 18 };
    /// MouseWheel
    pub const MouseWheel: Event = Event { bits: 19 };
    /// DndEnter
    pub const DndEnter: Event = Event { bits: 20 };
    /// Drag n Drop: Drag
    pub const DndDrag: Event = Event { bits: 21 };
    /// Drag n Drop: Leave
    pub const DndLeave: Event = Event { bits: 22 };
    /// Drag n Drop: Release
    pub const DndRelease: Event = Event { bits: 23 };
    /// Screen Config Changed
    pub const ScreenConfigChanged: Event = Event { bits: 24 };
    /// Fullscreen
    pub const Fullscreen: Event = Event { bits: 25 };
    /// Zoom Gesture
    pub const ZoomGesture: Event = Event { bits: 26 };
    /// Zoom Event
    pub const ZoomEvent: Event = Event { bits: 27 };
    /// Window Resize Event.
    /// Avoid resizing the parent during a resize event
    /// to avoid infinite recursion
    pub const Resize: Event = Event { bits: 28 };
    /// Gets the inner value
    pub const fn bits(&self) -> i32 {
        self.bits
    }
    /// Creates an event from an i32 value
    pub const fn from_i32(val: i32) -> Event {
        Event { bits: val }
    }
}

impl From<i32> for Event {
    fn from(val: i32) -> Event {
        Event::from_i32(val)
    }
}

#[allow(unreachable_patterns)]
impl std::fmt::Debug for Event {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", *self)
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
                write!(f, "Event::from_i32({})", self.bits())
            }
        }
    }
}

/// Defines the inputted virtual keycode
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Key {
    bits: i32,
}

impl Key {
    /// None
    pub const None: Key = Key { bits: 0 };
    /// Button
    pub const Button: Key = Key { bits: 0xfee8 };
    /// BackSpace
    pub const BackSpace: Key = Key { bits: 0xff08 };
    /// Tab
    pub const Tab: Key = Key { bits: 0xff09 };
    /// IsoKey
    pub const IsoKey: Key = Key { bits: 0xff0c };
    /// Enter
    pub const Enter: Key = Key { bits: 0xff0d };
    /// Pause
    pub const Pause: Key = Key { bits: 0xff13 };
    /// ScrollLock
    pub const ScrollLock: Key = Key { bits: 0xff14 };
    /// Escape
    pub const Escape: Key = Key { bits: 0xff1b };
    /// Kana
    pub const Kana: Key = Key { bits: 0xff2e };
    /// Eisu
    pub const Eisu: Key = Key { bits: 0xff2f };
    /// Yen
    pub const Yen: Key = Key { bits: 0xff30 };
    /// JISUnderscore
    pub const JISUnderscore: Key = Key { bits: 0xff31 };
    /// Home
    pub const Home: Key = Key { bits: 0xff50 };
    /// Left
    pub const Left: Key = Key { bits: 0xff51 };
    /// Up
    pub const Up: Key = Key { bits: 0xff52 };
    /// Right
    pub const Right: Key = Key { bits: 0xff53 };
    /// Down
    pub const Down: Key = Key { bits: 0xff54 };
    /// PageUp
    pub const PageUp: Key = Key { bits: 0xff55 };
    /// PageDown
    pub const PageDown: Key = Key { bits: 0xff56 };
    /// End
    pub const End: Key = Key { bits: 0xff57 };
    /// Print
    pub const Print: Key = Key { bits: 0xff61 };
    /// Insert
    pub const Insert: Key = Key { bits: 0xff63 };
    /// Menu
    pub const Menu: Key = Key { bits: 0xff67 };
    /// Help
    pub const Help: Key = Key { bits: 0xff68 };
    /// NumLock
    pub const NumLock: Key = Key { bits: 0xff7f };
    /// Keypad
    pub const KP: Key = Key { bits: 0xff80 };
    /// Keypad Enter
    pub const KPEnter: Key = Key { bits: 0xff8d };
    /// Keypad Last
    pub const KPLast: Key = Key { bits: 0xffbd };
    /// F1
    pub const F1: Key = Key { bits: 0xffbd + 1 };
    /// F2
    pub const F2: Key = Key { bits: 0xffbd + 2 };
    /// F3
    pub const F3: Key = Key { bits: 0xffbd + 3 };
    /// F4
    pub const F4: Key = Key { bits: 0xffbd + 4 };
    /// F5
    pub const F5: Key = Key { bits: 0xffbd + 5 };
    /// F6
    pub const F6: Key = Key { bits: 0xffbd + 6 };
    /// F7
    pub const F7: Key = Key { bits: 0xffbd + 7 };
    /// F8
    pub const F8: Key = Key { bits: 0xffbd + 8 };
    /// F9
    pub const F9: Key = Key { bits: 0xffbd + 9 };
    /// F10
    pub const F10: Key = Key { bits: 0xffbd + 10 };
    /// F11
    pub const F11: Key = Key { bits: 0xffbd + 11 };
    /// F12
    pub const F12: Key = Key { bits: 0xffbd + 12 };
    /// Function key last
    pub const FLast: Key = Key { bits: 0xffe0 };
    /// Shift Left
    pub const ShiftL: Key = Key { bits: 0xffe1 };
    /// Shift Right
    pub const ShiftR: Key = Key { bits: 0xffe2 };
    /// Control Left
    pub const ControlL: Key = Key { bits: 0xffe3 };
    /// Control Right
    pub const ControlR: Key = Key { bits: 0xffe4 };
    /// Caps Lock
    pub const CapsLock: Key = Key { bits: 0xffe5 };
    /// Meta Left
    pub const MetaL: Key = Key { bits: 0xffe7 };
    /// Meta Right
    pub const MetaR: Key = Key { bits: 0xffe8 };
    /// Alt Left
    pub const AltL: Key = Key { bits: 0xffe9 };
    /// Alt Right
    pub const AltR: Key = Key { bits: 0xffea };
    /// Delete
    pub const Delete: Key = Key { bits: 0xffff };
    /// Gets the i32 value of a Key
    pub const fn bits(&self) -> i32 {
        self.bits
    }
    /// Gets a Key from an i32
    pub const fn from_i32(val: i32) -> Key {
        Key { bits: val }
    }

    /// Gets a Key from a char
    pub const fn from_char(val: char) -> Key {
        Key { bits: val as i32 }
    }

    /// Get the char representation of a Key.
    pub const fn to_char(&self) -> Option<char> {
        let bits = self.bits();
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
            Key::F1 => write!(f, "Key::F1"),
            Key::F2 => write!(f, "Key::F2"),
            Key::F3 => write!(f, "Key::F3"),
            Key::F4 => write!(f, "Key::F4"),
            Key::F5 => write!(f, "Key::F5"),
            Key::F6 => write!(f, "Key::F6"),
            Key::F7 => write!(f, "Key::F7"),
            Key::F8 => write!(f, "Key::F8"),
            Key::F9 => write!(f, "Key::F9"),
            Key::F10 => write!(f, "Key::F10"),
            Key::F11 => write!(f, "Key::F11"),
            Key::F12 => write!(f, "Key::F12"),
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
            _ => {
                write!(f, "Key::from_i32({})", self.bits())
            }
        }
    }
}

bitflags::bitflags! {
    /// Defines the modifiers of virtual keycodes
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
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
            Shortcut::Meta.bits()
        } else {
            Shortcut::Ctrl.bits()
        };
        /// Control (Ctrl for macOS, Meta for other systems)
        const Control = if cfg!(target_os = "macos") {
            Shortcut::Ctrl.bits()
        } else {
            Shortcut::Meta.bits()
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
        Shortcut::from_bits_retain(c as _)
    }

    /// Create a shortcut from a key
    pub const fn from_key(k: Key) -> Shortcut {
        Shortcut::from_bits_retain(k.bits())
    }

    /// Create a shortcut from an i32
    pub const fn from_i32(v: i32) -> Shortcut {
        Shortcut::from_bits_retain(v)
    }

    /// get key mask
    pub const fn key(&self) -> Key {
        let mut temp = self.bits();
        temp &= 0x0000_ffff;
        Key::from_i32(temp)
    }

    /// Get the button number
    pub const fn button(button_num: i32) -> Shortcut {
        Shortcut::from_bits_retain(0x0080_0000 << button_num)
    }
}

bitflags::bitflags! {
    /// Defines the types of triggers for widget callback functions. Equivalent to FL_WHEN
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
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
        /// A child widget is closed (in a Tabs widget)
        const Closed = 16;
    }
}

/// Defines the callback reasons which can be queried using `app::callback_reason()`.
#[repr(i32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum CallbackReason {
    /// Unknown reason
    Unknown,
    /// Item was selected
    Selected,
    /// Item was deselected
    Deselected,
    /// Item was reselected
    Reselected,
    /// Item was opened
    Opened,
    /// Item was closed
    Closed,
    /// Item was dragged
    Dragged,
    /// Operation cancelled
    Cancelled,
    /// Item was changed
    Changed,
    /// Item got focus
    GotFocus,
    /// Item lost focus
    LostFocus,
    /// Item released
    Released,
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
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
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
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
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
        unsafe { mem::transmute(self.bits() | other as i32) }
    }
}

impl std::ops::BitOr<Key> for Shortcut {
    type Output = Shortcut;
    fn bitor(self, other: Key) -> Self::Output {
        unsafe { mem::transmute(self.bits() | other.bits()) }
    }
}

impl std::ops::BitOr<i32> for Align {
    type Output = Align;
    fn bitor(self, rhs: i32) -> Self::Output {
        unsafe { mem::transmute(self.bits() | rhs) }
    }
}
