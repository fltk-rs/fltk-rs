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


/// Creates a CheckBrowser widget
#[derive(WidgetBase, WidgetExt, Debug)]
pub struct CheckBrowser {
    _inner: *mut Fl_Check_Browser,
    _tracker: *mut fltk_sys::fl::Fl_Widget_Tracker,
}

impl CheckBrowser {
    /// Add an item, returns the number of current items
    pub fn add(&mut self, s: &str, checked: bool) -> i32 {
        assert!(!self.was_deleted());
        let s = CString::safe_new(s);
        unsafe {
            Fl_Check_Browser_add(self._inner, s.as_ptr(), checked as i32)
        }
    }

    /// Remove item at index, returns the number of current items
    pub fn remove(&mut self, item: usize) -> i32 {
        assert!(!self.was_deleted());
        unsafe {
            Fl_Check_Browser_remove(self._inner, item as i32)
        }
    }

    /// Clear the browser
    pub fn clear(&mut self) {
        assert!(!self.was_deleted());
        unsafe {
            Fl_Check_Browser_clear(self._inner)
        }
    }

    /// Return the number of items
    pub fn nitems(&self) -> usize {
        assert!(!self.was_deleted());
        unsafe {
            Fl_Check_Browser_nitems(self._inner) as usize
        }
    }

    /// Get the number of checked items
    pub fn nchecked(&self) -> usize {
        assert!(!self.was_deleted());
        unsafe {
            Fl_Check_Browser_nchecked(self._inner) as usize
        }
    }

    /// Returns whether an item is checked
    pub fn checked(&self, item: i32) -> bool {
        assert!(!self.was_deleted());
        unsafe {
            Fl_Check_Browser_checked(self._inner, item) != 0
        }
    }

    /// Check selected item
    pub fn set_checked(&mut self, item: i32) {
        assert!(!self.was_deleted());
        unsafe {
            Fl_Check_Browser_set_checked(self._inner, item)
        }
    }

    /// Ckeck all of the items
    pub fn check_all(&mut self) {
        assert!(!self.was_deleted());
        unsafe {
            Fl_Check_Browser_check_all(self._inner)
        }
    }

    /// Check none of the items
    pub fn check_none(&mut self) {
        assert!(!self.was_deleted());
        unsafe {
            Fl_Check_Browser_check_none(self._inner)
        }
    }

    /// Get the currently selected item
    pub fn value(&self) -> usize {
        assert!(!self.was_deleted());
        unsafe {
            Fl_Check_Browser_value(self._inner) as usize
        }
    }

    /// Get the text of the item
    pub fn text(&self, item: i32) -> Option<String> {
        assert!(!self.was_deleted());
        unsafe {
            let ptr = Fl_Check_Browser_text(self._inner, item);
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
        unsafe { std::mem::transmute(Fl_Check_Browser_textfont(self._inner)) }
    }

    /// Sets the text font
    pub fn set_text_font(&mut self, f: Font) {
        assert!(!self.was_deleted());
        unsafe { Fl_Check_Browser_set_textfont(self._inner, f.bits() as i32) }
    }

    /// Gets the text size
    pub fn text_size(&self) -> u32 {
        assert!(!self.was_deleted());
        unsafe { Fl_Check_Browser_textsize(self._inner) as u32 }
    }

    /// Sets the text size
    pub fn set_text_size(&mut self, s: u32) {
        debug_assert!(
            s <= std::isize::MAX as u32,
            "u32 entries have to be < std::isize::MAX for compatibility!"
        );
        assert!(!self.was_deleted());
        unsafe { Fl_Check_Browser_set_textsize(self._inner, s as i32) }
    }

    /// Gets the text's color
    pub fn text_color(&self) -> Color {
        assert!(!self.was_deleted());
        unsafe { std::mem::transmute(Fl_Check_Browser_textcolor(self._inner)) }
    }

    /// Sets the text's color
    pub fn set_text_color(&mut self, color: Color) {
        assert!(!self.was_deleted());
        unsafe { Fl_Check_Browser_set_textcolor(self._inner, color.bits() as u32) }
    }

    /// Gets the vertical scroll position of the list as a pixel position
    pub fn position(&self) -> u32 {
        assert!(!self.was_deleted());
        unsafe {
            Fl_Check_Browser_position(self._inner) as u32
        }
    }

    /// Sets the vertical scroll position of the list as a pixel position
    pub fn set_position(&mut self, pos: u32) {
        assert!(!self.was_deleted());
        debug_assert!(pos <= std::isize::MAX as u32, "u32 entries have to be < std::isize::MAX for compatibility!");
        unsafe {
            Fl_Check_Browser_set_position(self._inner, pos as i32)
        }
    }

    /// Gets the horizontal scroll position of the list as a pixel position
    pub fn hposition(&self) -> u32 {
        assert!(!self.was_deleted());
        unsafe {
            Fl_Check_Browser_hposition(self._inner) as u32
        }
    }

    /// Sets the horizontal scroll position of the list as a pixel position
    pub fn set_hposition(&mut self, pos: u32) {
        assert!(!self.was_deleted());
        debug_assert!(pos <= std::isize::MAX as u32, "u32 entries have to be < std::isize::MAX for compatibility!");
        unsafe {
            Fl_Check_Browser_set_hposition(self._inner, pos as i32)
        }
    }

    /// Returns the type of scrollbar associated with the browser
    pub fn has_scrollbar(&self) -> BrowserScrollbar {
        assert!(!self.was_deleted());
        unsafe {
            mem::transmute(Fl_Check_Browser_has_scrollbar(self._inner))
        }
    }

    /// Sets the type of scrollbar associated with the browser
    pub fn set_has_scrollbar(&mut self, mode: BrowserScrollbar) {
        assert!(!self.was_deleted());
        unsafe {
            Fl_Check_Browser_set_has_scrollbar(self._inner, mode as raw::c_uchar)
        }
    }
    
    /// Gets the scrollbar size
    pub fn scrollbar_size(&self) -> u32 {
        assert!(!self.was_deleted());
        unsafe {
            Fl_Check_Browser_scrollbar_size(self._inner) as u32
        }
    }

    /// Sets the scrollbar size
    pub fn set_scrollbar_size(&mut self, new_size: u32) {
        assert!(!self.was_deleted());
        debug_assert!(new_size <= std::isize::MAX as u32, "u32 entries have to be < std::isize::MAX for compatibility!");
        unsafe {
            Fl_Check_Browser_set_scrollbar_size(self._inner, new_size as i32)
        }
    }

    /// Sort browser elements
    pub fn sort(&mut self) {
        assert!(!self.was_deleted());
        unsafe {
            Fl_Check_Browser_sort(self._inner)
        }
    }

    /// Returns the vertical scrollbar
    pub fn scrollbar(&self) -> Box<dyn ValuatorExt> {
        assert!(!self.was_deleted());
        unsafe {
            let ptr = Fl_Check_Browser_scrollbar(self._inner);
            assert!(!ptr.is_null());
            Box::new(crate::valuator::Scrollbar::from_widget_ptr(ptr as *mut fltk_sys::widget::Fl_Widget))
        }
    }

    /// Returns the horizontal scrollbar
    pub fn hscrollbar(&self) -> Box<dyn ValuatorExt> {
        assert!(!self.was_deleted());
        unsafe {
            let ptr = Fl_Check_Browser_hscrollbar(self._inner);
            assert!(!ptr.is_null());
            Box::new(crate::valuator::Scrollbar::from_widget_ptr(ptr as *mut fltk_sys::widget::Fl_Widget))
        }
    }
}