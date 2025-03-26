use crate::enums::{Color, Font};
use crate::prelude::*;
use crate::utils::FlString;
use fltk_sys::group::*;
use std::{
    ffi::{CStr, CString},
    os::raw::c_char,
};

/// Creates a scrollable display widget to handle terminal-like behavior, such as
/// logging events or debug information.
/// Replaces `SimpleTerminal` widget
///
#[derive(Debug)]
pub struct Terminal {
    inner: crate::widget::WidgetTracker,
    is_derived: bool,
}

crate::macros::widget::impl_widget_ext!(Terminal, Fl_Terminal);
crate::macros::widget::impl_widget_base!(Terminal, Fl_Terminal);
crate::macros::widget::impl_widget_default!(Terminal, Fl_Terminal);
crate::macros::group::impl_group_ext!(Terminal, Fl_Terminal);

///   Determines when `Fl_Terminal` calls `redraw()` if new text is added.
/// `RATE_LIMITED` is the recommended setting, using `redraw_rate(float)` to determine
/// the maximum rate of redraws.
/// see `redraw_style()`, `redraw_rate()`
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct RedrawStyle {
    bits: u32,
}

impl RedrawStyle {
    /// App must call `redraw()` as needed to update text to screen
    pub const NoRedraw: RedrawStyle = RedrawStyle { bits: 0x0000 };
    /// timer controlled redraws. (DEFAULT)
    pub const RateLimited: RedrawStyle = RedrawStyle { bits: 0x0001 };
    /// redraw triggered after *every* `append()` / `printf()` / etc. operation
    pub const PerWrite: RedrawStyle = RedrawStyle { bits: 0x0002 };

    /// Gets the inner representation
    pub const fn bits(&self) -> u32 {
        self.bits
    }
    /// Build a `RedrawStyle` enum with an arbitrary value.
    pub const fn new(val: u32) -> Self {
        RedrawStyle { bits: val }
    }
}

bitflags::bitflags! {
    /// Bits for the per-character attributes, which control text features
    /// such as italic, bold, underlined text, etc.
    /// Can be combined with | operator
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct Attrib: u8 {
        /// all attributes off
        const Normal =  0x00 ;
        /// bold text: uses bold font, color brighter than normal
        const Bold = 0x01 ;
        /// dim text; color slightly darker than normal
        const Dim =  0x02 ;
        /// italic font text
        const Italic =  0x04 ;
        /// underlined text
        const Underline =  0x08 ;
        /// <EM>(reserved for internal future use)</EM>
        const _Reserved1 =   0x10 ;
        /// inverse text; fg/bg color are swapped
        const Inverse =   0x20 ;
        /// <EM>(reserved for internal future use)</EM>
        const _Reserved2 = 0x40 ;
        /// strikeout text
        const Strikeout = 0x80 ;
    }
}

bitflags::bitflags! {
    /// Output translation flags for special control character translations.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct OutFlags: u8 {
        ///< no output translation
        const OFF        = 0x00;
        ///< carriage return generates a vertical line-feed (\\r -> \\n)
        const CR_TO_LF   = 0x01;
        ///< line-feed generates a carriage return (\\n -> \\r)
        const LF_TO_CR   = 0x02;
        ///< line-feed generates a carriage return line-feed (\\n -> \\r\\n)
        const LF_TO_CRLF = 0x04;
    }
}

///    'xterm color' values, used in `set_text_fg_color_xterm` and `set_text_bg_color_xterm`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
#[allow(missing_docs)] // These color names are self-documenting
#[non_exhaustive]
pub enum XtermColor {
    Black = 0,
    Red = 1,
    Green = 2,
    Yellow = 3,
    Blue = 4,
    Magenta = 5,
    Cyan = 6,
    White = 7,
}

bitflags::bitflags! {
    /// Per-character 8 bit flags (u8) used to manage special states for characters.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct CharFlags: u8 {
        /// No flags
        const NONE   = 0x00;
        /// this char's fg color is an XTERM color; can be affected by Dim+Bold
        const FG_XTERM   = 0x01;
        /// this char's bg color is an XTERM color; can be affected by Dim+Bold
        const BG_XTERM   = 0x02;
        /// used internally for line re-wrap during screen resizing
        const _EOL        = 0x04;
        /// Reserved
        const _RESV_A     = 0x08;
        /// Reserved
        const _RESV_B     = 0x10;
        /// Reserved
        const _RESV_C     = 0x20;
        /// Reserved
        const _RESV_D     = 0x40;
        /// Reserved
        const _RESV_E     = 0x80;
    }
}

///    Controls behavior of scrollbar
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ScrollbarStyle {
    bits: i32,
}
impl ScrollbarStyle {
    /// Scrollbar is always invisible
    pub const OFF: ScrollbarStyle = ScrollbarStyle { bits: 0 };
    /// scrollbar is visible if widget has been resized in a way that hides some columns (default)
    pub const AUTO: ScrollbarStyle = ScrollbarStyle { bits: 1 };
    /// Scrollbar is always visible
    pub const ON: ScrollbarStyle = ScrollbarStyle { bits: 2 };

    /// Gets the inner representation
    pub const fn bits(&self) -> i32 {
        self.bits
    }
    /// Build a `HScrollbarStyle` with an arbitrary value.
    pub const fn new(val: i32) -> Self {
        ScrollbarStyle { bits: val }
    }
}

impl std::fmt::Debug for ScrollbarStyle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            ScrollbarStyle::OFF => write!(f, "ScrollbarStyle::OFF"),
            ScrollbarStyle::ON => write!(f, "ScrollbarStyle::ON"),
            ScrollbarStyle::AUTO => write!(f, "ScrollbarStyle::AUTO"),
            _ => write!(f, "ScrollbarStyle::{}", self.bits()),
        }
    }
}

///    Class to manage the terminal's individual UTF-8 characters.
///    Includes fg/bg color, attributes (BOLD, UNDERLINE..)
/// *This is a low-level "protected" class in the fltk library*
pub struct Utf8Char {
    inner: *const Fl_Terminal_Utf8Char, // This points to a C++ Fl_Terminal::Utf8Char structure
}

impl std::fmt::Debug for Utf8Char {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let x = self.text_utf8();
        write!(
            f,
            "Utf8Char {:?} '{}'  fg:{} bg:{} {:?}",
            x,
            std::str::from_utf8(x).unwrap(),
            self.fgcolor(),
            self.bgcolor(),
            self.attrib()
        )
    }
}

///    Class to read characters from the terminal's buffer rows.
///    Includes indexing access and iterators
///    *This is a low-level "protected" class*
pub struct BuffRow<'a> {
    inner: *const Fl_Terminal_Utf8Char, // This points to an array of Fl_Terminal::Utf8Char
    /// Parent terminal widget that owns this buffer
    _parent: &'a Terminal,
    /// Number of characters in the row
    pub length: usize,
    /// `sizeof(Fl_Terminal::Utf8Char)`
    pub char_size: usize,
}

impl Terminal {
    /// Returns whether the terminal is in ANSI mode.
    pub fn ansi(&self) -> bool {
        unsafe { Fl_Terminal_ansi(self.inner.widget() as _) != 0 }
    }

