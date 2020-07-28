pub use crate::prelude::*;
use fltk_sys::dialog::*;
use std::{
    ffi::{CStr, CString},
    mem,
    os::raw,
};

/// Creates a file button
#[derive(Debug)]
pub struct FileDialog {
    _inner: *mut Fl_Native_File_Chooser,
}

/// Defines the type of dialog, which can be changed dynamically using the set_type() method
#[repr(i32)]
#[derive(WidgetType, Debug, Copy, Clone, PartialEq)]
pub enum FileDialogType {
    BrowseFile = 0,
    BrowseDir,
    BrowseMultiFile,
    BrowseMultiDir,
    BrowseSaveFile,
    BrowseSaveDir,
}

/// Defines the File dialog options, which can be set using the set_option() method.
#[repr(i32)]
#[derive(WidgetType, Debug, Copy, Clone, PartialEq)]
pub enum FileDialogOptions {
    NoOptions = 0,
    SaveAsConfirm = 1,
    NewFolder = 2,
    Preview = 4,
    UseFilterExt = 8,
}

impl FileDialog {
    /// Creates an new file dialog
    pub fn new(op: FileDialogType) -> FileDialog {
        unsafe {
            let file_dialog = Fl_Native_File_Chooser_new(mem::transmute(op));
            assert!(!file_dialog.is_null());
            FileDialog {
                _inner: file_dialog,
            }
        }
    }

    /// Returns the chosen file name
    pub fn filename(&self) -> std::path::PathBuf {
        unsafe {
            let cnt = Fl_Native_File_Chooser_count(self._inner);
            if cnt == 0 {
                return std::path::PathBuf::from("");
            }
            let x = Fl_Native_File_Chooser_filenames(self._inner, 0);
            std::path::PathBuf::from(
                CStr::from_ptr(x as *mut raw::c_char)
                    .to_string_lossy()
                    .to_string(),
            )
        }
    }

    /// Returns the chosen file names
    pub fn filenames(&self) -> Vec<std::path::PathBuf> {
        unsafe {
            let cnt = Fl_Native_File_Chooser_count(self._inner);
            let mut names: Vec<std::path::PathBuf> = vec![];
            if cnt == 0 {
                names
            } else {
                for i in 0..cnt {
                    let x = Fl_Native_File_Chooser_filenames(self._inner, i);
                    names.push(std::path::PathBuf::from(
                        CStr::from_ptr(x as *mut raw::c_char)
                            .to_string_lossy()
                            .to_string(),
                    ))
                }
                names
            }
        }
    }

    /// Returns the preset directory
    pub fn directory(&self) -> std::path::PathBuf {
        unsafe {
            let x = Fl_Native_File_Chooser_directory(self._inner);
            if !x.is_null() {
                std::path::PathBuf::from(
                    CStr::from_ptr(x as *mut raw::c_char)
                        .to_string_lossy()
                        .to_string(),
                )
            } else {
                std::path::PathBuf::from("")
            }
        }
    }

    /// Sets the starting directory
    pub fn set_directory(&mut self, dir: &std::path::Path) {
        let dir = CString::new(dir.to_str().unwrap()).unwrap();
        unsafe { Fl_Native_File_Chooser_set_directory(self._inner, dir.as_ptr()) }
    }

    /// Shows the file dialog
    pub fn show(&mut self) {
        unsafe {
            Fl_Native_File_Chooser_show(self._inner);
        }
    }

    /// Sets the option for the dialog
    pub fn set_option(&mut self, opt: FileDialogOptions) {
        unsafe { Fl_Native_File_Chooser_set_option(self._inner, opt as i32) }
    }

    /// Sets the type for the dialog
    pub fn set_type(&mut self, op: FileDialogType) {
        unsafe { Fl_Native_File_Chooser_set_type(self._inner, op as i32) }
    }

    /// Sets the title for the dialog
    pub fn set_title(&mut self, title: &str) {
        let title = CString::new(title).unwrap();
        unsafe { Fl_Native_File_Chooser_set_title(self._inner, title.as_ptr()) }
    }

