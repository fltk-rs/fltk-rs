pub use crate::enums::*;
pub use crate::prelude::*;
use fltk_sys::text::*;
use std::{ffi::CString, mem, os::raw};

/// Creates a non-editable text display widget
#[derive(WidgetTrait, Debug, Clone)]
pub struct TextDisplay {
    _inner: *mut Fl_Text_Display,
}

/// Creates an editable text display widget
#[derive(WidgetTrait, Debug, Clone)]
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
    /// Sets the text inside the TextEditor widget
    pub fn set_text(&mut self, txt: &str) {
        unsafe {
            let txt = CString::new(txt).unwrap();
            Fl_Text_Editor_set_text(self._inner, txt.into_raw() as *const raw::c_char)
        }
    }
    /// Gets the text inside the TextEditor widget
    pub fn text(&self) -> String {
        unsafe {
            CString::from_raw(Fl_Text_Editor_text(self._inner) as *mut raw::c_char)
                .into_string()
                .unwrap()
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
    /// Return the text font
    pub fn text_font(&self) -> Font {
        unsafe { mem::transmute(text_font(self._inner as *mut Fl_Text_Display)) }
    }
    /// Sets the text font
    pub fn set_text_font(&mut self, font: Font) {
        unsafe { set_text_font(self._inner as *mut Fl_Text_Display, font as i32) }
    }
    /// Return the text color
    pub fn text_color(&self) -> Color {
        unsafe { mem::transmute(text_color(self._inner as *mut Fl_Text_Display)) }
    }
    /// Sets the text color
    pub fn set_text_color(&mut self, color: Color) {
        unsafe { set_text_color(self._inner as *mut Fl_Text_Display, color as i32) }
    }
    /// Return the text size
    pub fn text_size(&self) -> usize {
        unsafe { text_size(self._inner as *mut Fl_Text_Display) as usize }
    }
    /// Sets the text size
    pub fn set_text_size(&mut self, sz: usize) {
        unsafe { set_text_size(self._inner as *mut Fl_Text_Display, sz as i32) }
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
    /// Set the text inside the TextDisplay
    pub fn set_text(&mut self, txt: &str) {
        unsafe {
            let txt = CString::new(txt).unwrap();
            Fl_Text_Display_set_text(self._inner, txt.into_raw() as *const raw::c_char)
        }
    }
    /// Returns the text inside the TextDisplay
    pub fn text(&self) -> String {
        unsafe {
            CString::from_raw(Fl_Text_Display_text(self._inner) as *mut raw::c_char)
                .into_string()
                .unwrap()
        }
    }
    /// Return the text font
    pub fn text_font(&self) -> Font {
        unsafe { mem::transmute(text_font(self._inner)) }
    }
    /// Sets the text font
    pub fn set_text_font(&mut self, font: Font) {
        unsafe { set_text_font(self._inner, font as i32) }
    }
    /// Return the text color
    pub fn text_color(&self) -> Color {
        unsafe { mem::transmute(text_color(self._inner)) }
    }
    /// Sets the text color
    pub fn set_text_color(&mut self, color: Color) {
        unsafe { set_text_color(self._inner, color as i32) }
    }
    /// Return the text size
    pub fn text_size(&self) -> usize {
        unsafe { text_size(self._inner) as usize }
    }
    /// Sets the text size
    pub fn set_text_size(&mut self, sz: usize) {
        unsafe { set_text_size(self._inner, sz as i32) }
    }
}
