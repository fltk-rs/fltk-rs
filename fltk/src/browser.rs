use crate::enums::{Color, Font};
use crate::prelude::*;
use crate::utils::FlString;
use fltk_sys::browser::*;
use std::{
    ffi::{CStr, CString},
    mem,
    os::raw,
};

/**
    Creates a normal browser.
    Example usage:
    ```rust,no_run
    use fltk::{prelude::*, *};
    fn main() {
        let app = app::App::default();
        let mut win = window::Window::default().with_size(900, 300);
        let mut b = browser::Browser::new(10, 10, 900 - 20, 300 - 20, "");
        let widths = &[50, 50, 50, 70, 70, 40, 40, 70, 70, 50];
        b.set_column_widths(widths);
        b.set_column_char('\t');
        b.add("USER\tPID\t%CPU\t%MEM\tVSZ\tRSS\tTTY\tSTAT\tSTART\tTIME\tCOMMAND");
        b.add("root\t2888\t0.0\t0.0\t1352\t0\ttty3\tSW\tAug15\t0:00\t@b@f/sbin/mingetty tty3");
        b.add("erco\t2889\t0.0\t13.0\t221352\t0\ttty3\tR\tAug15\t1:34\t@b@f/usr/local/bin/render a35 0004");
        b.add("uucp\t2892\t0.0\t0.0\t1352\t0\tttyS0\tSW\tAug15\t0:00\t@b@f/sbin/agetty -h 19200 ttyS0 vt100");
        b.add("root\t13115\t0.0\t0.0\t1352\t0\ttty2\tSW\tAug30\t0:00\t@b@f/sbin/mingetty tty2");
        b.add(
            "root\t13464\t0.0\t0.0\t1352\t0\ttty1\tSW\tAug30\t0:00\t@b@f/sbin/mingetty tty1 --noclear",
        );
        win.end();
        win.show();
        app.run().unwrap();
    }
    ```
*/
#[derive(Debug)]
pub struct Browser {
    inner: *mut Fl_Browser,
    tracker: *mut fltk_sys::fl::Fl_Widget_Tracker,
    is_derived: bool,
}

crate::macros::widget::impl_widget_ext!(Browser, Fl_Browser);
crate::macros::widget::impl_widget_base!(Browser, Fl_Browser);
crate::macros::browser::impl_browser_ext!(Browser, Fl_Browser);

/// Defines the browser type
#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum BrowserType {
    /// Normal browser
    Normal = 0,
    /// Enable select
    Select = 1,
    /// Enable holding
    Hold = 2,
    /// Multi selection
    Multi = 3,
}

crate::macros::widget::impl_widget_type!(BrowserType);

/// Defines the type of Scrollbar associated with the browser
#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum BrowserScrollbar {
    /// Never show bars
    None = 0,
    /// Show vertical bar
    Horizontal = 1,
    /// Show vertical bar
    Vertical = 2,
    /// Show both horizontal and vertical bars
    Both = 3,
    /// Always show bars
    AlwaysOn = 4,
    /// Show horizontal bar always
    HorizontalAlways = 5,
    /// Show vertical bar always
    VerticalAlways = 6,
    /// Always show both horizontal and vertical bars
    BothAlways = 7,
}

/// Creates a select browser
#[derive(Debug)]
pub struct SelectBrowser {
    inner: *mut Fl_Select_Browser,
    tracker: *mut fltk_sys::fl::Fl_Widget_Tracker,
    is_derived: bool,
}

crate::macros::widget::impl_widget_ext!(SelectBrowser, Fl_Select_Browser);
crate::macros::widget::impl_widget_base!(SelectBrowser, Fl_Select_Browser);
crate::macros::browser::impl_browser_ext!(SelectBrowser, Fl_Select_Browser);

/// Creates a multi-browser
#[derive(Debug)]
pub struct MultiBrowser {
    inner: *mut Fl_Multi_Browser,
    tracker: *mut fltk_sys::fl::Fl_Widget_Tracker,
    is_derived: bool,
}