    /// Sets the filter for the dialog, can be:
    /// A single wildcard (eg. "*.txt")
    /// Multiple wildcards (eg. "*.{cxx,h,H}")
    /// A descriptive name followed by a "\t" and a wildcard (eg. "Text Files\t*.txt")
    /// A list of separate wildcards with a "\n" between each (eg. "*.{cxx,H}\n*.txt")
    /// A list of descriptive names and wildcards (eg. "C++ Files\t*.{cxx,H}\nTxt Files\t*.txt")
    pub fn set_filter(&mut self, f: &str) {
        let f = CString::new(f).unwrap();
        unsafe { Fl_Native_File_Chooser_set_filter(self._inner, f.as_ptr()) }
    }

    /// Sets the preset filter for the dialog
    pub fn set_preset_file(&mut self, f: &str) {
        let f = CString::new(f).unwrap();
        unsafe { Fl_Native_File_Chooser_set_preset_file(self._inner, f.as_ptr()) }
    }

    /// returns the error message from the file dialog
    pub fn error_message(&self) -> Option<String> {
        unsafe {
            let err_msg = Fl_Native_File_Chooser_errmsg(self._inner);
            if err_msg.is_null() {
                None
            } else {
                Some(
                    CStr::from_ptr(err_msg as *mut raw::c_char)
                        .to_string_lossy()
                        .to_string(),
                )
            }
        }
    }
}

impl Drop for FileDialog {
    fn drop(&mut self) {
        unsafe { Fl_Native_File_Chooser_delete(self._inner) }
    }
}

/// Displays a message box
pub fn message(x: i32, y: i32, txt: &str) {
    unsafe {
        let txt = CString::new(txt).unwrap();
        Fl_message(x, y, txt.as_ptr())
    }
}

/// Displays an alert box
pub fn alert(x: i32, y: i32, txt: &str) {
    unsafe {
        let txt = CString::new(txt).unwrap();
        Fl_alert(x, y, txt.as_ptr())
    }
}

/// Displays a choice box with upto three choices
/// An empty choice will not be shown
pub fn choice(x: i32, y: i32, txt: &str, b0: &str, b1: &str, b2: &str) -> u32 {
    unsafe {
        let txt = CString::new(txt).unwrap();
        let b0 = CString::new(b0).unwrap();
        let b1 = CString::new(b1).unwrap();
        let b2 = CString::new(b2).unwrap();
        Fl_choice(x, y, txt.as_ptr(), b0.as_ptr(), b1.as_ptr(), b2.as_ptr()) as u32
    }
}

/// Displays an input box, which returns the inputted string.
/// Can be used for gui io
pub fn input(x: i32, y: i32, txt: &str, deflt: &str) -> Option<String> {
    unsafe {
        let temp = CString::new(deflt).unwrap();
        let txt = CString::new(txt).unwrap();
        let x = Fl_input(x, y, txt.as_ptr(), temp.as_ptr());
        if x.is_null() {
            None
        } else {
            Some(
                CStr::from_ptr(x as *const raw::c_char)
                    .to_string_lossy()
                    .to_string(),
            )
        }
    }
}

/// Shows an input box, but with hidden string
pub fn password(x: i32, y: i32, txt: &str, deflt: &str) -> Option<String> {
    unsafe {
        let temp = CString::new(deflt).unwrap();
        let txt = CString::new(txt).unwrap();
        let x = Fl_password(x, y, txt.as_ptr(), temp.as_ptr());
        if x.is_null() {
            None
        } else {
            Some(
                CStr::from_ptr(x as *const raw::c_char)
                    .to_string_lossy()
                    .to_string(),
            )
        }
    }
}

/// Creates a help dialog
#[derive(Debug)]
pub struct HelpDialog {
    _inner: *mut Fl_Help_Dialog,
}

impl HelpDialog {
    /// Creates a default (size and location) help dialog
    pub fn default() -> HelpDialog {
        unsafe {
            let help_dialog = Fl_Help_Dialog_new();
            assert!(!help_dialog.is_null());
            HelpDialog {
                _inner: help_dialog,
            }
        }
    }

