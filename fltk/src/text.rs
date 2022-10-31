use crate::enums::{Color, Font, Key};
use crate::prelude::*;
use crate::utils::FlString;
use fltk_sys::text::*;
use std::{
    ffi::{CStr, CString},
    os::raw,
    sync::atomic::{AtomicUsize, Ordering},
};

/// Defines the text cursor styles supported by fltk
#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Cursor {
    /// Normal
    Normal,
    /// Caret
    Caret,
    /// Dim
    Dim,
    /// Block
    Block,
    /// Heavy
    Heavy,
    /// Simple
    Simple,
}

/// Wraps a text buffer, Cloning a text buffer invalidates the underlying pointer, thus the no derive(Clone)
#[derive(Debug)]
pub struct TextBuffer {
    inner: *mut Fl_Text_Buffer,
    refcount: AtomicUsize,
}

impl std::default::Default for TextBuffer {
    /// Initialized a default text buffer
    fn default() -> TextBuffer {
        unsafe {
            let text_buffer = Fl_Text_Buffer_new();
            assert!(!text_buffer.is_null());
            TextBuffer {
                inner: text_buffer,
                refcount: AtomicUsize::new(2),
            }
        }
    }
}

impl TextBuffer {
    /// Deletes the `TextBuffer`
    /// # Safety
    /// The buffer shouldn't be deleted while the Display widget still needs it
    pub unsafe fn delete(mut buf: Self) {
        Fl_Text_Buffer_delete(buf.inner);
        buf.inner = std::ptr::null_mut::<Fl_Text_Buffer>();
    }

    /// Deletes the `TextBuffer`
    /// # Safety
    /// The buffer shouldn't be deleted while the Display widget still needs it
    pub unsafe fn delete_buffer(mut buf: TextBuffer) {
        Fl_Text_Buffer_delete(buf.inner);
        buf.inner = std::ptr::null_mut::<Fl_Text_Buffer>();
    }

    /// Initialized a text buffer from a pointer
    /// # Safety
    /// The pointer must be valid
    pub unsafe fn from_ptr(ptr: *mut Fl_Text_Buffer) -> Self {
        assert!(!ptr.is_null());
        TextBuffer {
            inner: ptr,
            refcount: AtomicUsize::new(2),
        }
    }

    /// Returns the inner pointer from a text buffer
    /// # Safety
    /// Can return multiple mutable pointers to the same buffer
    pub unsafe fn as_ptr(&self) -> *mut Fl_Text_Buffer {
        self.inner
    }

    /// Sets the text of the buffer
    pub fn set_text(&mut self, txt: &str) {
        assert!(!self.inner.is_null());
        unsafe {
            let txt = CString::safe_new(txt);
            Fl_Text_Buffer_set_text(self.inner, txt.as_ptr())
        }
    }

    /// Returns the text of the buffer
    pub fn text(&self) -> String {
        assert!(!self.inner.is_null());
        unsafe {
            let text = Fl_Text_Buffer_text(self.inner);
            assert!(!text.is_null());
            CStr::from_ptr(text as *mut raw::c_char)
                .to_string_lossy()
                .to_string()
        }
    }

    /**
        Appends to the buffer.
        To append and scroll to the end of the buffer:
        ```rust,no_run
        use fltk::{prelude::*, *};
        let txt = "Some long text!";
        let buf = text::TextBuffer::default();
        let mut disp = text::TextDisplay::default();
        disp.set_buffer(Some(buf));
        disp.buffer().unwrap().append(txt);
        disp.set_insert_position(disp.buffer().unwrap().length());
        disp.scroll(
            disp.count_lines(0, disp.buffer().unwrap().length(), true),
            0,
        );
        ```
    */
    pub fn append(&mut self, text: &str) {
        assert!(!self.inner.is_null());
        let text = CString::safe_new(text);
        unsafe { Fl_Text_Buffer_append(self.inner, text.as_ptr()) }
    }

    /// Get the length of the buffer
    pub fn length(&self) -> i32 {
        assert!(!self.inner.is_null());
        unsafe { Fl_Text_Buffer_length(self.inner) as i32 }
    }

    /// Removes from the buffer
    pub fn remove(&mut self, start: i32, end: i32) {
        assert!(!self.inner.is_null());
        unsafe {
            Fl_Text_Buffer_remove(self.inner, start as i32, end as i32);
        }
    }

    /// Returns the text within the range
    pub fn text_range(&self, start: i32, end: i32) -> Option<String> {
        assert!(!self.inner.is_null());
        unsafe {
            let x = Fl_Text_Buffer_text_range(self.inner, start as i32, end as i32);
            if x.is_null() {
                None
            } else {
                Some(
                    CStr::from_ptr(x as *mut raw::c_char)
                        .to_string_lossy()
                        .to_string(),
                )
            }
        }
    }

