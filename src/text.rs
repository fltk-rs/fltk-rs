use crate::image::Image;
pub use crate::prelude::*;
use fltk_sys::text::*;
use std::{
    ffi::{CStr, CString},
    mem,
    os::raw,
};

/// Wraps a text buffer, Cloning a text buffer invalidates the underlying pointer, thus the no derive(Clone)
#[derive(Debug)]
pub struct TextBuffer {
    _inner: *mut Fl_Text_Buffer,
}

impl TextBuffer {
    /// Initialized a default text buffer
    pub fn default() -> TextBuffer {
        unsafe {
            let text_buffer = Fl_Text_Buffer_new();
            assert!(!text_buffer.is_null());
            TextBuffer {
                _inner: text_buffer,
            }
        }
    }

    /// Deletes the TextBuffer
    /// # Safety
    /// The buffer shouldn't be deleted while the Display widget still needs it
    pub unsafe fn delete(&mut self) {
        Fl_Text_Buffer_delete(self._inner);
        self._inner = std::ptr::null_mut::<Fl_Text_Buffer>();
    }

    /// Initialized a text buffer from a pointer
    /// # Safety
    /// The pointer must be valid
    pub unsafe fn from_ptr(ptr: *mut Fl_Text_Buffer) -> Self {
        assert!(!ptr.is_null());
        TextBuffer { _inner: ptr }
    }

    /// Returns the inner pointer from a text buffer
    /// # Safety
    /// Can return multiple mutable pointers to the same buffer
    pub unsafe fn as_ptr(&self) -> *mut Fl_Text_Buffer {
        self._inner
    }

    /// Sets the text of the buffer
    pub fn set_text(&mut self, txt: &str) {
        assert!(!self._inner.is_null());
        unsafe {
            let txt = CString::new(txt).unwrap();
            Fl_Text_Buffer_set_text(self._inner, txt.as_ptr())
        }
    }

    /// Returns the text of the buffer
    pub fn text(&self) -> String {
        assert!(!self._inner.is_null());
        unsafe {
            let text = Fl_Text_Buffer_text(self._inner);
            assert!(!text.is_null());
            CStr::from_ptr(text as *mut raw::c_char)
                .to_string_lossy()
                .to_string()
        }
    }

    /// Appends to the buffer
    pub fn append(&mut self, text: &str) {
        assert!(!self._inner.is_null());
        let text = CString::new(text).unwrap();
        unsafe { Fl_Text_Buffer_append(self._inner, text.as_ptr()) }
    }

    /// Get the length of the buffer
    pub fn length(&self) -> u32 {
        assert!(!self._inner.is_null());
        unsafe { Fl_Text_Buffer_length(self._inner) as u32 }
    }

    /// Removes from the buffer
    pub fn remove(&mut self, start: u32, end: u32) {
        assert!(!self._inner.is_null());
        debug_assert!(
            start <= std::i32::MAX as u32,
            "u32 entries must be < std::i32::MAX for compatibility!"
        );
        debug_assert!(
            end <= std::i32::MAX as u32,
            "u32 entries must be < std::i32::MAX for compatibility!"
        );
        unsafe {
            Fl_Text_Buffer_remove(self._inner, start as i32, end as i32);
        }
    }