    /// Enable/disable ANSI mode. If true, ANSI and VT100/xterm codes will be processed.
    /// If false, these codes won't be processed and will either be ignored or print the
    /// error character "¿", depending on the value of `show_unknown()`.
    pub fn set_ansi(&mut self, arg1: bool) {
        unsafe { Fl_Terminal_set_ansi(self.inner.widget() as _, i32::from(arg1)) }
    }

    /// Appends text to the terminal at current cursor position using the current text color/attributes.
    /// Redraws are managed automatically by default; see `redraw_style()`
    pub fn append(&mut self, s: &str) {
        let raw_s = CString::safe_new(s).into_raw();
        unsafe {
            Fl_Terminal_append(self.inner.widget() as _, raw_s as _);
            // Take ownership of raw_s back so it will be dropped
            let _raw_s = CString::from_raw(raw_s);
        }
    }

    /// Appends data to the terminal at current cursor position using the current text color/attributes
    /// Redraws are managed automatically by default; see `redraw_style()`
    pub fn append_u8(&mut self, s: &[u8]) {
        unsafe { Fl_Terminal_append_u8(self.inner.widget() as _, s.as_ptr() as _, s.len() as _) }
    }

    /// Appends text to the terminal at current cursor position using the current text color/attributes.
    /// Slightly more efficient than `append_utf8`
    /// Redraws are managed automatically by default; see `redraw_style()`
    pub fn append_ascii(&mut self, s: &str) {
        let raw_s = CString::safe_new(s).into_raw();
        unsafe {
            Fl_Terminal_append_ascii(self.inner.widget() as _, raw_s as _);
            // Take ownership of raw_s back so it will be dropped
            let _raw_s = CString::from_raw(raw_s);
        }
    }

    /// Appends text to the terminal at current cursor position using the current text color/attributes.
    /// Handles UTF-8 chars split across calls
    /// Redraws are managed automatically by default; see `redraw_style()`
    pub fn append_utf8(&mut self, s: &str) {
        let raw_s = CString::safe_new(s).into_raw();
        unsafe {
            Fl_Terminal_append_utf8(self.inner.widget() as _, raw_s as _);
            // Take ownership of raw_s back so it will be dropped
            let _raw_s = CString::from_raw(raw_s);
        }
    }

    /// Appends data to the terminal at current cursor position using the current text color/attributes
    /// Handles UTF-8 chars split across calls
    /// Redraws are managed automatically by default; see `redraw_style()`
    pub fn append_utf8_u8(&mut self, s: &[u8]) {
        unsafe {
            Fl_Terminal_append_utf8_u8(self.inner.widget() as _, s.as_ptr() as _, s.len() as _);
        }
    }

    /// Clears the screen to the current `textbgcolor()`, and homes the cursor.
    pub fn clear(&mut self) {
        unsafe { Fl_Terminal_clear(self.inner.widget() as _) }
    }

    /// Clear any current mouse selection.
    pub fn clear_mouse_selection(&mut self) {
        unsafe { Fl_Terminal_clear_mouse_selection(self.inner.widget() as _) }
    }

    ///  Clears the screen to a specific color `val` and homes the cursor.
    /// Does not affect the value of `text_bg_color` or `text_bg_color_default`
    pub fn clear_to_color(&mut self, val: Color) {
        unsafe { Fl_Terminal_clear_to_color(self.inner.widget() as _, val.bits()) }
    }

    ///   Clear the terminal screen only; does not affect the cursor position.
    ///
    /// Also clears the current mouse selection.
    ///
    /// If `scroll_to_hist` is true, the screen is cleared by scrolling the
    /// contents into the scrollback history, where it can be retrieved with the
    /// scrollbar. If false, the screen is cleared
    /// and the scrollback history is unchanged.
    ///
    /// Similar to the escape sequence `\<ESC\>[2J`.
    pub fn clear_screen(&mut self, arg1: bool) {
        unsafe { Fl_Terminal_clear_screen(self.inner.widget() as _, i32::from(arg1)) }
    }

    ///   Clear the terminal screen and home the cursor
    ///
    /// Also clears the current mouse selection.
    ///
    /// If `scroll_to_hist` is true, the screen is cleared by scrolling the
    /// contents into the scrollback history, where it can be retrieved with the
    /// scrollbar. If false, the screen is cleared
    /// and the scrollback history is unchanged.
    ///
    /// Similar to the escape sequence `\<ESC\>[2J\<ESC\>[H`.
    pub fn clear_screen_home(&mut self, arg1: bool) {
        unsafe { Fl_Terminal_clear_screen_home(self.inner.widget() as _, i32::from(arg1)) }
    }

    /// Clears the scroll history buffer and adjusts scrollbar, forcing it to `redraw()`
    pub fn clear_history(&mut self) {
        unsafe { Fl_Terminal_clear_history(self.inner.widget() as _) }
    }

    /// Get the background color for the terminal's `Fl_Group::box()`.
    pub fn color(&self) -> Color {
        Color::from_rgbi(unsafe { Fl_Terminal_color(self.inner.widget() as _) })
    }

    /// Sets the background color for the terminal's `Fl_Group::box()`.
    ///
    /// If the `textbgcolor()` and `textbgcolor_default()` are set to the special
    /// "see through" color 0xffffffff when any text was added, changing `color()`
    /// affects the color that shows through behind that existing text.
    ///
    /// Otherwise, whatever specific background color was set for existing text will
    ///  persist after changing `color()`.
    ///
    /// To see the effects of a change to `color()`, follow up with a call to `redraw()`.
    ///
    /// The default value is 0x0.
    pub fn set_color(&mut self, color: Color) {
        unsafe { Fl_Terminal_set_color(self.inner.widget() as _, color.bits()) }
    }

    /// Return the cursor's current column position on the screen.
    pub fn cursor_col(&self) -> i32 {
        unsafe { Fl_Terminal_cursor_col(self.inner.widget() as _) }
    }

    /// Set the cursor's current column position on the screen.
    /// *This is a low-level "protected" function of the fltk library*
    pub fn set_cursor_col(&mut self, val: i32) {
        unsafe { Fl_Terminal_set_cursor_col(self.inner.widget() as _, val) }
    }

    /// Return the cursor's current row position on the screen.
    pub fn cursor_row(&self) -> i32 {
        unsafe { Fl_Terminal_cursor_row(self.inner.widget() as _) }
    }

    /// Set the cursor's current row position on the screen.
    /// *This is a low-level "protected" function of the fltk library*
    pub fn set_cursor_row(&mut self, val: i32) {
        unsafe { Fl_Terminal_set_cursor_row(self.inner.widget() as _, val) }
    }

    /// Moves cursor up `count` lines.
    ///  If cursor hits screen top, it either stops (does not wrap) if `do_scroll`
    ///  is false, or scrolls down if `do_scroll` is true.
    /// *This is a low-level "protected" function of the fltk library*
    pub fn cursor_up(&mut self, count: i32, do_scroll: bool) {
        unsafe { Fl_Terminal_cursor_up(self.inner.widget() as _, count, i32::from(do_scroll)) }
    }

