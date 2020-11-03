use crate::image::Image;
pub use crate::prelude::*;
use fltk_sys::browser::*;
use std::{
    ffi::{CStr, CString},
    mem,
    os::raw,
};

/// Creates a normal browser
#[derive(WidgetBase, WidgetExt, BrowserExt, Debug)]
pub struct Browser {
    _inner: *mut Fl_Browser,
    _tracker: *mut fltk_sys::fl::Fl_Widget_Tracker,
}

/// Defines the browser type, which can be changed dynamically using the set_type function().
#[repr(i32)]
#[derive(WidgetType, Debug, Copy, Clone, PartialEq)]
pub enum BrowserType {
    Normal = 0,
    Select = 1,
    Hold = 2,
    Multi = 3,
}

/// Defines the type of Scrollbar associated with the browser
#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum BrowserScrollbar {
    None = 0,
    Horizontal = 1,
    Vertical = 2,
    Both = 3,
    AlwaysOn = 4,
    HorizontalAlways = 5,
    VerticalAlways = 6,
    BothAlways = 7,
}

/// Creates a select browser
#[derive(WidgetBase, WidgetExt, BrowserExt, Debug)]
pub struct SelectBrowser {
    _inner: *mut Fl_Select_Browser,
    _tracker: *mut fltk_sys::fl::Fl_Widget_Tracker,
}

/// Creates a multi-browser
#[derive(WidgetBase, WidgetExt, BrowserExt, Debug)]
pub struct MultiBrowser {
    _inner: *mut Fl_Multi_Browser,
    _tracker: *mut fltk_sys::fl::Fl_Widget_Tracker,
}

/// Creates a hold browser
#[derive(WidgetBase, WidgetExt, BrowserExt, Debug)]
pub struct HoldBrowser {
    _inner: *mut Fl_Hold_Browser,
    _tracker: *mut fltk_sys::fl::Fl_Widget_Tracker,
}

/// Creates a file browser
#[derive(WidgetBase, WidgetExt, BrowserExt, Debug)]
pub struct FileBrowser {
    _inner: *mut Fl_File_Browser,
    _tracker: *mut fltk_sys::fl::Fl_Widget_Tracker,
}

#[repr(i32)]
#[derive(Copy, Clone, Debug)]
pub enum FileType {
    Files = 0,
    Dirs,
}

impl FileBrowser {
    /// Gets the icon size
    pub fn iconsize(&self) -> u32 {
        assert!(!self.was_deleted());
        unsafe { Fl_File_Browser_iconsize(self._inner) }
    }

    /// Sets the icon size
    pub fn set_iconsize(&mut self, s: u32) {
        assert!(!self.was_deleted());
        unsafe { Fl_File_Browser_set_iconsize(self._inner, s) }
    }

    /// Sets the filter for the FileBrowser
    /// The following syntax is used for the pattern:
    /// * matches any sequence of 0 or more characters.
    /// ? matches any single character.
    /// [set] matches any character in the set. Set can contain any single characters, or a-z to represent a range. To match ] or - they must be the first characters. To match ^ or ! they must not be the first characters.
    /// [^set] or [!set] matches any character not in the set.
    /// {X|Y|Z} or {X,Y,Z} matches any one of the subexpressions literally.
    /// \x quotes the character x so it has no special meaning.
    /// x all other characters must be matched exactly.
    pub fn set_filter(&mut self, pattern: &str) {
        assert!(!self.was_deleted());
        let pattern = CString::safe_new(pattern);
        unsafe {
            // This is deleted on the C++ side
            Fl_File_Browser_set_filter(self._inner, pattern.into_raw())
        }
    }

    /// Gets the filter for the FileBrowser
    pub fn filter(&self) -> Option<String> {
        assert!(!self.was_deleted());
        unsafe {
            let ptr = Fl_File_Browser_filter(self._inner);
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

    /// Gets the FileType of the FileBrowser
    pub fn filetype(&self) -> FileType {
        assert!(!self.was_deleted());
        unsafe { mem::transmute(Fl_File_Browser_filetype(self._inner)) }
    }

    /// Sets the FileType of the FileBrowser
    pub fn set_filetype(&mut self, t: FileType) {
        assert!(!self.was_deleted());
        unsafe { Fl_File_Browser_set_filetype(self._inner, t as i32) }
    }
}
