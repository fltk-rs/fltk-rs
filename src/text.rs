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

/// Creates an editable text display widget
#[derive(WidgetTrait, DisplayTrait, Debug, Clone)]
pub struct SimpleTerminal {
    _inner: *mut Fl_Simple_Terminal,
}

#[derive(Debug, Clone, Copy)]
pub struct StyleTableEntry {
    pub color: Color,
    pub font: Font,
    pub size: usize,
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
    /// Creates a default and zero initialized TextEditor
    pub fn default() -> TextEditor {
        let temp = CString::new("").unwrap();
        unsafe {
            let mut x = TextEditor {
                _inner: Fl_Text_Editor_new(0, 0, 0, 0, temp.into_raw() as *const raw::c_char),
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
    /// Creates a default and zero initialized TextDisplay
    pub fn default() -> TextDisplay {
        let temp = CString::new("").unwrap();
        unsafe {
            let mut x = TextDisplay {
                _inner: Fl_Text_Display_new(0, 0, 0, 0, temp.into_raw() as *const raw::c_char),
            };
            x.init();
            x
        }
    }
}

impl SimpleTerminal {
    fn init(&mut self) {
        unsafe { Fl_Simple_Terminal_init(self._inner) }
    }
    /// Create an new SimpleTerminal widget
    pub fn new(x: i32, y: i32, w: i32, h: i32) -> SimpleTerminal {
        let temp = CString::new("").unwrap();
        unsafe {
            let mut x = SimpleTerminal {
                _inner: Fl_Simple_Terminal_new(x, y, w, h, temp.into_raw() as *const raw::c_char),
            };
            x.init();
            x
        }
    }
    /// Creates a default and zero initialized SimpleTerminal
    pub fn default() -> SimpleTerminal {
        let temp = CString::new("").unwrap();
        unsafe {
            let mut x = SimpleTerminal {
                _inner: Fl_Simple_Terminal_new(0, 0, 0, 0, temp.into_raw() as *const raw::c_char),
            };
            x.init();
            x
        }
    }
}