    /// Moves cursor down `count` lines.
    ///  If cursor hits screen bottom, it either stops (does not wrap) if `do_scroll`
    ///  is false, or wraps and scrolls up if `do_scroll` is true.
    /// *This is a low-level "protected" function of the fltk library*
    pub fn cursor_down(&mut self, count: i32, do_scroll: bool) {
        unsafe { Fl_Terminal_cursor_down(self.inner.widget() as _, count, i32::from(do_scroll)) }
    }

    /// Moves cursor left `count` columns, and cursor stops (does not wrap) if it hits screen edge.
    /// *This is a low-level "protected" function of the fltk library*
    pub fn cursor_left(&mut self, count: i32) {
        unsafe { Fl_Terminal_cursor_left(self.inner.widget() as _, count) }
    }

    /// Moves cursor right `count` columns. If cursor hits right edge of screen,
    ///  it either stops (does not wrap) if `do_scroll` is false, or wraps and
    ///  scrolls up one line if `do_scroll` is true.
    /// *This is a low-level "protected" function of the fltk library*
    pub fn cursor_right(&mut self, count: i32, do_scroll: bool) {
        unsafe { Fl_Terminal_cursor_right(self.inner.widget() as _, count, i32::from(do_scroll)) }
    }

    /// Scroll the selection up(+)/down(-) number of rows
    /// *This is a low-level "protected" function of the fltk library*
    pub fn scroll(&mut self, count: i32) {
        unsafe { Fl_Terminal_scroll(self.inner.widget() as _, count) }
    }

    /// Clear from cursor to End Of Display (EOD), like "`<ESC>[J<ESC>[0J`".
    pub fn clear_eod(&mut self) {
        unsafe { Fl_Terminal_clear_eod(self.inner.widget() as _) }
    }

    /// Clear from cursor to End Of Line (EOL), like "`<ESC>[K`".
    pub fn clear_eol(&mut self) {
        unsafe { Fl_Terminal_clear_eol(self.inner.widget() as _) }
    }

    /// Clear entire line cursor is currently on.
    pub fn clear_cur_line(&mut self) {
        unsafe { Fl_Terminal_clear_cur_line(self.inner.widget() as _) }
    }

    /// Clear entire line for specified row.
    pub fn clear_line(&mut self, drow: i32) {
        unsafe { Fl_Terminal_clear_line(self.inner.widget() as _, drow) }
    }

    /// Clear from cursor to Start Of Display (EOD), like "`<ESC>[1J`".
    pub fn clear_sod(&mut self) {
        unsafe { Fl_Terminal_clear_sod(self.inner.widget() as _) }
    }

    /// Clear from cursor to Start Of Line (SOL), like "`<ESC>[1K`".
    pub fn clear_sol(&mut self) {
        unsafe { Fl_Terminal_clear_sol(self.inner.widget() as _) }
    }

    ///   Insert char `c` at the current cursor position for `rep` times.
    ///   Works only for single-byte characters, `c` can't be multi-byte UTF-8.
    ///   Does not wrap; characters at end of line are lost.
    /// *This is a low-level "protected" function of the fltk library*
    pub fn insert_char(&mut self, c: char, rep: i32) {
        let c = if c.len_utf8() > 1 { b' ' } else { c as u8 };
        unsafe { Fl_Terminal_insert_char(self.inner.widget() as _, c as c_char, rep) }
    }

    /// Insert char `c` for `rep` times at display row `drow` and column `dcol`.
    ///   Works only for single-byte characters, `c` can't be multi-byte UTF-8.
    ///   Does not wrap; characters at end of line are lost.
    /// *This is a low-level "protected" function of the fltk library*
    pub fn insert_char_eol(&mut self, c: char, drow: i32, dcol: i32, rep: i32) {
        let c = if c.len_utf8() > 1 { b' ' } else { c as u8 };
        unsafe {
            Fl_Terminal_insert_char_eol(self.inner.widget() as _, c as c_char, drow, dcol, rep);
        }
    }

    /// Insert `count` rows at current cursor position.
    ///  Causes rows below to scroll down, and empty lines created.
    ///  Lines deleted by scroll down are NOT moved into the scroll history.
    /// *This is a low-level "protected" function of the fltk library*
    pub fn insert_rows(&mut self, count: i32) {
        unsafe { Fl_Terminal_insert_rows(self.inner.widget() as _, count) }
    }

    /// Delete char(s) at (`drow`,`dcol`) for `count` times.
    pub fn delete_chars(&mut self, drow: i32, dcol: i32, count: i32) {
        unsafe { Fl_Terminal_delete_chars(self.inner.widget() as _, drow, dcol, count) }
    }

    /// Delete char(s) at cursor position for `count` times.
    pub fn delete_cur_chars(&mut self, count: i32) {
        unsafe { Fl_Terminal_delete_cur_chars(self.inner.widget() as _, count) }
    }

    ///  Delete `count` rows at cursor position.
    ///   Causes rows to scroll up, and empty lines created at bottom of screen.
    ///    Lines deleted by scroll up are NOT moved into the scroll history.
    /// *This is a low-level "protected" function of the fltk library*
    pub fn delete_rows(&mut self, count: i32) {
        unsafe { Fl_Terminal_delete_rows(self.inner.widget() as _, count) }
    }

    /// Get the cursor's background color used for the cursor itself.
    pub fn cursor_bg_color(&self) -> Color {
        Color::from_rgbi(unsafe { Fl_Terminal_cursor_bg_color(self.inner.widget() as _) })
    }

    /// Set the cursor's background color used for the cursor itself.
    pub fn set_cursor_bg_color(&mut self, color: Color) {
        unsafe { Fl_Terminal_set_cursor_bg_color(self.inner.widget() as _, color.bits()) }
    }

    /// Get the cursor's foreground color used for the cursor itself.
    pub fn cursor_fg_color(&self) -> Color {
        Color::from_rgbi(unsafe { Fl_Terminal_cursor_fg_color(self.inner.widget() as _) })
    }

    /// Set the cursor's foreground color used for the cursor itself.
    pub fn set_cursor_fg_color(&mut self, color: Color) {
        unsafe { Fl_Terminal_set_cursor_fg_color(self.inner.widget() as _, color.bits()) }
    }

    /// Get the current mouse selection. Returns `None` if no selection, or `Some([srow, scol, erow, ecol])` if there is a selection,
    ///   where row and col represent start/end positions in the ring buffer.
    /// *This is a low-level "protected" function of the fltk library*
    pub fn get_selection(&self) -> Option<[i32; 4]> {
        let mut retval: [i32; 4] = [0; 4];
        let ret =
            unsafe { Fl_Terminal_get_selection(self.inner.widget() as _, retval.as_mut_ptr()) };
        if ret != 0 { Some(retval) } else { None }
    }

    /// Move cursor to the home position (top/left).
    pub fn cursor_home(&mut self) {
        unsafe { Fl_Terminal_cursor_home(self.inner.widget() as _) }
    }

    /// Return terminal's display width in columns of text characters.
    pub fn display_columns(&self) -> i32 {
        unsafe { Fl_Terminal_display_columns(self.inner.widget() as _) }
    }