    /// Creates a new Help dialog with position(x, y) and size(w, h)
    pub fn new(x: i32, y: i32, w: i32, h: i32) -> HelpDialog {
        let mut temp = HelpDialog::default();
        temp.resize(x, y, w, h);
        temp
    }

    /// Hides the help dialog
    pub fn hide(&mut self) {
        unsafe { Fl_Help_Dialog_hide(self._inner) }
    }

    /// Loads a file for the help dialog
    pub fn load(&mut self, file: &std::path::Path) -> Result<(), FltkError> {
        let f = file.to_str().unwrap();
        let f = CString::new(f)?;
        unsafe {
            match Fl_Help_Dialog_load(self._inner, f.as_ptr()) {
                0 => Ok(()),
                _ => Err(FltkError::Internal(FltkErrorKind::ResourceNotFound)),
            }
        }
    }

    /// Sets the position of the help dialog
    pub fn position(&mut self, x: i32, y: i32) {
        unsafe { Fl_Help_Dialog_position(self._inner, x, y) }
    }

    /// Resizes the help dialog
    pub fn resize(&mut self, x: i32, y: i32, w: i32, h: i32) {
        unsafe { Fl_Help_Dialog_resize(self._inner, x, y, w, h) }
    }

    /// Shows the help dialog
    pub fn show(&mut self) {
        unsafe { Fl_Help_Dialog_show(self._inner) }
    }

    /// Sets the text size
    pub fn set_text_size(&mut self, s: u32) {
        unsafe { Fl_Help_Dialog_set_text_size(self._inner, s as i32) }
    }

    /// Returns the text size
    pub fn text_size(&mut self) -> u32 {
        unsafe { Fl_Help_Dialog_text_size(self._inner) as u32 }
    }

    /// Sets the value of the help dialog
    pub fn set_value(&mut self, f: &str) {
        let f = CString::new(f).unwrap();
        unsafe { Fl_Help_Dialog_set_value(self._inner, f.as_ptr()) }
    }

    /// Returns the value of the help dialog
    pub fn value(&self) -> Option<String> {
        unsafe {
            let val = Fl_Help_Dialog_value(self._inner);
            if val.is_null() {
                None
            } else {
                Some(CStr::from_ptr(val).to_string_lossy().to_string())
            }
        }
    }

    /// Returs whether the help dialog is visible
    pub fn visible(&mut self) -> bool {
        unsafe {
            match Fl_Help_Dialog_visible(self._inner) {
                0 => false,
                _ => true,
            }
        }
    }

    /// Returns the width of the help dialog
    pub fn width(&mut self) -> i32 {
        unsafe { Fl_Help_Dialog_w(self._inner) }
    }

    /// Returns the height of the help dialog
    pub fn height(&mut self) -> i32 {
        unsafe { Fl_Help_Dialog_h(self._inner) }
    }

    /// Returns the x position of the help dialog
    pub fn x(&mut self) -> i32 {
        unsafe { Fl_Help_Dialog_x(self._inner) }
    }

    /// Returns the y position of the help dialog
    pub fn y(&mut self) -> i32 {
        unsafe { Fl_Help_Dialog_y(self._inner) }
    }
}

impl Drop for HelpDialog {
    fn drop(&mut self) {
        unsafe { Fl_Help_Dialog_delete(self._inner) }
    }
}

/// Defines the type of beep to be passed to the beep function
#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum BeepType {
    Default = 0,
    Message,
    Error,
    Question,
    Password,
    Notification,
}

/// Emits a beep
pub fn beep(tp: BeepType) {
    unsafe { Fl_beep(tp as i32) }
}

/// FLTK's own FileChooser. Which differs for the Native FileDialog
pub struct FileChooser {
    _inner: *mut Fl_File_Chooser,
}