    /// Inserts text into a position
    pub fn insert(&mut self, pos: i32, text: &str) {
        assert!(!self.inner.is_null());
        let text = CString::safe_new(text);
        unsafe { Fl_Text_Buffer_insert(self.inner, pos as i32, text.as_ptr()) }
    }

    /// Replaces text from position `start` to `end`
    pub fn replace(&mut self, start: i32, end: i32, text: &str) {
        assert!(!self.inner.is_null());
        let text = CString::safe_new(text);
        unsafe { Fl_Text_Buffer_replace(self.inner, start as i32, end as i32, text.as_ptr()) }
    }

    /// Copies text from a source buffer into the current buffer
    pub fn copy_from(&mut self, source_buf: &TextBuffer, start: i32, end: i32, to: i32) {
        assert!(!self.inner.is_null());
        unsafe {
            Fl_Text_Buffer_copy(
                self.inner,
                source_buf.as_ptr(),
                start as i32,
                end as i32,
                to as i32,
            )
        }
    }

    /// Copies whole text from a source buffer into a new buffer
    pub fn copy(&self) -> TextBuffer {
        assert!(!self.inner.is_null());
        let mut temp = TextBuffer::default();
        temp.copy_from(self, 0, 0, self.length());
        temp
    }

    /// Performs an undo operation on the buffer
    /// # Errors
    /// Errors on failure to undo
    pub fn undo(&mut self) -> Result<(), FltkError> {
        assert!(!self.inner.is_null());
        unsafe {
            match Fl_Text_Buffer_undo(self.inner, std::ptr::null_mut()) {
                0 => Err(FltkError::Unknown(String::from("Failed to undo"))),
                _ => Ok(()),
            }
        }
    }

    /// Sets whether the buffer can undo
    pub fn can_undo(&mut self, flag: bool) {
        assert!(!self.inner.is_null());
        unsafe { Fl_Text_Buffer_canUndo(self.inner, flag as raw::c_char) }
    }

    /// Loads a file into the buffer
    /// # Errors
    /// Errors on failure to load file
    pub fn load_file<P: AsRef<std::path::Path>>(&mut self, path: P) -> Result<(), FltkError> {
        assert!(!self.inner.is_null());
        if !path.as_ref().exists() {
            return Err(FltkError::Internal(FltkErrorKind::ResourceNotFound));
        }
        let path = path
            .as_ref()
            .to_str()
            .ok_or_else(|| FltkError::Unknown(String::from("Failed to convert path to string")))?;
        let path = CString::new(path)?;
        unsafe {
            match Fl_Text_Buffer_load_file(self.inner, path.as_ptr()) {
                0 => Ok(()),
                _ => Err(FltkError::Internal(FltkErrorKind::ResourceNotFound)),
            }
        }
    }

    /// Saves a buffer into a file
    /// # Errors
    /// Errors on failure to save file
    pub fn save_file<P: AsRef<std::path::Path>>(&mut self, path: P) -> Result<(), FltkError> {
        assert!(!self.inner.is_null());
        let path = path
            .as_ref()
            .to_str()
            .ok_or_else(|| FltkError::Unknown(String::from("Failed to convert path to string")))?;
        let path = CString::new(path)?;
        unsafe {
            match Fl_Text_Buffer_save_file(self.inner, path.as_ptr()) {
                0 => Ok(()),
                _ => Err(FltkError::Internal(FltkErrorKind::ResourceNotFound)),
            }
        }
    }

    /// Returns the tab distance for the buffer
    pub fn tab_distance(&self) -> i32 {
        assert!(!self.inner.is_null());
        unsafe { Fl_Text_Buffer_tab_distance(self.inner) as i32 }
    }

    /// Sets the tab distance
    pub fn set_tab_distance(&mut self, tab_dist: i32) {
        assert!(!self.inner.is_null());
        unsafe { Fl_Text_Buffer_set_tab_distance(self.inner, tab_dist as i32) }
    }

    /// Selects the text from start to end
    pub fn select(&mut self, start: i32, end: i32) {
        assert!(!self.inner.is_null());
        unsafe { Fl_Text_Buffer_select(self.inner, start as i32, end as i32) }
    }

    /// Returns whether text is selected
    pub fn selected(&self) -> bool {
        assert!(!self.inner.is_null());
        unsafe { Fl_Text_Buffer_selected(self.inner) != 0 }
    }

    /// Unselects text
    pub fn unselect(&mut self) {
        assert!(!self.inner.is_null());
        unsafe { Fl_Text_Buffer_unselect(self.inner) }
    }

    /// Returns the selection position
    pub fn selection_position(&self) -> Option<(i32, i32)> {
        assert!(!self.inner.is_null());
        unsafe {
            let mut start = 0;
            let mut end = 0;
            let ret = Fl_Text_Buffer_selection_position(self.inner, &mut start as _, &mut end as _);
            if ret == 0 {
                None
            } else {
                let x = (start as i32, end as i32);
                Some(x)
            }
        }
    }