    /// Set terminal's display width in columns of text characters.
    /// Does not resize the terminal.
    pub fn set_display_columns(&mut self, val: i32) {
        unsafe { Fl_Terminal_set_display_columns(self.inner.widget() as _, val) }
    }

    /// Return terminal's display height in lines of text.
    pub fn display_rows(&self) -> i32 {
        unsafe { Fl_Terminal_display_rows(self.inner.widget() as _) }
    }

    /// Sets the terminal's scrollback history buffer size in lines of text (rows).
    pub fn set_history_rows(&mut self, arg1: i32) {
        unsafe { Fl_Terminal_set_history_rows(self.inner.widget() as _, arg1) }
    }

    /// Gets the terminal's scrollback history buffer size in lines of text (rows).
    pub fn history_rows(&self) -> i32 {
        unsafe { Fl_Terminal_history_rows(self.inner.widget() as _) }
    }

    /// Returns how many lines are "in use" by the screen history buffer.
    pub fn history_use(&self) -> i32 {
        unsafe { Fl_Terminal_history_use(self.inner.widget() as _) }
    }

    /// Set the bottom margin
    pub fn set_margin_bottom(&mut self, arg1: i32) {
        unsafe { Fl_Terminal_set_margin_bottom(self.inner.widget() as _, arg1) }
    }

    /// Return the bottom margin
    pub fn margin_bottom(&self) -> i32 {
        unsafe { Fl_Terminal_margin_bottom(self.inner.widget() as _) }
    }

    /// Set the left margin
    pub fn set_margin_left(&mut self, arg1: i32) {
        unsafe { Fl_Terminal_set_margin_left(self.inner.widget() as _, arg1) }
    }

    /// Return the left margin
    pub fn margin_left(&self) -> i32 {
        unsafe { Fl_Terminal_margin_left(self.inner.widget() as _) }
    }

    /// Set the right margin
    pub fn set_margin_right(&mut self, arg1: i32) {
        unsafe { Fl_Terminal_set_margin_right(self.inner.widget() as _, arg1) }
    }

    /// Return the right margin
    pub fn margin_right(&self) -> i32 {
        unsafe { Fl_Terminal_margin_right(self.inner.widget() as _) }
    }

    /// Set the top margin
    pub fn set_margin_top(&mut self, arg1: i32) {
        unsafe { Fl_Terminal_set_margin_top(self.inner.widget() as _, arg1) }
    }

    /// Return the top margin
    pub fn margin_top(&self) -> i32 {
        unsafe { Fl_Terminal_margin_top(self.inner.widget() as _) }
    }

    /// Given a height in pixels, return number of rows that "fits" into that area.
    /// *This is a low-level "protected" function of the fltk library*
    pub fn h_to_row(&self, pix: i32) -> i32 {
        unsafe { Fl_Terminal_h_to_row(self.inner.widget() as _, pix) }
    }

    /// Given a width in pixels, return number of columns that "fits" into that area.
    /// *This is a low-level "protected" function of the fltk library*
    pub fn w_to_col(&self, pix: i32) -> i32 {
        unsafe { Fl_Terminal_w_to_col(self.inner.widget() as _, pix) }
    }

    /// Sets the combined output translation flags to `val`.
    ///
    /// `val` can be sensible combinations of the `OutFlags` bit flags.
    ///
    /// The default is `LF_TO_CRLF`, so that \\n will generate both carriage-return (CR)
    /// and line-feed (LF).
    ///
    /// For \\r and \\n to be handled literally, use `output_translate(Terminal::OutFlags::OFF)`;
    /// To disable all output translations, use 0 or `Terminal::OutFlags::OFF`.
    pub fn set_output_translate(&mut self, val: OutFlags) {
        unsafe { Fl_Terminal_set_output_translate(self.inner.widget() as _, u32::from(val.bits())) }
    }

    /// Return the current combined output translation flags.
    pub fn output_translate(&self) -> OutFlags {
        let result = unsafe { Fl_Terminal_output_translate(self.inner.widget() as _) as i32 };
        OutFlags::from_bits(result as u8)
            .unwrap_or_else(|| panic!("Unknown OutFlags value {result} from output_translate"))
    }

    /// Prints single ASCII char `c` at current cursor position, and advances the cursor.
    /// - `c` must be ASCII, not utf-8
    /// - Does not trigger redraws
    pub fn print_char(&mut self, c: char) {
        unsafe { Fl_Terminal_print_char(self.inner.widget() as _, c as std::os::raw::c_char) }
    }

    ///   Prints single UTF-8 char `c` at current cursor position, and advances the cursor if the character
    ///   is printable. Handles ASCII and control codes (CR, LF, etc).
    ///
    ///  The character is displayed at the current cursor position
    ///  using the current text color/attributes.
    ///
    /// Handles control codes and can be used to construct ANSI/XTERM escape sequences.
    /// - `c` must be a single char only (whether UTF-8 or ASCII)
    /// - `c` can be an ASCII character, though not as efficent as `print_char()`
    /// - Invalid UTF-8 chars show the error character (¿) depending on `show_unknown(bool)`.
    /// - Does not trigger redraws
    pub fn print_char_utf8(&mut self, c: char) {
        let txt = c.to_string();
        unsafe {
            Fl_Terminal_print_char_utf8(
                self.inner.widget() as _,
                txt.as_ptr() as _,
                txt.len() as _,
            );
        }
    }

    /// Print the ASCII character `c` at the terminal's display position `(drow,dcol)`.
    ///   The character MUST be printable (in range 0x20 - 0x7e), and is displayed
    ///   using the current text color/attributes. Characters outside that range are either
    ///   ignored or print the error character (¿), depending on `show_unknown(bool)`.
    ///
    /// No range checking is done on drow,dcol:
    /// - drow must be in range `0..(display_rows()-1)`
    /// - dcol must be in range `0..(display_columns()-1)`
    /// - Does not trigger redraws
    /// - Does NOT handle control codes, ANSI or XTERM escape sequences.
    pub fn plot_char(&mut self, c: char, row: i32, col: i32) {
        unsafe {
            Fl_Terminal_plot_char(
                self.inner.widget() as _,
                c as std::os::raw::c_char,
                row,
                col,
            );
        }
    }

    /// Print a single UTF-8 character len at display position `(drow,dcol)`.
    /// The character is displayed using the current text color/attributes.
    ///
    /// This is a very low level method.
    /// No range checking is done on drow,dcol:
    /// -  drow must be in range `0..(display_rows()-1)`
    /// -  dcol must be in range `0..(display_columns()-1)`
    /// - Does not trigger redraws
    /// - Does not handle ANSI or XTERM escape sequences
    /// - Invalid UTF-8 chars show the error character (¿) depending on `show_unknown(bool)`.
    pub fn plot_char_utf8(&mut self, c: char, drow: i32, dcol: i32) {
        let txt = c.to_string();
        unsafe {
            Fl_Terminal_plot_char_utf8(
                self.inner.widget() as _,
                txt.as_ptr() as _,
                txt.len() as _,
                drow,
                dcol,
            );
        }
    }

    /// Set the maximum rate redraw speed in floating point seconds if `redraw_style()` is set to `RATE_LIMITED`.
    pub fn set_redraw_rate(&mut self, set: f32) {
        unsafe { Fl_Terminal_set_redraw_rate(self.inner.widget() as _, set) }
    }

