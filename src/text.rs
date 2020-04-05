pub use crate::prelude::*;
use fltk_sys::text::*;
use crate::image::Image;
use std::{ffi::{CStr, CString}, mem, os::raw};

/// Wraps a text buffer
#[derive(Debug, Clone)]
pub struct TextBuffer {
    _inner: *mut Fl_Text_Buffer,
}

impl TextBuffer {
    pub fn default() -> Self {
        unsafe {
            let text_buffer = Fl_Text_Buffer_new();
            assert!(!text_buffer.is_null());
            TextBuffer {
                _inner: text_buffer,
            }
        }
    }
    pub fn from_ptr(ptr: *mut Fl_Text_Buffer) -> Self {
        TextBuffer {
            _inner: ptr,
        }
    }
    pub fn as_ptr(&self) -> *mut Fl_Text_Buffer {
        self._inner
    }

    pub fn set_text(&mut self, txt: &str) {
        unsafe {
            let txt = CString::new(txt).unwrap();
            Fl_Text_Buffer_set_text(self._inner, txt.into_raw() as *const raw::c_char)
        }
    }

    pub fn text(&self) -> String {
        unsafe {
            let text = Fl_Text_Buffer_text(self._inner);
            assert!(!text.is_null());
            CString::from_raw(text as *mut raw::c_char)
                .to_string_lossy().to_string()
        }
    }

    pub fn append(&mut self, text: &str) {
        let text = CString::new(text).unwrap();
        unsafe {
            Fl_Text_Buffer_append(self._inner, text.into_raw() as *const raw::c_char)
        }
    }

    pub fn length(&self) -> usize {
        unsafe {
            Fl_Text_Buffer_length(self._inner) as usize
        }
    }

    pub fn remove(&mut self, start: usize, end: usize) {
        unsafe {
            Fl_Text_Buffer_remove(self._inner, start as i32, end as i32);
        }
    }
}

impl Drop for TextBuffer {
    fn drop(&mut self) {
        unsafe {
            Fl_Text_Buffer_delete(self._inner)
        }
    }
}

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
    pub size: u32,
}

impl TextEditor {
    /// Create an new TextEditor widget
    pub fn new(x: i32, y: i32, w: i32, h: i32, buf: &mut TextBuffer) -> TextEditor {
        let temp = CString::new("").unwrap();
        unsafe {
            let text_editor = Fl_Text_Editor_new(x, y, w, h, temp.into_raw() as *const raw::c_char);
            assert!(!text_editor.is_null());
            let mut x = TextEditor {
                _inner: text_editor,
            };
            x.set_buffer(buf);
            x
        }
    }
    /// Creates a default and zero initialized TextEditor
    pub fn default(buf: &mut TextBuffer) -> TextEditor {
        let temp = CString::new("").unwrap();
        unsafe {
            let text_editor = Fl_Text_Editor_new(0, 0, 0, 0, temp.into_raw() as *const raw::c_char);
            assert!(!text_editor.is_null());
            let mut x = TextEditor {
                _inner: text_editor,
            };
            x.set_buffer(buf);
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
    /// Create an new TextDisplay widget
    pub fn new(x: i32, y: i32, w: i32, h: i32, buf: &mut TextBuffer) -> TextDisplay {
        let temp = CString::new("").unwrap();
        unsafe {
            let text_display = Fl_Text_Display_new(x, y, w, h, temp.into_raw() as *const raw::c_char);
            assert!(!text_display.is_null());
            let mut x = TextDisplay {
                _inner: text_display,
            };
            x.set_buffer(buf);
            x
        }
    }
    /// Creates a default and zero initialized TextDisplay
    pub fn default(buf: &mut TextBuffer) -> TextDisplay {
        let temp = CString::new("").unwrap();
        unsafe {
            let text_display = Fl_Text_Display_new(0, 0, 0, 0, temp.into_raw() as *const raw::c_char);
            assert!(!text_display.is_null());
            let mut x = TextDisplay {
                _inner: text_display,
            };
            x.set_buffer(buf);
            x
        }
    }
}

impl SimpleTerminal {
    /// Create an new SimpleTerminal widget
    pub fn new(x: i32, y: i32, w: i32, h: i32, buf: &mut TextBuffer)-> SimpleTerminal {
        let temp = CString::new("").unwrap();
        unsafe {
            let simple_terminal = Fl_Simple_Terminal_new(x, y, w, h, temp.into_raw() as *const raw::c_char);
            assert!(!simple_terminal.is_null());
            let mut x = SimpleTerminal {
                _inner: simple_terminal,
            };
            x.set_buffer(buf);
            x
        }
    }
    /// Creates a default and zero initialized SimpleTerminal
    pub fn default(buf: &mut TextBuffer) -> SimpleTerminal {
        let temp = CString::new("").unwrap();
        unsafe {
            let simple_terminal = Fl_Simple_Terminal_new(0, 0, 0, 0, temp.into_raw() as *const raw::c_char);
            assert!(!simple_terminal.is_null());
            let mut x = SimpleTerminal {
                _inner: simple_terminal,
            };
            x.set_buffer(buf);
            x
        }
    }
}