    /// Returns the selection text
    pub fn selection_text(&self) -> String {
        assert!(!self.inner.is_null());
        unsafe {
            let x = Fl_Text_Buffer_selection_text(self.inner);
            assert!(!x.is_null());
            CStr::from_ptr(x as *mut raw::c_char)
                .to_string_lossy()
                .to_string()
        }
    }

    /// Removes the selection
    pub fn remove_selection(&mut self) {
        assert!(!self.inner.is_null());
        unsafe { Fl_Text_Buffer_remove_selection(self.inner) }
    }

    /// Replaces selection
    pub fn replace_selection(&mut self, text: &str) {
        assert!(!self.inner.is_null());
        let text = CString::safe_new(text);
        unsafe { Fl_Text_Buffer_replace_selection(self.inner, text.as_ptr()) }
    }

    /// Secondary selects the text from start to end
    pub fn secondary_select(&mut self, start: i32, end: i32) {
        assert!(!self.inner.is_null());
        unsafe { Fl_Text_Buffer_secondary_select(self.inner, start as i32, end as i32) }
    }

    /// Returns whether text is secondary selected
    pub fn secondary_selected(&self) -> bool {
        assert!(!self.inner.is_null());
        unsafe { Fl_Text_Buffer_secondary_selected(self.inner) != 0 }
    }

    /// Unselects text (secondary selection)
    pub fn secondary_unselect(&mut self) {
        assert!(!self.inner.is_null());
        unsafe { Fl_Text_Buffer_secondary_unselect(self.inner) }
    }

    /// Returns the secondary selection position
    pub fn secondary_selection_position(&self) -> Option<(i32, i32)> {
        assert!(!self.inner.is_null());
        unsafe {
            let mut start = 0;
            let mut end = 0;
            let ret = Fl_Text_Buffer_secondary_selection_position(
                self.inner,
                &mut start as _,
                &mut end as _,
            );
            if ret == 0 {
                None
            } else {
                let x = (start as i32, end as i32);
                Some(x)
            }
        }
    }

    /// Returns the secondary selection text
    pub fn secondary_selection_text(&self) -> String {
        assert!(!self.inner.is_null());
        unsafe {
            let x = Fl_Text_Buffer_secondary_selection_text(self.inner);
            assert!(!x.is_null());
            CStr::from_ptr(x as *mut raw::c_char)
                .to_string_lossy()
                .to_string()
        }
    }

    /// Removes the secondary selection
    pub fn remove_secondary_selection(&mut self) {
        assert!(!self.inner.is_null());
        unsafe { Fl_Text_Buffer_remove_secondary_selection(self.inner) }
    }

    /// Replaces the secondary selection
    pub fn replace_secondary_selection(&mut self, text: &str) {
        assert!(!self.inner.is_null());
        let text = CString::safe_new(text);
        unsafe { Fl_Text_Buffer_replace_secondary_selection(self.inner, text.as_ptr()) }
    }

    /// Highlights selection
    pub fn highlight(&mut self, start: i32, end: i32) {
        assert!(!self.inner.is_null());
        unsafe { Fl_Text_Buffer_highlight(self.inner, start as i32, end as i32) }
    }

    /// Returns whether text is highlighted
    pub fn is_highlighted(&self) -> bool {
        assert!(!self.inner.is_null());
        unsafe { Fl_Text_Buffer_is_highlighted(self.inner) != 0 }
    }

    /// Unhighlights text
    pub fn unhighlight(&mut self) {
        assert!(!self.inner.is_null());
        unsafe { Fl_Text_Buffer_unhighlight(self.inner) }
    }

    /// Returns the highlight position
    pub fn highlight_position(&self) -> Option<(i32, i32)> {
        assert!(!self.inner.is_null());
        unsafe {
            let mut start = 0;
            let mut end = 0;
            let ret = Fl_Text_Buffer_highlight_position(self.inner, &mut start as _, &mut end as _);
            if ret == 0 {
                None
            } else {
                let x = (start as i32, end as i32);
                Some(x)
            }
        }
    }

    /// Returns the highlighted text
    pub fn highlight_text(&self) -> String {
        assert!(!self.inner.is_null());
        unsafe {
            let x = Fl_Text_Buffer_highlight_text(self.inner);
            assert!(!x.is_null());
            CStr::from_ptr(x as *mut raw::c_char)
                .to_string_lossy()
                .to_string()
        }
    }

    /// Returns the line at pos
    pub fn line_text(&self, pos: i32) -> String {
        assert!(!self.inner.is_null());
        unsafe {
            let x = Fl_Text_Buffer_line_text(self.inner, pos as i32);
            assert!(!x.is_null());
            CStr::from_ptr(x as *mut raw::c_char)
                .to_string_lossy()
                .to_string()
        }
    }