    /// Get max rate redraw speed in floating point seconds.
    pub fn redraw_rate(&self) -> f32 {
        unsafe { Fl_Terminal_redraw_rate(self.inner.widget() as _) }
    }

    /// Set how Terminal manages screen redrawing.
    pub fn set_redraw_style(&mut self, set: RedrawStyle) {
        unsafe { Fl_Terminal_set_redraw_style(self.inner.widget() as _, set.bits() as i32) }
    }

    /// Get the redraw style.
    pub fn redraw_style(&self) -> RedrawStyle {
        let result = unsafe { Fl_Terminal_redraw_style(self.inner.widget() as _) as u32 };
        RedrawStyle::new(result) // Construct a style with the given value
    }

    /// Resets terminal to default colors, clears screen, history and mouse selection, homes cursor, resets tabstops.
    pub fn reset_terminal(&mut self) {
        unsafe { Fl_Terminal_reset_terminal(self.inner.widget() as _) }
    }

    /// Returns the scrollbar's actual "trough size", which is the width of `FL_VERTICAL`
    /// scrollbars, or height of `FL_HORIZONTAL` scrollbars.
    ///
    /// If `scrollbar_size()` is zero (default), then the value of the global `Fl::scrollbar_size()`
    /// is returned, which is the default global scrollbar size for the entire application.
    pub fn scrollbar_actual_size(&self) -> i32 {
        unsafe { Fl_Terminal_scrollbar_actual_size(self.inner.widget() as _) }
    }

    ///   Get current pixel size of all the scrollbar's troughs for this widget,
    ///   or zero if the global `Fl::scrollbar_size()` is being used (default).
    ///
    ///   If this value returns *zero*, this widget's scrollbars are using the
    ///   global `Fl::scrollbar_size()`, in which case use `scrollbar_actual_size()`
    ///   to get the actual (effective) pixel scrollbar size being used.
    ///
    ///   __returns__ Scrollbar trough size in pixels, or 0 if the global `Fl::scrollbar_size()` is being used.
    pub fn scrollbar_size(&self) -> i32 {
        unsafe { Fl_Terminal_scrollbar_size(self.inner.widget() as _) }
    }

    /// Set the width of the both horizontal and vertical scrollbar's trough to `val`, in pixels.
    /// If this value is zero (default), this widget will use fltk's master `scrollbar_size()` value
    pub fn set_scrollbar_size(&mut self, val: i32) {
        unsafe { Fl_Terminal_set_scrollbar_size(self.inner.widget() as _, val) }
    }

    /// Get mouse selection background color.
    pub fn selection_bg_color(&self) -> Color {
        Color::from_rgbi(unsafe { Fl_Terminal_selection_bg_color(self.inner.widget() as _) })
    }

    /// Returns the vertical scrollbar
    pub fn scrollbar(&self) -> crate::valuator::Scrollbar {
        unsafe {
            let ptr = Fl_Terminal_scrollbar(self.inner.widget() as _);
            assert!(!ptr.is_null());
            crate::valuator::Scrollbar::from_widget_ptr(ptr as *mut fltk_sys::widget::Fl_Widget)
        }
    }

    /// Returns the horizontal scrollbar
    pub fn hscrollbar(&self) -> crate::valuator::Scrollbar {
        unsafe {
            let ptr = Fl_Terminal_hscrollbar(self.inner.widget() as _);
            assert!(!ptr.is_null());
            crate::valuator::Scrollbar::from_widget_ptr(ptr as *mut fltk_sys::widget::Fl_Widget)
        }
    }

    /// Get the horizontal scrollbar behavior style.
    ///
    ///  This determines when the scrollbar is visible.
    ///
    /// Value will be one of the `Fl_Terminal::HScrollbarStyle` enum values.
    pub fn hscrollbar_style(&self) -> ScrollbarStyle {
        unsafe { ScrollbarStyle::new(Fl_Terminal_hscrollbar_style(self.inner.widget() as _)) }
    }

    ///   Set the scrollbar behavior style.
    ///
    ///  This determines when the scrollbar is visible.
    ///
    ///  |   `ScrollbarStyle` enum    | Description                                           |
    ///  |---------------------------|-------------------------------------------------------|
    ///  |   ON                      | Scrollbar is always displayed.             |
    ///  |   OFF                     | Scrollbar is never displayed.              |
    ///  |   AUTO                    | Scrollbar is displayed whenever the widget has been resized so that some of the text is hidden. |
    ///
    ///  The default style is AUTO
    pub fn set_hscrollbar_style(&mut self, val: ScrollbarStyle) {
        unsafe { Fl_Terminal_set_hscrollbar_style(self.inner.widget() as _, val.bits()) }
    }

    /// Set mouse selection background color.
    pub fn set_selection_bg_color(&mut self, color: Color) {
        unsafe { Fl_Terminal_set_selection_bg_color(self.inner.widget() as _, color.bits()) }
    }

    /// Get mouse selection foreground color.
    pub fn selection_fg_color(&self) -> Color {
        Color::from_rgbi(unsafe { Fl_Terminal_selection_fg_color(self.inner.widget() as _) })
    }

    /// Set mouse selection foreground color.
    pub fn set_selection_fg_color(&mut self, color: Color) {
        unsafe { Fl_Terminal_set_selection_fg_color(self.inner.widget() as _, color.bits()) }
    }

    /// Return the "show unknown" flag. if true, show unknown chars as '¿'
    pub fn show_unknown(&self) -> bool {
        unsafe { Fl_Terminal_show_unknown(self.inner.widget() as _) != 0 }
    }

    /// Set the "show unknown" flag. if true, show unknown chars as '¿' (default off)
    pub fn set_show_unknown(&mut self, arg1: bool) {
        unsafe { Fl_Terminal_set_show_unknown(self.inner.widget() as _, i32::from(arg1)) }
    }

    /// Return the text attribute bits (underline, inverse, etc) for subsequent appends.
    pub fn text_attrib(&self) -> Attrib {
        // Attrib::from_bits( unsafe { Fl_Terminal_text_attrib(self.inner.widget()) as _ } ).unwrap()
        let result = unsafe { Fl_Terminal_text_attrib(self.inner.widget() as _) };
        Attrib::from_bits(result).unwrap_or_else(|| panic!("Unknown Attrib value {result}"))
    }

    /// Set text attribute bits (underline, inverse, etc) for subsequent appends.
    pub fn set_text_attrib(&mut self, arg1: Attrib) {
        unsafe { Fl_Terminal_set_text_attrib(self.inner.widget() as _, arg1.bits()) }
    }

    /// Set text background color to fltk color val.
    /// Use this for temporary color changes, similar to \<ESC\>[48;2;{R};{G};{B}m
    ///
    /// This setting does not affect the 'default' text colors used by \<ESC\>[0m, \<ESC\>c, `reset_terminal()`, etc.
    /// To change both the current and default bg color, also use `text_bg_color_default(Fl_Color)`.
    pub fn set_text_bg_color(&mut self, color: Color) {
        unsafe { Fl_Terminal_set_text_bg_color(self.inner.widget() as _, color.bits()) }
    }