    /// Returns the text within the range
    pub fn text_range(&self, start: u32, end: u32) -> Option<String> {
        assert!(!self._inner.is_null());
        debug_assert!(
            start <= std::i32::MAX as u32,
            "u32 entries must be < std::i32::MAX for compatibility!"
        );
        debug_assert!(
            end <= std::i32::MAX as u32,
            "u32 entries must be < std::i32::MAX for compatibility!"
        );
        unsafe {
            let x = Fl_Text_Buffer_text_range(self._inner, start as i32, end as i32);
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
    pub fn insert(&mut self, pos: u32, text: &str) {
        assert!(!self._inner.is_null());
        debug_assert!(
            pos <= std::i32::MAX as u32,
            "u32 entries must be < std::i32::MAX for compatibility!"
        );
        let text = CString::new(text).unwrap();
        unsafe { Fl_Text_Buffer_insert(self._inner, pos as i32, text.as_ptr()) }
    }

    /// Replaces text from position ```start``` to ```end```
    pub fn replace(&mut self, start: u32, end: u32, text: &str) {
        assert!(!self._inner.is_null());
        debug_assert!(
            start <= std::i32::MAX as u32,
            "u32 entries must be < std::i32::MAX for compatibility!"
        );
        debug_assert!(
            end <= std::i32::MAX as u32,
            "u32 entries must be < std::i32::MAX for compatibility!"
        );
        let text = CString::new(text).unwrap();
        unsafe { Fl_Text_Buffer_replace(self._inner, start as i32, end as i32, text.as_ptr()) }
    }

    /// Copies text from a source buffer into the current buffer
    pub fn copy_from(&mut self, source_buf: &TextBuffer, start: u32, end: u32, to: u32) {
        assert!(!self._inner.is_null());
        debug_assert!(
            start <= std::i32::MAX as u32,
            "u32 entries must be < std::i32::MAX for compatibility!"
        );
        debug_assert!(
            end <= std::i32::MAX as u32,
            "u32 entries must be < std::i32::MAX for compatibility!"
        );
        debug_assert!(
            to <= std::i32::MAX as u32,
            "u32 entries must be < std::i32::MAX for compatibility!"
        );
        unsafe {
            Fl_Text_Buffer_copy(
                self._inner,
                source_buf.as_ptr(),
                start as i32,
                end as i32,
                to as i32,
            )
        }
    }

    /// Copies whole text from a source buffer into a new buffer
    pub fn copy(&self) -> TextBuffer {
        assert!(!self._inner.is_null());
        let mut temp = TextBuffer::default();
        temp.copy_from(self, 0, 0, self.length());
        temp
    }

    /// Performs an undo operation on the buffer
    pub fn undo(&mut self) -> Result<(), FltkError> {
        assert!(!self._inner.is_null());
        unsafe {
            match Fl_Text_Buffer_undo(self._inner, std::ptr::null_mut()) {
                0 => Err(FltkError::Unknown(String::from("Failed to undo"))),
                _ => Ok(()),
            }
        }
    }

    /// Sets whether the buffer can undo
    pub fn can_undo(&mut self, flag: bool) {
        assert!(!self._inner.is_null());
        unsafe { Fl_Text_Buffer_canUndo(self._inner, flag as raw::c_char) }
    }

    /// Loads a file into the buffer
    pub fn load_file(&mut self, path: &std::path::Path) -> Result<(), FltkError> {
        assert!(!self._inner.is_null());
        if !path.exists() {
            return Err(FltkError::Internal(FltkErrorKind::ResourceNotFound));
        }
        let path = path.to_str().unwrap();
        let path = CString::new(path)?;
        unsafe {
            match Fl_Text_Buffer_loadfile(self._inner, path.as_ptr(), 0) {
                0 => Ok(()),
                _ => Err(FltkError::Internal(FltkErrorKind::ResourceNotFound)),
            }
        }
    }

    /// Returns the tab distance for the buffer
    pub fn tab_distance(&self) -> u32 {
        assert!(!self._inner.is_null());
        unsafe { Fl_Text_Buffer_tab_distance(self._inner) as u32 }
    }

    /// Sets the tab distance
    pub fn set_tab_distance(&mut self, tab_dist: u32) {
        assert!(!self._inner.is_null());
        debug_assert!(
            tab_dist <= std::i32::MAX as u32,
            "u32 entries must be < std::i32::MAX for compatibility!"
        );
        unsafe { Fl_Text_Buffer_set_tab_distance(self._inner, tab_dist as i32) }
    }

    /// Selects the text from start to end
    pub fn select(&mut self, start: u32, end: u32) {
        assert!(!self._inner.is_null());
        debug_assert!(
            start <= std::i32::MAX as u32,
            "u32 entries must be < std::i32::MAX for compatibility!"
        );
        debug_assert!(
            end <= std::i32::MAX as u32,
            "u32 entries must be < std::i32::MAX for compatibility!"
        );
        unsafe { Fl_Text_Buffer_select(self._inner, start as i32, end as i32) }
    }

    /// Returns whether text is selected
    pub fn selected(&self) -> bool {
        assert!(!self._inner.is_null());
        unsafe {
            match Fl_Text_Buffer_selected(self._inner) {
                0 => false,
                _ => true,
            }
        }
    }

    /// Unselects text
    pub fn unselect(&mut self) {
        assert!(!self._inner.is_null());
        unsafe { Fl_Text_Buffer_unselect(self._inner) }
    }

    /// Returns the selection position
    pub fn selection_position(&mut self) -> Option<(u32, u32)> {
        assert!(!self._inner.is_null());
        unsafe {
            let start: *mut raw::c_int = std::ptr::null_mut();
            let end: *mut raw::c_int = std::ptr::null_mut();
            let ret = Fl_Text_Buffer_selection_position(self._inner, start, end);
            if ret != 0 {
                let x = (*start as u32, *end as u32);
                Some(x)
            } else {
                None
            }
        }
    }

    /// Returns the selection text
    pub fn selection_text(&mut self) -> String {
        assert!(!self._inner.is_null());
        unsafe {
            let x = Fl_Text_Buffer_selection_text(self._inner);
            assert!(!x.is_null());
            CStr::from_ptr(x as *mut raw::c_char)
                .to_string_lossy()
                .to_string()
        }
    }

    /// Removes the selection
    pub fn remove_selection(&mut self) {
        assert!(!self._inner.is_null());
        unsafe { Fl_Text_Buffer_remove_selection(self._inner) }
    }

    /// Replaces selection
    pub fn replace_selection(&mut self, text: &str) {
        assert!(!self._inner.is_null());
        let text = CString::new(text).unwrap();
        unsafe { Fl_Text_Buffer_replace_selection(self._inner, text.as_ptr()) }
    }

    /// Highlights selection
    pub fn highlight(&mut self, start: u32, end: u32) {
        assert!(!self._inner.is_null());
        debug_assert!(
            start <= std::i32::MAX as u32,
            "u32 entries must be < std::i32::MAX for compatibility!"
        );
        debug_assert!(
            end <= std::i32::MAX as u32,
            "u32 entries must be < std::i32::MAX for compatibility!"
        );
        unsafe { Fl_Text_Buffer_highlight(self._inner, start as i32, end as i32) }
    }

    /// Returns whether text is highlighted
    pub fn is_highlighted(&mut self) -> bool {
        assert!(!self._inner.is_null());
        unsafe {
            match Fl_Text_Buffer_is_highlighted(self._inner) {
                0 => false,
                _ => true,
            }
        }
    }

    /// Unhighlights text
    pub fn unhighlight(&mut self) {
        assert!(!self._inner.is_null());
        unsafe { Fl_Text_Buffer_unhighlight(self._inner) }
    }

    /// Returns the highlight position
    pub fn highlight_position(&mut self) -> Option<(u32, u32)> {
        assert!(!self._inner.is_null());
        unsafe {
            let start: *mut raw::c_int = std::ptr::null_mut();
            let end: *mut raw::c_int = std::ptr::null_mut();
            let ret = Fl_Text_Buffer_highlight_position(self._inner, start, end);
            if ret != 0 {
                let x = (*start as u32, *end as u32);
                Some(x)
            } else {
                None
            }
        }
    }

    /// Returns the highlighted text
    pub fn highlight_text(&mut self) -> String {
        assert!(!self._inner.is_null());
        unsafe {
            let x = Fl_Text_Buffer_highlight_text(self._inner);
            assert!(!x.is_null());
            CStr::from_ptr(x as *mut raw::c_char)
                .to_string_lossy()
                .to_string()
        }
    }

    /// Returns the line at pos
    pub fn line_text(&self, pos: u32) -> String {
        assert!(!self._inner.is_null());
        debug_assert!(
            pos <= std::i32::MAX as u32,
            "u32 entries must be < std::i32::MAX for compatibility!"
        );
        unsafe {
            let x = Fl_Text_Buffer_line_text(self._inner, pos as i32);
            assert!(!x.is_null());
            CStr::from_ptr(x as *mut raw::c_char)
                .to_string_lossy()
                .to_string()
        }
    }

    /// Returns the index of the line's start position at pos
    pub fn line_start(&self, pos: u32) -> u32 {
        assert!(!self._inner.is_null());
        debug_assert!(
            pos <= std::i32::MAX as u32,
            "u32 entries must be < std::i32::MAX for compatibility!"
        );
        unsafe { Fl_Text_Buffer_line_start(self._inner, pos as i32) as u32 }
    }

    /// Returns the index of the first character of a word at pos
    pub fn word_start(&self, pos: u32) -> u32 {
        assert!(!self._inner.is_null());
        debug_assert!(
            pos <= std::i32::MAX as u32,
            "u32 entries must be < std::i32::MAX for compatibility!"
        );
        unsafe { Fl_Text_Buffer_word_start(self._inner, pos as i32) as u32 }
    }

    /// Returns the index of the last character of a word at pos
    pub fn word_end(&self, pos: u32) -> u32 {
        assert!(!self._inner.is_null());
        debug_assert!(
            pos <= std::i32::MAX as u32,
            "u32 entries must be < std::i32::MAX for compatibility!"
        );
        unsafe { Fl_Text_Buffer_word_end(self._inner, pos as i32) as u32 }
    }

    /// Counts the lines from start to end
    pub fn count_lines(&self, start: u32, end: u32) -> u32 {
        assert!(!self._inner.is_null());
        debug_assert!(
            start <= std::i32::MAX as u32,
            "u32 entries must be < std::i32::MAX for compatibility!"
        );
        debug_assert!(
            end <= std::i32::MAX as u32,
            "u32 entries must be < std::i32::MAX for compatibility!"
        );
        unsafe { Fl_Text_Buffer_count_lines(self._inner, start as i32, end as i32) as u32 }
    }

    /// Calls the modify callbacks
    pub fn call_modify_callbacks(&mut self) {
        assert!(!self._inner.is_null());
        unsafe { Fl_Text_Buffer_call_modify_callbacks(self._inner) }
    }

    /// Adds a modify callback
    pub fn add_modify_callback(&mut self, cb: Box<dyn FnMut(u32, u32, u32, u32, &str)>) {
        assert!(!self._inner.is_null());
        unsafe {
            unsafe extern "C" fn shim(
                pos: raw::c_int,
                inserted: raw::c_int,
                deleted: raw::c_int,
                restyled: raw::c_int,
                deleted_text: *const raw::c_char,
                data: *mut raw::c_void,
            ) {
                let temp = if !deleted_text.is_null() {
                    CStr::from_ptr(deleted_text).to_string_lossy().to_string()
                } else {
                    String::from("")
                };
                let a: *mut Box<dyn FnMut(u32, u32, u32, u32, &str)> =
                    data as *mut Box<dyn for<'r> FnMut(u32, u32, u32, u32, &'r str)>;
                let f: &mut (dyn FnMut(u32, u32, u32, u32, &str)) = &mut **a;
                let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                    f(
                        pos as u32,
                        inserted as u32,
                        deleted as u32,
                        restyled as u32,
                        &temp,
                    )
                }));
            }
            let a: *mut Box<dyn FnMut(u32, u32, u32, u32, &str)> = Box::into_raw(Box::new(cb));
            let data: *mut raw::c_void = a as *mut std::ffi::c_void;
            let callback: Fl_Text_Modify_Cb = Some(shim);
            Fl_Text_Buffer_add_modify_callback(self._inner, callback, data);
        }
    }

    /// Removes a modify callback
    pub fn remove_modify_callback(&mut self, cb: Box<dyn FnMut(u32, u32, u32, u32, &str)>) {
        assert!(!self._inner.is_null());
        unsafe {
            unsafe extern "C" fn shim(
                pos: raw::c_int,
                inserted: raw::c_int,
                deleted: raw::c_int,
                restyled: raw::c_int,
                deleted_text: *const raw::c_char,
                data: *mut raw::c_void,
            ) {
                let temp = if !deleted_text.is_null() {
                    CStr::from_ptr(deleted_text).to_string_lossy().to_string()
                } else {
                    String::from("")
                };
                let a: *mut Box<dyn FnMut(u32, u32, u32, u32, &str)> =
                    data as *mut Box<dyn for<'r> FnMut(u32, u32, u32, u32, &'r str)>;
                let f: &mut (dyn FnMut(u32, u32, u32, u32, &str)) = &mut **a;
                let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                    f(
                        pos as u32,
                        inserted as u32,
                        deleted as u32,
                        restyled as u32,
                        &temp,
                    )
                }));
            }
            let a: *mut Box<dyn FnMut(u32, u32, u32, u32, &str)> = Box::into_raw(Box::new(cb));
            let data: *mut raw::c_void = a as *mut std::ffi::c_void;
            let callback: Fl_Text_Modify_Cb = Some(shim);
            Fl_Text_Buffer_remove_modify_callback(self._inner, callback, data);
        }
    }
}