    /// Returns the index of the line's start position at pos
    pub fn line_start(&self, pos: i32) -> i32 {
        assert!(!self.inner.is_null());
        unsafe { Fl_Text_Buffer_line_start(self.inner, pos as i32) as i32 }
    }

    /// Returns the index of the first character of a word at pos
    pub fn word_start(&self, pos: i32) -> i32 {
        assert!(!self.inner.is_null());
        unsafe { Fl_Text_Buffer_word_start(self.inner, pos as i32) as i32 }
    }

    /// Returns the index of the last character of a word at pos
    pub fn word_end(&self, pos: i32) -> i32 {
        assert!(!self.inner.is_null());
        unsafe { Fl_Text_Buffer_word_end(self.inner, pos as i32) as i32 }
    }

    /// Counts the lines from start to end
    pub fn count_lines(&self, start: i32, end: i32) -> i32 {
        assert!(!self.inner.is_null());
        unsafe { Fl_Text_Buffer_count_lines(self.inner, start as i32, end as i32) as i32 }
    }

    /// Calls the modify callbacks
    pub fn call_modify_callbacks(&mut self) {
        assert!(!self.inner.is_null());
        unsafe { Fl_Text_Buffer_call_modify_callbacks(self.inner) }
    }

    /// Adds a modify callback.
    /// callback args:
    /// pos: i32, inserted items: i32, deleted items: i32, restyled items: i32, `deleted_text`
    pub fn add_modify_callback<F: FnMut(i32, i32, i32, i32, &str) + 'static>(&mut self, cb: F) {
        assert!(!self.inner.is_null());
        unsafe {
            unsafe extern "C" fn shim(
                pos: raw::c_int,
                inserted: raw::c_int,
                deleted: raw::c_int,
                restyled: raw::c_int,
                deleted_text: *const raw::c_char,
                data: *mut raw::c_void,
            ) {
                let temp = if deleted_text.is_null() {
                    String::from("")
                } else {
                    CStr::from_ptr(deleted_text).to_string_lossy().to_string()
                };
                let a: *mut Box<dyn FnMut(i32, i32, i32, i32, &str)> =
                    data as *mut Box<dyn for<'r> FnMut(i32, i32, i32, i32, &'r str)>;
                let f: &mut (dyn FnMut(i32, i32, i32, i32, &str)) = &mut **a;
                let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                    f(
                        pos as i32,
                        inserted as i32,
                        deleted as i32,
                        restyled as i32,
                        &temp,
                    )
                }));
            }
            let a: *mut Box<dyn FnMut(i32, i32, i32, i32, &str)> =
                Box::into_raw(Box::new(Box::new(cb)));
            let data: *mut raw::c_void = a as *mut std::ffi::c_void;
            let callback: Fl_Text_Modify_Cb = Some(shim);
            Fl_Text_Buffer_add_modify_callback(self.inner, callback, data);
        }
    }

    /// Removes a modify callback.
    /// callback args:
    /// pos: i32, inserted items: i32, deleted items: i32, restyled items: i32, `deleted_text`
    pub fn remove_modify_callback<F: FnMut(i32, i32, i32, i32, &str) + 'static>(&mut self, cb: F) {
        assert!(!self.inner.is_null());
        unsafe {
            unsafe extern "C" fn shim(
                pos: raw::c_int,
                inserted: raw::c_int,
                deleted: raw::c_int,
                restyled: raw::c_int,
                deleted_text: *const raw::c_char,
                data: *mut raw::c_void,
            ) {
                let temp = if deleted_text.is_null() {
                    String::from("")
                } else {
                    CStr::from_ptr(deleted_text).to_string_lossy().to_string()
                };
                let a: *mut Box<dyn FnMut(i32, i32, i32, i32, &str)> =
                    data as *mut Box<dyn for<'r> FnMut(i32, i32, i32, i32, &'r str)>;
                let f: &mut (dyn FnMut(i32, i32, i32, i32, &str)) = &mut **a;
                let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                    f(
                        pos as i32,
                        inserted as i32,
                        deleted as i32,
                        restyled as i32,
                        &temp,
                    )
                }));
            }
            let a: *mut Box<dyn FnMut(i32, i32, i32, i32, &str)> =
                Box::into_raw(Box::new(Box::new(cb)));
            let data: *mut raw::c_void = a as *mut std::ffi::c_void;
            let callback: Fl_Text_Modify_Cb = Some(shim);
            Fl_Text_Buffer_remove_modify_callback(self.inner, callback, data);
        }
    }

    /// Forward search for a string
    pub fn search_forward(
        &self,
        start_pos: i32,
        search_string: &str,
        match_case: bool,
    ) -> Option<i32> {
        unsafe {
            let search_string = CString::safe_new(search_string);
            let mut found_pos = 0;
            let ret = Fl_Text_Buffer_search_forward(
                self.inner,
                start_pos,
                search_string.as_ptr() as _,
                &mut found_pos as _,
                match_case as _,
            );
            if ret == 0 {
                None
            } else {
                Some(found_pos)
            }
        }
    }

    /// Backward search for a string
    pub fn search_backward(
        &self,
        start_pos: i32,
        search_string: &str,
        match_case: bool,
    ) -> Option<i32> {
        unsafe {
            let search_string = CString::safe_new(search_string);
            let mut found_pos = 0;
            let ret = Fl_Text_Buffer_search_backward(
                self.inner,
                start_pos,
                search_string.as_ptr() as _,
                &mut found_pos as _,
                match_case as _,
            );
            if ret == 0 {
                None
            } else {
                Some(found_pos)
            }
        }
    }

    /// Forward search for a char
    pub fn find_char_forward(&self, start_pos: i32, search_char: char) -> Option<i32> {
        unsafe {
            let mut found_pos = 0;
            let ret = Fl_Text_Buffer_findchar_forward(
                self.inner,
                start_pos,
                search_char as _,
                &mut found_pos as _,
            );
            if ret == 0 {
                None
            } else {
                Some(found_pos)
            }
        }
    }

    /// Backward search for a char
    pub fn find_char_backward(&self, start_pos: i32, search_char: char) -> Option<i32> {
        unsafe {
            let mut found_pos = 0;
            let ret = Fl_Text_Buffer_findchar_backward(
                self.inner,
                start_pos,
                search_char as _,
                &mut found_pos as _,
            );
            if ret == 0 {
                None
            } else {
                Some(found_pos)
            }
        }
    }
}