    /// Get the text background color.
    pub fn text_bg_color(&self) -> Color {
        Color::from_rgbi(unsafe { Fl_Terminal_text_bg_color(self.inner.widget() as _) })
    }

    /// Set the default text background color used by \<ESC\>c, \<ESC\>[0m, and `reset_terminal()`.
    /// Does not affect the 'current' text fg color; use `set_text_bg_color(Fl_Color)` to set that.
    pub fn set_text_bg_color_default(&mut self, color: Color) {
        unsafe { Fl_Terminal_set_text_bg_color_default(self.inner.widget() as _, color.bits()) }
    }

    /// Return the default text background color.
    pub fn text_bg_color_default(&self) -> Color {
        Color::from_rgbi(unsafe { Fl_Terminal_text_bg_color_default(self.inner.widget() as _) })
    }

    /// Sets the background text color as one of the 8 'xterm color' values.
    ///
    /// This will be the background color used for all newly printed text, similar to the \<ESC\>[#m escape sequence, where # is between 40 and 47.
    ///
    /// This color will be reset to the default bg color if `reset_terminal()` is called, or by \<ESC\>c, \<ESC\>[0m, etc.
    ///
    /// The xterm color intensity values can be influenced by the Dim/Bold/Normal modes (which can be set with e.g. \<ESC\>[1m, `textattrib()`, etc), so the actual RGB values of these colors allow room for Dim/Bold to influence their brightness. For instance, "Normal Red" is not full brightness to allow "Bold Red" to be brighter. This goes for all colors except 'Black', which is not influenced by Dim or Bold; Black is always Black.
    ///
    /// These background colors are slightly dimmer than the corresponding xterm foregroumd colors.
    ///
    /// The 8 color xterm values are:
    /// 0 = Black, 1 = Red, 2 = Green, 3 = Yellow, 4 = Blue,5 = Magenta, 6 = Cyan, 7 = White
    pub fn set_text_bg_color_xterm(&mut self, color: XtermColor) {
        unsafe { Fl_Terminal_set_text_bg_color_xterm(self.inner.widget() as _, color as u8) }
    }
    ///  Set the text color for the terminal.
    ///  This is a convenience method that sets *both* `textfgcolor()` and `textfgcolor_default()`,
    ///  ensuring both are set to the same value.
    pub fn set_text_color(&mut self, color: Color) {
        unsafe { Fl_Terminal_set_text_color(self.inner.widget() as _, color.bits()) }
    }
    /// Set text foreground drawing color to fltk color val.
    /// Use this for temporary color changes, similar to \<ESC\>[38;2;{R};{G};{B}m
    ///
    /// This setting does not affect the 'default' text colors used by \<ESC\>[0m, \<ESC\>c, `reset_terminal()`, etc.
    /// To change both the current and default fg color, also use `textfgcolor_default(Fl_Color)`
    pub fn set_text_fg_color(&mut self, color: Color) {
        unsafe { Fl_Terminal_set_text_fg_color(self.inner.widget() as _, color.bits()) }
    }

    /// Get the text foreground color.
    pub fn text_fg_color(&self) -> Color {
        Color::from_rgbi(unsafe { Fl_Terminal_text_fg_color(self.inner.widget() as _) })
    }

    /// Set the default text foreground color used by \<ESC\>c, \<ESC\>[0m, and `reset_terminal()`.
    /// Does not affect the 'current' text fg color; use `set_text_fg_color(Fl_Color)` to set that.
    pub fn set_text_fg_color_default(&mut self, color: Color) {
        unsafe { Fl_Terminal_set_text_fg_color_default(self.inner.widget() as _, color.bits()) }
    }

    /// Return the default text foreground color.
    pub fn text_fg_color_default(&self) -> Color {
        Color::from_rgbi(unsafe { Fl_Terminal_text_fg_color_default(self.inner.widget() as _) })
    }

    /// Sets the foreground text color as one of the 8 'xterm color' values.
    ///
    /// This will be the foreground color used for all newly printed text, similar to the \<ESC\>[#m escape sequence, where # is between 30 and 37.
    ///
    /// This color will be reset to the default bg color if `reset_terminal()` is called, or by \<ESC\>c, \<ESC\>[0m, etc.
    ///
    /// The xterm color intensity values can be influenced by the Dim/Bold/Normal modes (which can be set with e.g. \<ESC\>[1m, `textattrib()`, etc), so the actual RGB values of these colors allow room for Dim/Bold to influence their brightness. For instance, "Normal Red" is not full brightness to allow "Bold Red" to be brighter. This goes for all colors except 'Black', which is not influenced by Dim or Bold; Black is always Black.
    ///
    /// The 8 color xterm values are:
    /// 0 = Black, 1 = Red, 2 = Green, 3 = Yellow, 4 = Blue,5 = Magenta, 6 = Cyan, 7 = White
    pub fn set_text_fg_color_xterm(&mut self, color: XtermColor) {
        unsafe { Fl_Terminal_set_text_fg_color_xterm(self.inner.widget() as _, color as u8) }
    }

    /// Get the text font
    pub fn text_font(&self) -> Font {
        Font::by_index(unsafe { Fl_Terminal_text_font(self.inner.widget() as _) } as usize)
    }

    /// Sets the font used for all text displayed in the terminal.
    /// This affects all existing text (in display and history) as well as any newly printed text.
    /// Only monospace fonts are recommended.
    pub fn set_text_font(&mut self, font: Font) {
        unsafe { Fl_Terminal_set_text_font(self.inner.widget() as _, font.bits()) }
    }

    /// Return text font size used to draw all text in the terminal.
    pub fn text_size(&self) -> i32 {
        unsafe { Fl_Terminal_text_size(self.inner.widget() as _) }
    }

    /// Sets the font size used for all text displayed in the terminal.
    /// This affects all existing text (in display and history) as well as any newly printed text.
    /// Changing this will affect the `display_rows()` and `display_columns()`.
    pub fn set_text_size(&mut self, val: i32) {
        unsafe { Fl_Terminal_set_text_size(self.inner.widget() as _, val) }
    }

    /// Return a string copy of all lines in the terminal (including history).
    ///
    /// If `lines_below_cursor` is false, lines below the cursor on down
    /// to the bottom of the display are ignored, and not included in the returned string.
    ///
    ///  If `lines_below_cursor` is true, then all lines in the display are returned
    ///  including any below the cursor, even if all are blank.
    ///
    ///  Example use:
    ///  ```
    ///      use fltk::{prelude::*, terminal::Terminal};
    ///      let mut tty = Terminal::new(0, 0, 400, 300, None);
    ///      
    ///      let s = tty.text(true);   // get a copy of the terminal's contents
    ///      println!("Terminal's contents:\n{}", s);
    ///  ```
    pub fn text(&self, lines_below_cursor: bool) -> String {
        unsafe {
            let ptr = Fl_Terminal_text(self.inner.widget() as _, i32::from(lines_below_cursor));
            assert!(!ptr.is_null()); // Sanity check
            let result = CStr::from_ptr(ptr).to_string_lossy().to_string().clone();
            Fl_free_str(ptr);
            result
        }
    }