unsafe impl Sync for TextBuffer {}
unsafe impl Send for TextBuffer {}

impl Clone for TextBuffer {
    fn clone(&self) -> TextBuffer {
        assert!(!self._inner.is_null());
        // let mut temp = TextBuffer::default();
        // temp.copy(self, 0, 0, self.length());
        // temp
        TextBuffer {
            _inner: self._inner,
        }
    }
}

/// Creates a non-editable text display widget
#[derive(WidgetExt, DisplayExt, Debug)]
pub struct TextDisplay {
    _inner: *mut Fl_Text_Display,
    _tracker: *mut fltk_sys::fl::Fl_Widget_Tracker,
}

/// Creates an editable text display widget
#[derive(WidgetExt, DisplayExt, Debug)]
pub struct TextEditor {
    _inner: *mut Fl_Text_Editor,
    _tracker: *mut fltk_sys::fl::Fl_Widget_Tracker,
}

/// Creates an editable text display widget
/// SimpleTerminal already has an internal buffer
#[derive(WidgetExt, DisplayExt, Debug)]
pub struct SimpleTerminal {
    _inner: *mut Fl_Simple_Terminal,
    _tracker: *mut fltk_sys::fl::Fl_Widget_Tracker,
}

/// Defines the styles used in the set_highlight_data, which is used with style buffers
#[derive(Debug, Clone, Copy)]
pub struct StyleTableEntry {
    pub color: Color,
    pub font: Font,
    pub size: u32,
}