/// The types of FileChooser
#[repr(i32)]
#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub enum FileChooserType {
    Single = 0,
    Multi = 1,
    Create = 2,
    Directory = 4,
}

impl FileChooser {
    /// Instantiates a new FileChooser
    pub fn new(dir: &str, pattern: &str, typ: FileChooserType, title: &str) -> FileChooser {
        let dir = CString::new(dir).unwrap();
        let pattern = CString::new(pattern).unwrap();
        let title = CString::new(title).unwrap();
        unsafe {
            let ptr =
                Fl_File_Chooser_new(dir.as_ptr(), pattern.as_ptr(), typ as i32, title.as_ptr());
            assert!(!ptr.is_null());
            FileChooser { _inner: ptr }
        }
    }

    /// Deletes a FileChooser
    /// # Safety
    /// Can invalidate the underlying pointer
    pub unsafe fn delete(&mut self) {
        Fl_File_Chooser_delete(self._inner)
    }

    /// Gets the new button of the FileChooser
    pub fn new_button(&mut self) -> Option<crate::button::Button> {
        assert!(!self._inner.is_null());
        unsafe {
            let ptr = Fl_File_Chooser_newButton(self._inner);
            if ptr.is_null() {
                None
            } else {
                Some(crate::button::Button::from_widget_ptr(
                    ptr as *mut fltk_sys::widget::Fl_Widget,
                ))
            }
        }
    }

    /// Gets the preview button of the FileChooser
    pub fn preview_button(&mut self) -> Option<crate::button::CheckButton> {
        assert!(!self._inner.is_null());
        unsafe {
            let ptr = Fl_File_Chooser_previewButton(self._inner);
            if ptr.is_null() {
                None
            } else {
                Some(crate::button::CheckButton::from_widget_ptr(
                    ptr as *mut fltk_sys::widget::Fl_Widget,
                ))
            }
        }
    }

    /// Gets the show hidden button of the FileChooser
    pub fn show_hidden_button(&mut self) -> Option<crate::button::CheckButton> {
        assert!(!self._inner.is_null());
        unsafe {
            let ptr = Fl_File_Chooser_showHiddenButton(self._inner);
            if ptr.is_null() {
                None
            } else {
                Some(crate::button::CheckButton::from_widget_ptr(
                    ptr as *mut fltk_sys::widget::Fl_Widget,
                ))
            }
        }
    }