crate::macros::widget::impl_widget_ext!(MultiBrowser, Fl_Multi_Browser);
crate::macros::widget::impl_widget_base!(MultiBrowser, Fl_Multi_Browser);
crate::macros::browser::impl_browser_ext!(MultiBrowser, Fl_Multi_Browser);

/// Creates a hold browser
#[derive(Debug)]
pub struct HoldBrowser {
    inner: *mut Fl_Hold_Browser,
    tracker: *mut fltk_sys::fl::Fl_Widget_Tracker,
    is_derived: bool,
}

crate::macros::widget::impl_widget_ext!(HoldBrowser, Fl_Hold_Browser);
crate::macros::widget::impl_widget_base!(HoldBrowser, Fl_Hold_Browser);
crate::macros::browser::impl_browser_ext!(HoldBrowser, Fl_Hold_Browser);

/// Creates a file browser
#[derive(Debug)]
pub struct FileBrowser {
    inner: *mut Fl_File_Browser,
    tracker: *mut fltk_sys::fl::Fl_Widget_Tracker,
    is_derived: bool,
}

crate::macros::widget::impl_widget_ext!(FileBrowser, Fl_File_Browser);
crate::macros::widget::impl_widget_base!(FileBrowser, Fl_File_Browser);
crate::macros::browser::impl_browser_ext!(FileBrowser, Fl_File_Browser);

/// File types for the `FileBrowser`
#[repr(i32)]
#[derive(Copy, Clone, Debug)]
pub enum FileType {
    /// Show files
    Files = 0,
    /// Show dirs
    Dirs,
}

impl FileBrowser {
    /// Gets the icon size
    pub fn icon_size(&self) -> u32 {
        assert!(!self.was_deleted());
        unsafe { Fl_File_Browser_iconsize(self.inner) }
    }

    /// Sets the icon size
    pub fn set_icon_size(&mut self, s: u32) {
        assert!(!self.was_deleted());
        unsafe { Fl_File_Browser_set_iconsize(self.inner, s) }
    }

    /// Sets the filter for the `FileBrowser`.
    /// The following syntax is used for the pattern:
    /// `*` matches any sequence of 0 or more characters.
    /// `?` matches any single character.
    /// `[set]` matches any character in the set. The set can contain any single characters, or a-z to represent a range.
    /// To match `]` or `-`, they must be the first characters. To match `^` or `!`, they must not be the first characters.
    /// `[^set]` or `[!set]` matches any character not in the set.
    /// `{X|Y|Z}` or `{X,Y,Z}` matches any one of the subexpressions literally.
    /// `\x` quotes the character `x` so it has no special meaning.
    /// `x` all other characters must be matched exactly.
    pub fn set_filter(&mut self, pattern: &'static str) {
        assert!(!self.was_deleted());
        let pattern = CString::safe_new(pattern);
        unsafe {
            // This is deleted on the C++ side
            Fl_File_Browser_set_filter(self.inner, pattern.into_raw())
        }
    }

    /// Gets the filter for the `FileBrowser`
    pub fn filter(&self) -> Option<String> {
        assert!(!self.was_deleted());
        unsafe {
            let ptr = Fl_File_Browser_filter(self.inner);
            if ptr.is_null() {
                None
            } else {
                Some(
                    CStr::from_ptr(ptr as *mut raw::c_char)
                        .to_string_lossy()
                        .to_string(),
                )
            }
        }
    }

    /// Gets the `FileType` of the `FileBrowser`
    pub fn filetype(&self) -> FileType {
        assert!(!self.was_deleted());
        unsafe { mem::transmute(Fl_File_Browser_filetype(self.inner)) }
    }

    /// Sets the `FileType` of the `FileBrowser`
    pub fn set_filetype(&mut self, t: FileType) {
        assert!(!self.was_deleted());
        unsafe { Fl_File_Browser_set_filetype(self.inner, t as i32) }
    }
}