/// Opaque data containing the style entries
pub struct StyleTables {
    _inner: *mut raw::c_void,
}

impl StyleTables {
    /// Deletes the StyleTables
    /// # Safety
    /// The buffer shouldn't be deleted while the Display widget is still using it
    pub unsafe fn delete(&mut self) {
        Fl_delete_stable(self._inner);
        self._inner = std::ptr::null_mut::<raw::c_void>();
    }
}

impl TextEditor {
    /// Set to insert mode
    pub fn set_insert_mode(&mut self, b: bool) {
        assert!(!self.was_deleted());
        assert!(self.buffer().is_some());
        unsafe { Fl_Text_Editor_set_insert_mode(self._inner, b as i32) }
    }

    /// Returns whether insert mode is set
    pub fn insert_mode(&self) -> bool {
        assert!(!self.was_deleted());
        assert!(self.buffer().is_some());
        unsafe { Fl_Text_Editor_insert_mode(self._inner) != 0 }
    }

    /// Set tab navigation
    pub fn set_tab_nav(&mut self, val: bool) {
        assert!(!self.was_deleted());
        assert!(self.buffer().is_some());
        unsafe { Fl_Text_Editor_set_tab_nav(self._inner, val as i32) }
    }

    /// Returns whether tab navigation is set
    pub fn tab_nav(&self) -> bool {
        assert!(!self.was_deleted());
        assert!(self.buffer().is_some());
        unsafe { Fl_Text_Editor_tab_nav(self._inner) != 0 }
    }