unsafe impl Sync for TextBuffer {}

unsafe impl Send for TextBuffer {}

impl PartialEq for TextBuffer {
    fn eq(&self, other: &Self) -> bool {
        self.inner == other.inner
    }
}

impl Eq for TextBuffer {}

impl Clone for TextBuffer {
    fn clone(&self) -> TextBuffer {
        assert!(!self.inner.is_null());
        let x = self.refcount.fetch_add(1, Ordering::Relaxed);
        TextBuffer {
            inner: self.inner,
            refcount: AtomicUsize::new(x),
        }
    }
}

impl Drop for TextBuffer {
    fn drop(&mut self) {
        assert!(!self.inner.is_null());
        self.refcount.fetch_sub(1, Ordering::Relaxed);
        if *self.refcount.get_mut() < 1 {
            unsafe {
                Fl_Text_Buffer_delete(self.inner);
            }
            self.inner = std::ptr::null_mut();
        }
    }
}

/// Defines wrap modes
#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum WrapMode {
    /// No wrapping
    None,
    /// Wrap text at certain column
    AtColumn,
    /// Wrap text at certain pixel
    AtPixel,
    /// Wrap text at certain bounds
    AtBounds,
}

/// Defines drag types
#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum DragType {
    /// No dragging
    None = -2,
    /// Drag Start "drag n drop" event
    StartDnd = -1,
    /// Drag single character
    Char = 0,
    /// Drag single word
    Word = 1,
    /// Drag single line
    Line = 2,
}

/// Creates a non-editable text display widget
#[derive(Debug)]
pub struct TextDisplay {
    inner: *mut Fl_Text_Display,
    tracker: *mut fltk_sys::fl::Fl_Widget_Tracker,
    is_derived: bool,
}

crate::macros::widget::impl_widget_ext!(TextDisplay, Fl_Text_Display);
crate::macros::widget::impl_widget_base!(TextDisplay, Fl_Text_Display);
crate::macros::display::impl_display_ext!(TextDisplay, Fl_Text_Display);

/// Creates an editable text display widget
#[derive(Debug)]
pub struct TextEditor {
    inner: *mut Fl_Text_Editor,
    tracker: *mut fltk_sys::fl::Fl_Widget_Tracker,
    is_derived: bool,
}

crate::macros::widget::impl_widget_ext!(TextEditor, Fl_Text_Editor);
crate::macros::widget::impl_widget_base!(TextEditor, Fl_Text_Editor);
crate::macros::display::impl_display_ext!(TextEditor, Fl_Text_Editor);

/// Alias Fl_Text_Editor for use in `add_key_binding`
pub type TextEditorPtr = *mut Fl_Text_Editor;

/// Creates an editable text display widget to handle terminal-like behavior, such as
/// logging events or debug information.
/// `SimpleTerminal` already has an internal buffer.
/// It is NOT is a full terminal emulator; it does NOT
/// handle stdio redirection, pipes, pseudo ttys, termio character cooking,
/// keyboard input processing, screen addressing, random cursor positioning,
/// curses compatibility, or VT100/xterm emulation.
#[derive(Debug)]
pub struct SimpleTerminal {
    inner: *mut Fl_Simple_Terminal,
    tracker: *mut fltk_sys::fl::Fl_Widget_Tracker,
    is_derived: bool,
}

