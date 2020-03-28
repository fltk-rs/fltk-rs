pub use crate::enums::*;
pub use crate::prelude::*;
use fltk_sys::text::*;
use std::{ffi::{CStr, CString}, mem, os::raw};

/// Creates a non-editable text display widget
#[derive(WidgetTrait, DisplayTrait, Debug, Clone)]
pub struct TextDisplay {
    _inner: *mut Fl_Text_Display,
}

/// Creates an editable text display widget
#[derive(WidgetTrait, DisplayTrait, Debug, Clone)]
pub struct TextEditor {
    _inner: *mut Fl_Text_Editor,
}

impl TextEditor {
    fn init(&mut self) {
        unsafe { Fl_Text_Editor_init(self._inner) }
    }
    /// Create an new TextEditor widget
    pub fn new(x: i32, y: i32, w: i32, h: i32) -> TextEditor {
        let temp = CString::new("").unwrap();
        unsafe {
            let mut x = TextEditor {
                _inner: Fl_Text_Editor_new(x, y, w, h, temp.into_raw() as *const raw::c_char),
            };
            x.init();
            x
        }
    }
    /// Copies the text within the TextEditor widget
    pub fn copy(&self) {
        unsafe {
            kf_copy(self._inner);
        }
    }
    /// Cuts the text within the TextEditor widget
    pub fn cut(&self) {
        unsafe {
            kf_cut(self._inner);
        }
    }
    /// Pastes text from the clipboard into the TextEditor widget
    pub fn paste(&self) {
        unsafe {
            kf_paste(self._inner);
        }
    }
    /// Undo changes in the TextEditor widget
    pub fn undo(&self) {
        unsafe {
            kf_undo(self._inner);
        }
    }
}

impl TextDisplay {
    fn init(&mut self) {
        unsafe { Fl_Text_Display_init(self._inner) }
    }
    /// Create an new TextDisplay widget
    pub fn new(x: i32, y: i32, w: i32, h: i32) -> TextDisplay {
        let temp = CString::new("").unwrap();
        unsafe {
            let mut x = TextDisplay {
                _inner: Fl_Text_Display_new(x, y, w, h, temp.into_raw() as *const raw::c_char),
            };
            x.init();
            x
        }
    }
}