    /// Sets the callback of the FileChooser
    pub fn set_callback(&mut self, cb: Box<dyn FnMut()>) {
        assert!(!self._inner.is_null());
        unsafe {
            unsafe extern "C" fn shim(_arg1: *mut Fl_File_Chooser, data: *mut raw::c_void) {
                let a: *mut Box<dyn FnMut()> = data as *mut Box<dyn FnMut()>;
                let f: &mut (dyn FnMut()) = &mut **a;
                let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| f()));
            }
            let a: *mut Box<dyn FnMut()> = Box::into_raw(Box::new(cb));
            let data: *mut raw::c_void = a as *mut raw::c_void;
            let callback: Option<
                unsafe extern "C" fn(arg1: *mut Fl_File_Chooser, data: *mut raw::c_void),
            > = Some(shim);
            Fl_File_Chooser_callback(self._inner, callback, data)
        }
    }

    /// Sets the color of the FileChooser
    pub fn set_color(&mut self, c: Color) {
        assert!(!self._inner.is_null());
        unsafe { Fl_File_Chooser_set_color(self._inner, c as u32) }
    }

    /// Gets the color of the FileChooser
    pub fn color(&mut self) -> Color {
        assert!(!self._inner.is_null());
        unsafe { mem::transmute(Fl_File_Chooser_color(self._inner)) }
    }

    /// Gets the count of chosen items
    pub fn count(&mut self) -> u32 {
        assert!(!self._inner.is_null());
        unsafe { Fl_File_Chooser_count(self._inner) as u32 }
    }

    /// Sets the directory of the FileChooser
    pub fn set_directory(&mut self, dir: &str) {
        assert!(!self._inner.is_null());
        let dir = CString::new(dir).unwrap();
        unsafe { Fl_File_Chooser_set_directory(self._inner, dir.as_ptr()) }
    }

    /// Gets the directory of the FileChooser
    pub fn directory(&mut self) -> Option<String> {
        assert!(!self._inner.is_null());
        unsafe {
            let ptr = Fl_File_Chooser_directory(self._inner);
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

    /// Sets the filter of the FileChooser
    pub fn set_filter(&mut self, pattern: &str) {
        assert!(!self._inner.is_null());
        let pattern = CString::new(pattern).unwrap();
        unsafe { Fl_File_Chooser_set_filter(self._inner, pattern.as_ptr()) }
    }

    /// Gets the filter of the FileChooser
    pub fn filter(&mut self) -> Option<String> {
        assert!(!self._inner.is_null());
        unsafe {
            let ptr = Fl_File_Chooser_filter(self._inner);
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

    /// Gets the current filename filter selection
    pub fn filter_value(&mut self) -> u32 {
        assert!(!self._inner.is_null());
        unsafe { Fl_File_Chooser_filter_value(self._inner) as u32 }
    }

    /// Sets the current filename filter selection
    pub fn set_filter_value(&mut self, f: u32) {
        assert!(!self._inner.is_null());
        unsafe { Fl_File_Chooser_set_filter_value(self._inner, f as i32) }
    }

    /// Hides the File chooser
    pub fn hide(&mut self) {
        assert!(!self._inner.is_null());
        unsafe { Fl_File_Chooser_hide(self._inner) }
    }

    /// Sets the icon size of the FileChooser
    pub fn set_iconsize(&mut self, s: u8) {
        assert!(!self._inner.is_null());
        unsafe { Fl_File_Chooser_set_iconsize(self._inner, s) }
    }

    /// Gets the icon size of the FileChooser
    pub fn iconsize(&mut self) -> u8 {
        assert!(!self._inner.is_null());
        unsafe { Fl_File_Chooser_iconsize(self._inner) }
    }

    /// Sets the label of the FileChooser
    pub fn set_label(&mut self, l: &str) {
        assert!(!self._inner.is_null());
        let l = CString::new(l).unwrap();
        unsafe { Fl_File_Chooser_set_label(self._inner, l.as_ptr()) }
    }

    /// Gets the label of the FileChooser
    pub fn label(&mut self) -> Option<String> {
        assert!(!self._inner.is_null());
        unsafe {
            let ptr = Fl_File_Chooser_label(self._inner);
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

    /// Sets the label of the Ok button
    pub fn set_ok_label(&mut self, l: &str) {
        assert!(!self._inner.is_null());
        let l = CString::new(l).unwrap();
        unsafe { Fl_File_Chooser_set_ok_label(self._inner, l.as_ptr()) }
    }

    /// Gets the label of the Ok button
    pub fn ok_label(&mut self) -> Option<String> {
        assert!(!self._inner.is_null());
        unsafe {
            let ptr = Fl_File_Chooser_ok_label(self._inner);
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

    /// Add preview to the FileChooser
    pub fn set_preview(&mut self, e: bool) {
        assert!(!self._inner.is_null());
        unsafe { Fl_File_Chooser_set_preview(self._inner, e as i32) }
    }

    /// Returns whether preview is enabled for the FileChooser
    pub fn preview(&self) -> bool {
        assert!(!self._inner.is_null());
        unsafe { Fl_File_Chooser_preview(self._inner) != 0 }
    }

    /// Rescan the directory
    pub fn rescan(&mut self) {
        assert!(!self._inner.is_null());
        unsafe { Fl_File_Chooser_rescan(self._inner) }
    }

    /// Rescan the directory while keeping the file name
    pub fn rescan_keep_filename(&mut self) {
        assert!(!self._inner.is_null());
        unsafe { Fl_File_Chooser_rescan_keep_filename(self._inner) }
    }

    /// Shows the File Chooser
    pub fn show(&mut self) {
        assert!(!self._inner.is_null());
        unsafe { Fl_File_Chooser_show(self._inner) }
    }

    /// Returns whether the file chooser is shown
    pub fn shown(&mut self) -> bool {
        assert!(!self._inner.is_null());
        unsafe { Fl_File_Chooser_shown(self._inner) != 0 }
    }

    /// Sets the text color of the file chooser
    pub fn set_textcolor(&mut self, c: Color) {
        assert!(!self._inner.is_null());
        unsafe { Fl_File_Chooser_set_textcolor(self._inner, c as u32) }
    }

    /// Gets the text color of the file chooser
    pub fn textcolor(&mut self) -> Color {
        assert!(!self._inner.is_null());
        unsafe { mem::transmute(Fl_File_Chooser_textcolor(self._inner)) }
    }

    /// Sets the text font of the file chooser
    pub fn set_textfont(&mut self, f: Font) {
        assert!(!self._inner.is_null());
        unsafe { Fl_File_Chooser_set_textfont(self._inner, f as i32) }
    }

    /// Gets the text font of the file chooser
    pub fn textfont(&mut self) -> Font {
        assert!(!self._inner.is_null());
        unsafe { mem::transmute(Fl_File_Chooser_textfont(self._inner)) }
    }

    /// Sets the text size of the file chooser
    pub fn set_textsize(&mut self, s: u32) {
        assert!(!self._inner.is_null());
        unsafe { Fl_File_Chooser_set_textsize(self._inner, s as i32) }
    }

    /// Gets the text size of the file chooser
    pub fn textsize(&mut self) -> u32 {
        assert!(!self._inner.is_null());
        unsafe { Fl_File_Chooser_textsize(self._inner) as u32 }
    }

    /// Sets the type of the FileChooser
    pub fn set_type(&mut self, t: FileChooserType) {
        assert!(!self._inner.is_null());
        unsafe { Fl_File_Chooser_set_type(self._inner, t as i32) }
    }

    /// Gets the type of the FileChooser
    pub fn get_type(&mut self) -> FileChooserType {
        assert!(!self._inner.is_null());
        unsafe { mem::transmute(Fl_File_Chooser_type(self._inner)) }
    }

    /// Gets the user data of the FileChooser
    /// # Safety
    /// Can invalidate the user data while the FileChooser is in use
    pub unsafe fn raw_user_data(&self) -> *mut raw::c_void {
        Fl_File_Chooser_user_data(self._inner)
    }

    /// Gets the user data of the FileChooser
    /// # Safety
    /// Can invalidate the user data while the FileChooser is in use
    pub unsafe fn user_data(&self) -> Option<Box<dyn FnMut()>> {
        let ptr = Fl_File_Chooser_user_data(self._inner);
        if ptr.is_null() {
            None
        } else {
            let x = ptr as *mut Box<dyn FnMut()>;
            let x = Box::from_raw(x);
            Some(*x)
        }
    }

    /// Sets the user data of the FileChooser
    /// # Safety
    /// Can invalidate the user data while the FileChooser is in use
    pub unsafe fn set_user_data(&mut self, d: *mut raw::c_void) {
        Fl_File_Chooser_set_user_data(self._inner, d)
    }

    /// Gets the file or dir name chosen by the FileChooser
    pub fn value(&mut self, f: u32) -> Option<String> {
        assert!(!self._inner.is_null());
        let mut f = f;
        if f == 0 {
            f = 1;
        }
        unsafe {
            let ptr = Fl_File_Chooser_value(self._inner, f as i32);
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

    /// Sets the file or dir name chosen by the FileChooser
    pub fn set_value(&mut self, filename: &str) {
        assert!(!self._inner.is_null());
        let filename = CString::new(filename).unwrap();
        unsafe { Fl_File_Chooser_set_value(self._inner, filename.as_ptr()) }
    }

    /// Returns whether the FileChooser is visible or not
    pub fn visible(&mut self) -> bool {
        assert!(!self._inner.is_null());
        unsafe { Fl_File_Chooser_visible(self._inner) != 0 }
    }
}