/// Creates a `CheckBrowser` widget
#[derive(Debug)]
pub struct CheckBrowser {
    inner: *mut Fl_Check_Browser,
    tracker: *mut fltk_sys::fl::Fl_Widget_Tracker,
    is_derived: bool,
}

crate::macros::widget::impl_widget_ext!(CheckBrowser, Fl_Check_Browser);
crate::macros::widget::impl_widget_base!(CheckBrowser, Fl_Check_Browser);

impl CheckBrowser {
    /// Add an item, returns the number of current items
    pub fn add(&mut self, s: &str, checked: bool) -> i32 {
        assert!(!self.was_deleted());
        let s = CString::safe_new(s);
        unsafe { Fl_Check_Browser_add(self.inner, s.as_ptr(), checked as i32) }
    }

    /// Remove item at index, returns the number of current items
    pub fn remove(&mut self, item: usize) -> i32 {
        assert!(!self.was_deleted());
        if item > 0 && item <= self.size() {
            unsafe { Fl_Check_Browser_remove(self.inner, item as i32) }
        } else {
            self.nitems() as _
        }
    }

    /// Clear the browser
    pub fn clear(&mut self) {
        assert!(!self.was_deleted());
        unsafe { Fl_Check_Browser_clear(self.inner) }
    }

    /// Return the number of items
    pub fn nitems(&self) -> usize {
        assert!(!self.was_deleted());
        unsafe { Fl_Check_Browser_nitems(self.inner) as usize }
    }

    /// Return the number of items
    pub fn size(&self) -> usize {
        self.nitems()
    }

    /// Get the number of checked items
    pub fn nchecked(&self) -> usize {
        assert!(!self.was_deleted());
        unsafe { Fl_Check_Browser_nchecked(self.inner) as usize }
    }

    /// Returns whether an item is checked
    pub fn checked(&self, item: i32) -> bool {
        assert!(!self.was_deleted());
        if item > 0 && item <= self.size() as i32 {
            unsafe { Fl_Check_Browser_checked(self.inner, item) != 0 }
        } else {
            false
        }
    }

    /// Check selected item
    pub fn set_checked(&mut self, item: i32) {
        assert!(!self.was_deleted());
        if item > 0 && item <= self.size() as i32 {
            unsafe { Fl_Check_Browser_set_checked(self.inner, item) }
        }
    }

    /// Check all of the items
    pub fn check_all(&mut self) {
        assert!(!self.was_deleted());
        unsafe { Fl_Check_Browser_check_all(self.inner) }
    }

    /// Check none of the items
    pub fn check_none(&mut self) {
        assert!(!self.was_deleted());
        unsafe { Fl_Check_Browser_check_none(self.inner) }
    }

    /// Returns the selected line, returns 0 if no line is selected
    pub fn value(&self) -> i32 {
        assert!(!self.was_deleted());
        unsafe { Fl_Check_Browser_value(self.inner) as i32 }
    }

    /// Get the text of the item
    pub fn text(&self, item: i32) -> Option<String> {
        assert!(!self.was_deleted());
        unsafe {
            let ptr = Fl_Check_Browser_text(self.inner, item);
            if ptr.is_null() {
                None
            } else {
                Some(CStr::from_ptr(ptr).to_string_lossy().to_string())
            }
        }
    }

    /// Gets the text font
    pub fn text_font(&self) -> Font {
        assert!(!self.was_deleted());
        unsafe { std::mem::transmute(Fl_Check_Browser_text_font(self.inner)) }
    }

    /// Sets the text font
    pub fn set_text_font(&mut self, f: Font) {
        assert!(!self.was_deleted());
        unsafe { Fl_Check_Browser_set_text_font(self.inner, f.bits() as i32) }
    }