crate::macros::widget::impl_widget_ext!(SimpleTerminal, Fl_Simple_Terminal);
crate::macros::widget::impl_widget_base!(SimpleTerminal, Fl_Simple_Terminal);
crate::macros::display::impl_display_ext!(SimpleTerminal, Fl_Simple_Terminal);

/// The attribute of the style entry
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
#[non_exhaustive]
pub enum TextAttr {
    /// No attribute
    None = 0x0000,
    /// Use the background color in the `bgcolor` field
    BgColor = 0x0001,
    /// Use the background color in the `bgcolor` field to highlight the whole line
    BgColorExt = 0x0003,
    /// A single underline, underline types are mutually exclusive
    Underline = 0x0004,
    /// Grammar suggestion (blue dotted underline)
    Grammar = 0x0008,
    /// Spelling suggestion (red dotted underline)
    Spelling = 0x000C,
    /// Line through the middle of the text
    StrikeThrough = 0x0010,
}

/// Defines the styles used in the `set_highlight_data`, which is used with style buffers
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StyleTableEntry {
    /// Font color
    pub color: Color,
    /// Font type
    pub font: Font,
    /// Font size
    pub size: i32,
}

impl Default for StyleTableEntry {
    fn default() -> Self {
        Self {
            color: Color::Foreground,
            font: Font::Helvetica,
            size: crate::app::font_size(),
        }
    }
}

/// Defines the styles used in the `set_highlight_data`, which is used with style buffers
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StyleTableEntryExt {
    /// Font color
    pub color: Color,
    /// Font type
    pub font: Font,
    /// Font size
    pub size: i32,
    /// attribute
    pub attr: TextAttr,
    /// background color
    pub bgcolor: Color,
}

impl Default for StyleTableEntryExt {
    fn default() -> Self {
        Self {
            color: Color::Foreground,
            font: Font::Helvetica,
            size: crate::app::font_size(),
            attr: TextAttr::None,
            bgcolor: Color::Background2,
        }
    }
}

impl TextEditor {
    /// Any state/shortcut
    pub const AnyState: crate::enums::Shortcut = crate::enums::Shortcut::from_i32(-1);

    /// Set to insert mode
    pub fn set_insert_mode(&mut self, b: bool) {
        assert!(!self.was_deleted());
        assert!(self.buffer().is_some());
        unsafe { Fl_Text_Editor_set_insert_mode(self.inner, b as i32) }
    }

    /// Returns whether insert mode is set
    pub fn insert_mode(&self) -> bool {
        assert!(!self.was_deleted());
        assert!(self.buffer().is_some());
        unsafe { Fl_Text_Editor_insert_mode(self.inner) != 0 }
    }

    /// Set tab navigation
    pub fn set_tab_nav(&mut self, val: bool) {
        assert!(!self.was_deleted());
        assert!(self.buffer().is_some());
        unsafe { Fl_Text_Editor_set_tab_nav(self.inner, val as i32) }
    }

    /// Returns whether tab navigation is set
    pub fn tab_nav(&self) -> bool {
        assert!(!self.was_deleted());
        assert!(self.buffer().is_some());
        unsafe { Fl_Text_Editor_tab_nav(self.inner) != 0 }
    }

    /// Copies the text within the `TextEditor` widget
    pub fn copy(&self) {
        assert!(!self.was_deleted());
        assert!(self.buffer().is_some());
        unsafe {
            Fl_Text_Editor_kf_copy(self.inner);
        }
    }

    /// Cuts the text within the `TextEditor` widget
    pub fn cut(&self) {
        assert!(!self.was_deleted());
        assert!(self.buffer().is_some());
        unsafe {
            Fl_Text_Editor_kf_cut(self.inner);
        }
    }

    /// Pastes text from the clipboard into the `TextEditor` widget
    pub fn paste(&self) {
        assert!(!self.was_deleted());
        assert!(self.buffer().is_some());
        unsafe {
            Fl_Text_Editor_kf_paste(self.inner);
        }
    }

    /// Undo changes in the `TextEditor` widget
    pub fn undo(&self) {
        assert!(!self.was_deleted());
        assert!(self.buffer().is_some());
        unsafe {
            Fl_Text_Editor_kf_undo(self.inner);
        }
    }

    /// Inserts the text associated with key 'c'
    pub fn kf_default(&mut self, c: Key) {
        assert!(!self.was_deleted());
        assert!(self.buffer().is_some());
        unsafe {
            Fl_Text_Editor_kf_default(c.bits() as i32, self.inner);
        }
    }

    /// Ignores the key 'c' in editor
    pub fn kf_ignore(&mut self, c: Key) {
        assert!(!self.was_deleted());
        assert!(self.buffer().is_some());
        unsafe {
            Fl_Text_Editor_kf_ignore(c.bits() as i32, self.inner);
        }
    }