    /// Return text selection (for `copy()/paste()` operations)
    pub fn selection_text(&self) -> Option<String> {
        assert!(self.is_derived);
        unsafe {
            let ptr = Fl_Terminal_selection_text(self.inner.widget() as _);
            if ptr.is_null() {
                None
            } else {
                let result = CStr::from_ptr(ptr).to_string_lossy().to_string();
                Fl_free_str(ptr);
                Some(result)
            }
        }
    }

    ///  Return byte length of all UTF-8 chars in selection, or 0 if no selection.
    ///  NOTE: Length includes trailing white on each line.
    pub fn selection_text_len(&self) -> i32 {
        unsafe { Fl_Terminal_selection_text_len(self.inner.widget() as _) }
    }

    // Various methods to access the ring buffer

    ///  Return the ending row# in the display area.
    pub fn disp_erow(&self) -> i32 {
        unsafe { Fl_Terminal_disp_erow(self.inner.widget() as _) }
    }

    /// Return the starting row# in the display area.
    pub fn disp_srow(&self) -> i32 {
        unsafe { Fl_Terminal_disp_srow(self.inner.widget() as _) }
    }

    /// Return the ending row# of the scrollback history.
    pub fn hist_erow(&self) -> i32 {
        unsafe { Fl_Terminal_hist_erow(self.inner.widget() as _) }
    }

    /// Return the starting row# of the scrollback history.
    pub fn hist_srow(&self) -> i32 {
        unsafe { Fl_Terminal_hist_srow(self.inner.widget() as _) }
    }

    /// Return number of rows in use by the scrollback history.
    pub fn hist_use(&self) -> i32 {
        unsafe { Fl_Terminal_hist_use(self.inner.widget() as _) }
    }

    /// Return the starting row of the \"in use\" scrollback history.
    pub fn hist_use_srow(&self) -> i32 {
        unsafe { Fl_Terminal_hist_use_srow(self.inner.widget() as _) }
    }

    /// Is global row/column inside the current mouse selection?
    /// *This is a low-level "protected" function of the fltk library*
    pub fn is_inside_selection(&self, row: i32, col: i32) -> bool {
        unsafe { Fl_Terminal_is_inside_selection(self.inner.widget() as _, row, col) != 0 }
    }

    /// Returns true if there's a mouse selection.
    pub fn is_selection(&self) -> bool {
        unsafe { Fl_Terminal_is_selection(self.inner.widget() as _) != 0 }
    }

    /// Returns the current offset into the ring buffer.
    pub fn offset(&self) -> i32 {
        unsafe { Fl_Terminal_offset(self.inner.widget() as _) }
    }

    /// Return the ending row# in the ring buffer (Always ring_rows()-1)
    pub fn ring_erow(&self) -> i32 {
        unsafe { Fl_Terminal_ring_erow(self.inner.widget() as _) }
    }

    /// Return the starting row# in the ring buffer (Always 0)
    pub fn ring_srow(&self) -> i32 {
        unsafe { Fl_Terminal_ring_srow(self.inner.widget() as _) }
    }

    /// Return the number of rows in the ring buffer.
    pub fn ring_rows(&self) -> i32 {
        unsafe { Fl_Terminal_ring_rows(self.inner.widget() as _) }
    }

    /// Return the `Utf8Char` for character under cursor.
    pub fn u8c_cursor(&self) -> Utf8Char {
        unsafe {
            let x = self.inner.widget();
            let utf8_p = Fl_Terminal_u8c_cursor(x as _);
            Utf8Char { inner: utf8_p }
        }
    }

    /// Return u8c for beginning of row drow of the display.
    /// *This is a low-level "protected" function of the fltk library*
    pub fn u8c_disp_row(&self, drow: i32) -> BuffRow {
        // Fl_Terminal_u8c_disp_row returns pointer to the first C++ Utf8Char object,
        //  which becomes the `inner` element in the Rust BuffRow object
        let row_p = unsafe { Fl_Terminal_u8c_disp_row(self.inner.widget() as _, drow) };
        BuffRow::new(row_p, self)
    }

    /// Return u8c for beginning of row hrow inside the scrollback history.
    /// *This is a low-level "protected" function of the fltk library*
    pub fn u8c_hist_row(&self, hrow: i32) -> BuffRow {
        // Fl_Terminal_u8c_hist_row returns pointer to the first C++ Utf8Char object,
        //  which becomes the `inner` element in the Rust BuffRow object
        let row_p = unsafe { Fl_Terminal_u8c_hist_row(self.inner.widget() as _, hrow) };
        BuffRow::new(row_p, self)
    }

    /// Return u8c for beginning of row hurow inside the 'in use' part of the\n scrollback history.
    /// *This is a low-level "protected" function of the fltk library*
    pub fn u8c_hist_use_row(&self, hurow: i32) -> BuffRow {
        // Fl_Terminal_u8c_hist_use_row returns pointer to the first  C++ Utf8Char object,
        //  which becomes the `inner` element in the Rust BuffRow object
        let row_p = unsafe { Fl_Terminal_u8c_hist_use_row(self.inner.widget() as _, hurow) };
        BuffRow::new(row_p, self)
    }

    /// Return u8c for beginning of row grow in the ring buffer.
    /// *This is a low-level "protected" function of the fltk library*
    pub fn u8c_ring_row(&self, grow: i32) -> BuffRow {
        // Fl_Terminal_u8c_ring_use_row returns pointer to the first  C++ Utf8Char object,
        //  which becomes the `inner` element in the Rust BuffRow object
        let row_p = unsafe { Fl_Terminal_u8c_ring_row(self.inner.widget() as _, grow) };
        BuffRow::new(row_p, self)
    }
}

// So far only implementing "getter" methods. Todo: methods to modify Utf8Char
impl Utf8Char {
    /// Construct a new `Utf8Char`, single-byte only. This is really only useful for testing.
    ///  'c' must be "printable" ASCII in the range (0x20 <= c <= 0x7e).
    ///     Anything outside of that is silently ignored.
    ///
    /// Allocated `Utf8Char` will never be deleted.
    pub fn new(c: u8) -> Self {
        unsafe {
            let u8c = Fl_Terminal_Utf8Char_new_obj(c);
            Utf8Char { inner: u8c }
        }
    }

    /// Return the actual displayed color of char `u8c` possibly influenced by BOLD or DIM if the char is from Xterm.
    ///    BG color will be derived from the widget color if a widget is specified and the color is `TransparentBg`,
    ///    and that won't be influenced by charflag attributes.
    pub fn attr_bgcolor(&self, term: Option<&Terminal>) -> Color {
        Color::from_rgbi(match term {
            None => unsafe { Fl_Terminal_Utf8Char_attr_bgcolor(self.inner, std::ptr::null()) },
            Some(t) => unsafe {
                Fl_Terminal_Utf8Char_attr_bgcolor(self.inner, t.inner.widget() as _)
            },
        })
    }