    /// Gets the text size
    pub fn text_size(&self) -> i32 {
        assert!(!self.was_deleted());
        unsafe { Fl_Check_Browser_text_size(self.inner) as i32 }
    }

    /// Sets the text size
    pub fn set_text_size(&mut self, s: i32) {
        assert!(!self.was_deleted());
        unsafe { Fl_Check_Browser_set_text_size(self.inner, s as i32) }
    }

    /// Gets the text's color
    pub fn text_color(&self) -> Color {
        assert!(!self.was_deleted());
        unsafe { std::mem::transmute(Fl_Check_Browser_text_color(self.inner)) }
    }

    /// Sets the text's color
    pub fn set_text_color(&mut self, color: Color) {
        assert!(!self.was_deleted());
        unsafe { Fl_Check_Browser_set_text_color(self.inner, color.bits() as u32) }
    }

    /// Gets the vertical scroll position of the list as a pixel position
    pub fn position(&self) -> i32 {
        assert!(!self.was_deleted());
        unsafe { Fl_Check_Browser_position(self.inner) as i32 }
    }

    /// Sets the vertical scroll position of the list as a pixel position
    pub fn set_position(&mut self, pos: i32) {
        assert!(!self.was_deleted());
        unsafe { Fl_Check_Browser_set_position(self.inner, pos as i32) }
    }

    /// Gets the horizontal scroll position of the list as a pixel position
    pub fn hposition(&self) -> i32 {
        assert!(!self.was_deleted());
        unsafe { Fl_Check_Browser_hposition(self.inner) as i32 }
    }

    /// Sets the horizontal scroll position of the list as a pixel position
    pub fn set_hposition(&mut self, pos: i32) {
        assert!(!self.was_deleted());
        unsafe { Fl_Check_Browser_set_hposition(self.inner, pos as i32) }
    }

    /// Returns the type of scrollbar associated with the browser
    pub fn has_scrollbar(&self) -> BrowserScrollbar {
        assert!(!self.was_deleted());
        unsafe { mem::transmute(Fl_Check_Browser_has_scrollbar(self.inner)) }
    }

    /// Sets the type of scrollbar associated with the browser
    pub fn set_has_scrollbar(&mut self, mode: BrowserScrollbar) {
        assert!(!self.was_deleted());
        unsafe { Fl_Check_Browser_set_has_scrollbar(self.inner, mode as raw::c_uchar) }
    }

    /// Gets the scrollbar size
    pub fn scrollbar_size(&self) -> i32 {
        assert!(!self.was_deleted());
        unsafe { Fl_Check_Browser_scrollbar_size(self.inner) as i32 }
    }

    /// Sets the scrollbar size
    pub fn set_scrollbar_size(&mut self, new_size: i32) {
        assert!(!self.was_deleted());
        unsafe { Fl_Check_Browser_set_scrollbar_size(self.inner, new_size as i32) }
    }

    /// Sort browser elements
    pub fn sort(&mut self) {
        assert!(!self.was_deleted());
        unsafe { Fl_Check_Browser_sort(self.inner) }
    }

    /// Returns the vertical scrollbar
    pub fn scrollbar(&self) -> crate::valuator::Scrollbar {
        assert!(!self.was_deleted());
        unsafe {
            let ptr = Fl_Check_Browser_scrollbar(self.inner);
            assert!(!ptr.is_null());
            crate::valuator::Scrollbar::from_widget_ptr(ptr as *mut fltk_sys::widget::Fl_Widget)
        }
    }

    /// Returns the horizontal scrollbar
    pub fn hscrollbar(&self) -> crate::valuator::Scrollbar {
        assert!(!self.was_deleted());
        unsafe {
            let ptr = Fl_Check_Browser_hscrollbar(self.inner);
            assert!(!ptr.is_null());
            crate::valuator::Scrollbar::from_widget_ptr(ptr as *mut fltk_sys::widget::Fl_Widget)
        }
    }
}