    /// Copies the text within the TextEditor widget
    pub fn copy(&self) {
        assert!(!self.was_deleted());
        assert!(self.buffer().is_some());
        unsafe {
            Fl_Text_Editor_kf_copy(self._inner);
        }
    }

    /// Cuts the text within the TextEditor widget
    pub fn cut(&self) {
        assert!(!self.was_deleted());
        assert!(self.buffer().is_some());
        unsafe {
            Fl_Text_Editor_kf_cut(self._inner);
        }
    }

    /// Pastes text from the clipboard into the TextEditor widget
    pub fn paste(&self) {
        assert!(!self.was_deleted());
        assert!(self.buffer().is_some());
        unsafe {
            Fl_Text_Editor_kf_paste(self._inner);
        }
    }

    /// Undo changes in the TextEditor widget
    pub fn undo(&self) {
        assert!(!self.was_deleted());
        assert!(self.buffer().is_some());
        unsafe {
            Fl_Text_Editor_kf_undo(self._inner);
        }
    }

    /// Inserts the text associated with key 'c'
    pub fn kf_default(&mut self, c: Key) {
        assert!(!self.was_deleted());
        assert!(self.buffer().is_some());
        unsafe {
            Fl_Text_Editor_kf_default(c as i32, self._inner);
        }
    }

    /// Ignores the key 'c' in editor
    pub fn kf_ignore(&mut self, c: Key) {
        assert!(!self.was_deleted());
        assert!(self.buffer().is_some());
        unsafe {
            Fl_Text_Editor_kf_ignore(c as i32, self._inner);
        }
    }