    // /// Return the actual displayed color of char `u8c` possibly influenced by BOLD or DIM if the char is from Xterm.
    // ///    If a `grp` widget is specified (i.e. not `None`), don't let the color be
    // ///    influenced by the attribute bits *if* it matches the `grp` widget's own `color()`.
    // pub fn attr_color(&self, grp: Option<*const Fl_Widget>) -> Color {
    //     Color::from_rgbi(match grp {
    //         None => unsafe { Fl_Terminal_Utf8Char_attr_color(self.inner, std::ptr::null()) },
    //         Some(g) => unsafe { Fl_Terminal_Utf8Char_attr_color(self.inner, g) },
    //     })
    // }

    /// Return the actual displayed fg color of char `u8c` possibly influenced by BOLD or DIM if the char is from Xterm.
    ///    If a `term` widget is specified (i.e. not `None`), don't let the color be
    ///    influenced by the attribute bits *if* it matches the `term` widget's own `color()`.
    pub fn attr_fgcolor(&self, term: Option<&Terminal>) -> Color {
        Color::from_rgbi(match term {
            None => unsafe { Fl_Terminal_Utf8Char_attr_fgcolor(self.inner, std::ptr::null()) },
            Some(t) => unsafe {
                Fl_Terminal_Utf8Char_attr_fgcolor(self.inner, t.inner.widget() as _)
            },
        })
    }

    /// Return the attributes for this character.
    pub fn attrib(&self) -> Attrib {
        let result = unsafe { Fl_Terminal_Utf8Char_attrib(self.inner) };
        Attrib::from_bits(result).unwrap_or_else(|| panic!("Unknown Attrib value {result}"))
    }

    /// Return the background color for this character.
    pub fn bgcolor(&self) -> Color {
        Color::from_rgbi(unsafe { Fl_Terminal_Utf8Char_bgcolor(self.inner) })
    }

    /// Return the foreground color for this character.
    pub fn fgcolor(&self) -> Color {
        let result = unsafe { Fl_Terminal_Utf8Char_fgcolor(self.inner) };
        Color::from_rgbi(result)
    }

    /// Return the xterm `CharFlags` bits
    pub fn charflags(&self) -> CharFlags {
        let result = unsafe { i32::from(Fl_Terminal_Utf8Char_charflags(self.inner)) };
        CharFlags::from_bits(result as u8)
            .unwrap_or_else(|| panic!("Unknown CharFlags value {result}"))
    }

    /// Returns true if the character text in this struct matches the given ASCII character
    pub fn is_char(&self, c: u8) -> bool {
        let result = unsafe { Fl_Terminal_Utf8Char_is_char(self.inner, c as c_char) as i32 };
        result != 0
    }

    /// Return the length of this character in bytes (UTF-8 can be multibyte)
    pub fn length(&self) -> usize {
        unsafe { Fl_Terminal_Utf8Char_length(self.inner) as usize }
    }

    /// Return the maximum length in bytes of a UTF-8 character
    pub fn max_utf8(&self) -> usize {
        unsafe { Fl_Terminal_Utf8Char_max_utf8(self.inner) as usize }
    }

    /// Return the width of this character in floating point pixels.
    ///
    ///    WARNING: Uses current font, so assumes font and `font_size`
    ///             have already been set to current font!
    pub fn pwidth(&self) -> f64 {
        unsafe { Fl_Terminal_Utf8Char_pwidth(self.inner) as f64 }
    }

    /// Return the width of this character in integer pixels.
    ///
    ///    WARNING: Uses current font, so assumes font and `font_size`
    ///             have already been set to current font!
    pub fn pwidth_int(&self) -> usize {
        unsafe { Fl_Terminal_Utf8Char_pwidth_int(self.inner) as usize }
    }

    /// Return the UTF-8 text string for this character.
    pub fn text_utf8(&self) -> &[u8] {
        unsafe {
            let ptr = Fl_Terminal_Utf8Char_text_utf8(self.inner);
            let len = Fl_Terminal_Utf8Char_length(self.inner);
            std::slice::from_raw_parts(ptr, len as usize)
        }
    }

    // Note: Fl_Terminal_Utf8Char_size() is used internally but not exposed to user Rust programs
}

impl<'a> BuffRow<'a> {
    /// Generate a new `BuffRow` object based on a pointer from C++ `Fl_Terminal`
    pub fn new(ptr: *const Fl_Terminal_Utf8Char, parent: &'a Terminal) -> Self {
        unsafe {
            BuffRow {
                // inner is the pointer to the first C++ Utf8Char in the row
                inner: ptr,
                _parent: parent,
                // length: (i + 1) as usize,
                length: parent.display_columns() as usize,
                char_size: Fl_Terminal_Utf8Char_size() as usize,
            }
        }
    }

    /// Trim trailing blanks off of `BuffRow` object.
    /// Does not affect the data in the `RingBuff`, just this object's access.
    #[must_use]
    pub fn trim(mut self) -> Self {
        unsafe {
            let mut last_char = self.inner.add((self.length - 1) * self.char_size);
            let c = Utf8Char { inner: last_char };
            // If the last character is a blank, trim the length back.
            if c.text_utf8() == b" " {
                // Record the attributes etc of the last character
                let attr = c.attrib();
                let fg = c.fgcolor();
                let bg = c.bgcolor();
                self.length -= 1; // Already checked the last character
                while self.length > 0 {
                    last_char = last_char.sub(self.char_size);
                    let c = Utf8Char { inner: last_char };
                    if c.text_utf8() != b" "
                        || c.attrib() != attr
                        || c.fgcolor() != fg
                        || c.bgcolor() != bg
                    {
                        break; // Found a non-blank character or one with attrib changes
                    }
                    self.length -= 1;
                }
            }
        }
        self
    }

    /// Index into row array of `Utf8Char`
    pub fn col(&self, idx: usize) -> Utf8Char {
        assert!((idx <= self.length), "Index {idx} out of range");
        unsafe {
            let base = self.inner;
            Utf8Char {
                inner: base.add(idx * self.char_size),
            }
        }
    }

    /// Iterator object to step through a sequence of `Utf8Char` in a `BuffRow`
    pub fn iter(&self) -> BuffRowIter {
        BuffRowIter::new(self, self.length)
    }
}

/// Iterator object to step through a sequence of `Utf8Char` in a `BuffRow`
pub struct BuffRowIter<'a> {
    parent: &'a BuffRow<'a>,
    ptr: *const Fl_Terminal_Utf8Char, // This points to an array of Fl_Terminal::Utf8Char
    end: *const Fl_Terminal_Utf8Char, // points just past the ptr array end
}

impl<'a> BuffRowIter<'a> {
    fn new(parent: &'a BuffRow, len: usize) -> BuffRowIter<'a> {
        unsafe {
            BuffRowIter {
                parent,
                ptr: parent.inner,
                end: parent.inner.add(len * parent.char_size),
            }
        }
    }
}

impl Iterator for BuffRowIter<'_> {
    type Item = Utf8Char;
    fn next(&mut self) -> Option<Self::Item> {
        if self.ptr < self.end {
            let result = Utf8Char { inner: self.ptr };
            unsafe {
                self.ptr = self.ptr.add(self.parent.char_size);
            }
            Some(result)
        } else {
            None
        }
    }
}

impl<'a> IntoIterator for &'a BuffRow<'_> {
    type Item = Utf8Char;
    type IntoIter = BuffRowIter<'a>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