    /// Does a backspace
    pub fn kf_backspace(&mut self) {
        assert!(!self.was_deleted());
        assert!(self.buffer().is_some());
        unsafe {
            Fl_Text_Editor_kf_backspace(self.inner);
        }
    }

    /// Inserts a new line
    pub fn kf_enter(&mut self) {
        assert!(!self.was_deleted());
        assert!(self.buffer().is_some());
        unsafe {
            Fl_Text_Editor_kf_enter(self.inner);
        }
    }

    /// Moves the cursor in the direction indicated by the key
    pub fn kf_move(&mut self, c: Key) {
        assert!(!self.was_deleted());
        assert!(self.buffer().is_some());
        unsafe {
            Fl_Text_Editor_kf_move(c.bits() as i32, self.inner);
        }
    }

    /// Extends the current selection in the direction of key 'c'
    pub fn kf_shift_move(&mut self, c: Key) {
        assert!(!self.was_deleted());
        assert!(self.buffer().is_some());
        unsafe {
            Fl_Text_Editor_kf_shift_move(c.bits() as i32, self.inner);
        }
    }

    /// Moves the current text cursor in the direction indicated by control key 'c'
    pub fn kf_ctrl_move(&mut self, c: Key) {
        assert!(!self.was_deleted());
        assert!(self.buffer().is_some());
        unsafe {
            Fl_Text_Editor_kf_ctrl_move(c.bits() as i32, self.inner);
        }
    }

    /// Extends the current selection in the direction indicated by control key 'c'
    pub fn kf_c_s_move(&mut self, c: Key) {
        assert!(!self.was_deleted());
        assert!(self.buffer().is_some());
        unsafe {
            Fl_Text_Editor_kf_c_s_move(c.bits() as i32, self.inner);
        }
    }

    /// Moves the current text cursor in the direction indicated by meta key 'c'
    pub fn kf_meta_move(&mut self, c: Key) {
        assert!(!self.was_deleted());
        assert!(self.buffer().is_some());
        unsafe {
            Fl_Text_Editor_kf_meta_move(c.bits() as i32, self.inner);
        }
    }

    /// Extends the current selection in the direction indicated by meta key 'c'
    pub fn kf_m_s_move(&mut self, c: Key) {
        assert!(!self.was_deleted());
        assert!(self.buffer().is_some());
        unsafe {
            Fl_Text_Editor_kf_m_s_move(c.bits() as i32, self.inner);
        }
    }

    /// Moves the text cursor to the beginning of the current line
    pub fn kf_home(&mut self) {
        assert!(!self.was_deleted());
        assert!(self.buffer().is_some());
        unsafe {
            Fl_Text_Editor_kf_home(self.inner);
        }
    }

    /// Moves the text cursor to the end of the current line
    pub fn kf_end(&mut self) {
        assert!(!self.was_deleted());
        assert!(self.buffer().is_some());
        unsafe {
            Fl_Text_Editor_kf_end(self.inner);
        }
    }

    /// Moves the text cursor one character to the left
    pub fn kf_left(&mut self) {
        assert!(!self.was_deleted());
        assert!(self.buffer().is_some());
        unsafe {
            Fl_Text_Editor_kf_left(self.inner);
        }
    }

    /// Moves the text cursor one line up
    pub fn kf_up(&mut self) {
        assert!(!self.was_deleted());
        assert!(self.buffer().is_some());
        unsafe {
            Fl_Text_Editor_kf_up(self.inner);
        }
    }

    /// Moves the text cursor one character to the right
    pub fn kf_right(&mut self) {
        assert!(!self.was_deleted());
        assert!(self.buffer().is_some());
        unsafe {
            Fl_Text_Editor_kf_right(self.inner);
        }
    }

    /// Moves the text cursor one line down
    pub fn kf_down(&mut self) {
        assert!(!self.was_deleted());
        assert!(self.buffer().is_some());
        unsafe {
            Fl_Text_Editor_kf_down(self.inner);
        }
    }

    /// Moves the text cursor up one page
    pub fn kf_page_up(&mut self) {
        assert!(!self.was_deleted());
        assert!(self.buffer().is_some());
        unsafe {
            Fl_Text_Editor_kf_page_up(self.inner);
        }
    }

    /// Moves the text cursor down one page
    pub fn kf_page_down(&mut self) {
        assert!(!self.was_deleted());
        assert!(self.buffer().is_some());
        unsafe {
            Fl_Text_Editor_kf_page_down(self.inner);
        }
    }

    /// Toggles the insert mode for the editor
    pub fn kf_insert(&mut self) {
        assert!(!self.was_deleted());
        assert!(self.buffer().is_some());
        unsafe {
            Fl_Text_Editor_kf_insert(self.inner);
        }
    }

    /// Does a delete of selected text or the current character in the current buffer
    pub fn kf_delete(&mut self) {
        assert!(!self.was_deleted());
        assert!(self.buffer().is_some());
        unsafe {
            Fl_Text_Editor_kf_delete(self.inner);
        }
    }