    /// Does a backspace
    pub fn kf_backspace(&mut self) {
        assert!(!self.was_deleted());
        assert!(self.buffer().is_some());
        unsafe {
            Fl_Text_Editor_kf_backspace(self._inner);
        }
    }

    /// Inserts a new line
    pub fn kf_enter(&mut self) {
        assert!(!self.was_deleted());
        assert!(self.buffer().is_some());
        unsafe {
            Fl_Text_Editor_kf_enter(self._inner);
        }
    }

    /// Moves the cursor in the direction indicated by the key
    pub fn kf_move(&mut self, c: Key) {
        assert!(!self.was_deleted());
        assert!(self.buffer().is_some());
        unsafe {
            Fl_Text_Editor_kf_move(c as i32, self._inner);
        }
    }

    /// Extends the current selection in the direction of key 'c'
    pub fn kf_shift_move(&mut self, c: Key) {
        assert!(!self.was_deleted());
        assert!(self.buffer().is_some());
        unsafe {
            Fl_Text_Editor_kf_shift_move(c as i32, self._inner);
        }
    }

    /// Moves the current text cursor in the direction indicated by control key 'c' 
    pub fn kf_ctrl_move(&mut self, c: Key) {
        assert!(!self.was_deleted());
        assert!(self.buffer().is_some());
        unsafe {
            Fl_Text_Editor_kf_ctrl_move(c as i32, self._inner);
        }
    }

    /// Extends the current selection in the direction indicated by control key 'c' 
    pub fn kf_c_s_move(&mut self, c: Key) {
        assert!(!self.was_deleted());
        assert!(self.buffer().is_some());
        unsafe {
            Fl_Text_Editor_kf_c_s_move(c as i32, self._inner);
        }
    }

    /// Moves the current text cursor in the direction indicated by meta key 'c' 
    pub fn kf_meta_move(&mut self, c: Key) {
        assert!(!self.was_deleted());
        assert!(self.buffer().is_some());
        unsafe {
            Fl_Text_Editor_kf_meta_move(c as i32, self._inner);
        }
    }

    /// Extends the current selection in the direction indicated by meta key 'c'
    pub fn kf_m_s_move(&mut self, c: Key) {
        assert!(!self.was_deleted());
        assert!(self.buffer().is_some());
        unsafe {
            Fl_Text_Editor_kf_m_s_move(c as i32, self._inner);
        }
    }

    /// Moves the text cursor to the beginning of the current line
    pub fn kf_home(&mut self) {
        assert!(!self.was_deleted());
        assert!(self.buffer().is_some());
        unsafe {
            Fl_Text_Editor_kf_home(self._inner);
        }
    }

    /// Moves the text cursor to the end of the current line
    pub fn kf_end(&mut self) {
        assert!(!self.was_deleted());
        assert!(self.buffer().is_some());
        unsafe {
            Fl_Text_Editor_kf_end(self._inner);
        }
    }

    /// Moves the text cursor one character to the left
    pub fn kf_left(&mut self) {
        assert!(!self.was_deleted());
        assert!(self.buffer().is_some());
        unsafe {
            Fl_Text_Editor_kf_left(self._inner);
        }
    }

    /// Moves the text cursor one line up 
    pub fn kf_up(&mut self) {
        assert!(!self.was_deleted());
        assert!(self.buffer().is_some());
        unsafe {
            Fl_Text_Editor_kf_up(self._inner);
        }
    }

    /// Moves the text cursor one character to the right
    pub fn kf_right(&mut self) {
        assert!(!self.was_deleted());
        assert!(self.buffer().is_some());
        unsafe {
            Fl_Text_Editor_kf_right(self._inner);
        }
    }

    /// Moves the text cursor one line down 
    pub fn kf_down(&mut self) {
        assert!(!self.was_deleted());
        assert!(self.buffer().is_some());
        unsafe {
            Fl_Text_Editor_kf_down(self._inner);
        }
    }

    /// Moves the text cursor up one page
    pub fn kf_page_up(&mut self) {
        assert!(!self.was_deleted());
        assert!(self.buffer().is_some());
        unsafe {
            Fl_Text_Editor_kf_page_up(self._inner);
        }
    }

    /// Moves the text cursor down one page
    pub fn kf_page_down(&mut self) {
        assert!(!self.was_deleted());
        assert!(self.buffer().is_some());
        unsafe {
            Fl_Text_Editor_kf_page_down(self._inner);
        }
    }

    /// Toggles the insert mode for the editor
    pub fn kf_insert(&mut self) {
        assert!(!self.was_deleted());
        assert!(self.buffer().is_some());
        unsafe {
            Fl_Text_Editor_kf_insert(self._inner);
        }
    }

    /// Does a delete of selected text or the current character in the current buffer
    pub fn kf_delete(&mut self) {
        assert!(!self.was_deleted());
        assert!(self.buffer().is_some());
        unsafe {
            Fl_Text_Editor_kf_delete(self._inner);
        }
    }

    /// Selects all text in the associated buffer
    pub fn kf_select_all(&mut self) {
        assert!(!self.was_deleted());
        assert!(self.buffer().is_some());
        unsafe {
            Fl_Text_Editor_kf_select_all(self._inner);
        }
    }
}

impl SimpleTerminal {
    /// Sets whether the terminal automatically stays at the bottom
    pub fn set_stay_at_bottom(&mut self, arg1: bool) {
        assert!(!self.was_deleted());
        assert!(self.buffer().is_some());
        unsafe { Fl_Simple_Terminal_set_stay_at_bottom(self._inner, arg1 as i32) }
    }

    /// Returns whether the terminal automatically stays at the bottom
    pub fn stay_at_bottom(&self) -> bool {
        assert!(!self.was_deleted());
        assert!(self.buffer().is_some());
        unsafe { Fl_Simple_Terminal_stay_at_bottom(self._inner) != 0 }
    }

    /// Sets the max lines allowed in history
    pub fn set_history_lines(&mut self, arg1: u32) {
        assert!(!self.was_deleted());
        assert!(self.buffer().is_some());
        debug_assert!(
            arg1 <= std::i32::MAX as u32,
            "u32 entries have to be < std::i32::MAX for compatibility!"
        );
        unsafe { Fl_Simple_Terminal_set_history_lines(self._inner, arg1 as i32) }
    }

    /// Gets the max lines allowed in history
    pub fn history_lines(&self) -> u32 {
        assert!(!self.was_deleted());
        assert!(self.buffer().is_some());
        unsafe { Fl_Simple_Terminal_history_lines(self._inner) as u32 }
    }

    /// Enables ANSI sequences within the text to control text colors
    pub fn set_ansi(&mut self, val: bool) {
        assert!(!self.was_deleted());
        assert!(self.buffer().is_some());
        unsafe { Fl_Simple_Terminal_set_ansi(self._inner, val as i32) }
    }

    /// Returns whether ANSI sequences are enabled
    pub fn ansi(&self) -> bool {
        assert!(!self.was_deleted());
        assert!(self.buffer().is_some());
        unsafe { Fl_Simple_Terminal_ansi(self._inner) != 0 }
    }

    /// Appends text to the terminal buffer
    pub fn append(&mut self, s: &str) {
        assert!(!self.was_deleted());
        assert!(self.buffer().is_some());
        let s = CString::new(s).unwrap().into_raw();
        unsafe { Fl_Simple_Terminal_append(self._inner, s) }
    }

    /// Sets the text of the terminal buffer
    pub fn set_text(&mut self, s: &str) {
        assert!(!self.was_deleted());
        assert!(self.buffer().is_some());
        let s = CString::new(s).unwrap().into_raw();
        unsafe { Fl_Simple_Terminal_set_text(self._inner, s) }
    }

    /// Gets the text of the terminal buffer
    pub fn text(&self) -> String {
        assert!(!self.was_deleted());
        assert!(self.buffer().is_some());
        unsafe {
            let ptr = Fl_Simple_Terminal_text(self._inner);
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
        unsafe { Fl_Simple_Terminal_clear(self._inner) }
    }

    /// Removes `count` lines from `start`
    pub fn remove_lines(&mut self, start: u32, count: u32) {
        assert!(!self.was_deleted());
        assert!(self.buffer().is_some());
        debug_assert!(
            start <= std::i32::MAX as u32,
            "u32 entries have to be < std::i32::MAX for compatibility!"
        );
        debug_assert!(
            count <= std::i32::MAX as u32,
            "u32 entries have to be < std::i32::MAX for compatibility!"
        );
        unsafe { Fl_Simple_Terminal_remove_lines(self._inner, start as i32, count as i32) }
    }
}

#[cfg(test)]
mod editor {
    #[test]
    fn buffer() {}
}