    /// Selects all text in the associated buffer
    pub fn kf_select_all(&mut self) {
        assert!(!self.was_deleted());
        assert!(self.buffer().is_some());
        unsafe {
            Fl_Text_Editor_kf_select_all(self.inner);
        }
    }

    /// Add a key binding
    pub fn add_key_binding(
        &mut self,
        key: crate::enums::Key,
        shortcut: crate::enums::Shortcut,
        cb: fn(key: crate::enums::Key, editor: TextEditorPtr) -> i32,
    ) {
        assert!(!self.was_deleted());
        unsafe {
            Fl_Text_Editor_add_key_binding(
                self.inner,
                key.bits(),
                shortcut.bits(),
                std::mem::transmute(Some(cb)),
            );
        }
    }

    /// Remove a key binding
    pub fn remove_key_binding(&mut self, key: crate::enums::Key, shortcut: crate::enums::Shortcut) {
        assert!(!self.was_deleted());
        unsafe {
            Fl_Text_Editor_remove_key_binding(self.inner, key.bits(), shortcut.bits());
        }
    }
}

impl SimpleTerminal {
    /// Sets whether the terminal automatically stays at the bottom
    pub fn set_stay_at_bottom(&mut self, arg1: bool) {
        assert!(!self.was_deleted());
        assert!(self.buffer().is_some());
        unsafe { Fl_Simple_Terminal_set_stay_at_bottom(self.inner, arg1 as i32) }
    }

    /// Returns whether the terminal automatically stays at the bottom
    pub fn stay_at_bottom(&self) -> bool {
        assert!(!self.was_deleted());
        assert!(self.buffer().is_some());
        unsafe { Fl_Simple_Terminal_stay_at_bottom(self.inner) != 0 }
    }

    /// Sets the max lines allowed in history
    pub fn set_history_lines(&mut self, arg1: i32) {
        assert!(!self.was_deleted());
        assert!(self.buffer().is_some());
        unsafe { Fl_Simple_Terminal_set_history_lines(self.inner, arg1 as i32) }
    }

    /// Gets the max lines allowed in history
    pub fn history_lines(&self) -> i32 {
        assert!(!self.was_deleted());
        assert!(self.buffer().is_some());
        unsafe { Fl_Simple_Terminal_history_lines(self.inner) as i32 }
    }

    /// Enables ANSI sequences within the text to control text colors
    pub fn set_ansi(&mut self, val: bool) {
        assert!(!self.was_deleted());
        assert!(self.buffer().is_some());
        unsafe { Fl_Simple_Terminal_set_ansi(self.inner, val as i32) }
    }

    /// Returns whether ANSI sequences are enabled
    pub fn ansi(&self) -> bool {
        assert!(!self.was_deleted());
        assert!(self.buffer().is_some());
        unsafe { Fl_Simple_Terminal_ansi(self.inner) != 0 }
    }

    /// Appends text to the terminal buffer
    pub fn append(&mut self, s: &str) {
        assert!(!self.was_deleted());
        assert!(self.buffer().is_some());
        let s = CString::safe_new(s);
        unsafe { Fl_Simple_Terminal_append(self.inner, s.into_raw()) }
    }

    /// Appends data to the terminal buffer
    pub fn append2(&mut self, s: &[u8]) {
        assert!(!self.was_deleted());
        assert!(self.buffer().is_some());
        let s = CString::new(s).unwrap();
        unsafe { Fl_Simple_Terminal_append2(self.inner, s.into_raw(), -1) }
    }

    /// Sets the text of the terminal buffer
    pub fn set_text(&mut self, s: &str) {
        assert!(!self.was_deleted());
        assert!(self.buffer().is_some());
        let s = CString::safe_new(s);
        unsafe { Fl_Simple_Terminal_set_text(self.inner, s.into_raw()) }
    }

    /// Gets the text of the terminal buffer
    pub fn text(&self) -> String {
        assert!(!self.was_deleted());
        assert!(self.buffer().is_some());
        unsafe {
            let ptr = Fl_Simple_Terminal_text(self.inner);
            assert!(!ptr.is_null());
            CStr::from_ptr(ptr as *mut raw::c_char)
                .to_string_lossy()
                .to_string()
        }
    }

    /// Clears the terminal
    pub fn clear(&mut self) {
        assert!(!self.was_deleted());
        assert!(self.buffer().is_some());
        unsafe { Fl_Simple_Terminal_clear(self.inner) }
    }

    /// Removes `count` lines from `start`
    pub fn remove_lines(&mut self, start: i32, count: i32) {
        assert!(!self.was_deleted());
        assert!(self.buffer().is_some());
        unsafe { Fl_Simple_Terminal_remove_lines(self.inner, start as i32, count as i32) }
    }
}
