pub use crate::prelude::*;
use fltk_sys::dialog::*;
use std::{
    mem,
    os::raw,
    ffi::{CStr, CString},
};

/// Creates a file button
#[derive(Debug, Clone)]
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
            assert!(!file_dialog.is_null(), "Failed to instantiate file dialog!");
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
            std::path::PathBuf::from(CStr::from_ptr(x as *mut raw::c_char)
                .to_string_lossy()
                .to_string())
        }
    }

    /// Returns the chosen file names
    pub fn filenames(&self) -> Vec<std::path::PathBuf> {
        unsafe {
            let cnt = Fl_Native_File_Chooser_count(self._inner);
            let mut names: Vec<std::path::PathBuf> = vec![];
            if cnt == 0 {
                return names;
            } else {
                for i in 0..cnt {
                    let x = Fl_Native_File_Chooser_filenames(self._inner, i);
                    names.push(
                        std::path::PathBuf::from(CStr::from_ptr(x as *mut raw::c_char)
                            .to_string_lossy()
                            .to_string()),
                    )
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
                std::path::PathBuf::from(CStr::from_ptr(x as *mut raw::c_char)
                    .to_string_lossy()
                    .to_string())
            } else {
                std::path::PathBuf::from("")
            }
        }
    }

    /// Sets the starting directory
    pub fn set_directory(&mut self, dir: &std::path::Path) {
        unsafe {
            Fl_Native_File_Chooser_set_directory(self._inner, CString::new(dir.to_str().unwrap()).unwrap().into_raw())
        }
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
        unsafe {
            Fl_Native_File_Chooser_set_title(self._inner, title.into_raw() as *const raw::c_char)
        }
    }

    /// Sets the filter for the dialog
    pub fn set_filter(&mut self, f: &str) {
        let f = CString::new(f).unwrap();
        unsafe {
            Fl_Native_File_Chooser_set_filter(self._inner, f.into_raw() as *const raw::c_char)
        }
    }

    /// Sets the preset filter for the dialog
    pub fn set_preset_file(&mut self, f: &str) {
        let f = CString::new(f).unwrap();
        unsafe {
            Fl_Native_File_Chooser_set_preset_file(self._inner, f.into_raw() as *const raw::c_char)
        }
    }

    /// returns the error message from the file dialog
    pub fn error_message(&self) -> String {
        unsafe {
            let err_msg = Fl_Native_File_Chooser_errmsg(self._inner);
            assert!(!err_msg.is_null(), "Failed to retreive error message!");
            CStr::from_ptr(err_msg as *mut raw::c_char)
                .to_string_lossy()
                .to_string()
        }
    }
}

/// Displays a message box
pub fn message(txt: &str) {
    unsafe {
        let txt = CString::new(txt).unwrap();
        cfl_message(txt.into_raw() as *const raw::c_char)
    }
}

/// Displays an alert box
pub fn alert(txt: &str) {
    unsafe {
        let txt = CString::new(txt).unwrap();
        cfl_alert(txt.into_raw() as *const raw::c_char)
    }
}

/// Displays a choice box with upto three choices
/// An empty choice will not be shown
pub fn choice(txt: &str, b0: &str, b1: &str, b2: &str) -> u32 {
    unsafe {
        let txt = CString::new(txt).unwrap();
        let b0 = CString::new(b0).unwrap();
        let b1 = CString::new(b1).unwrap();
        let b2 = CString::new(b2).unwrap();
        cfl_choice(
            txt.into_raw() as *const raw::c_char,
            b0.into_raw() as *const raw::c_char,
            b1.into_raw() as *const raw::c_char,
            b2.into_raw() as *const raw::c_char,
        ) as u32
    }
}

/// Displays an input box, which returns the inputted string.
/// Can be used for gui io
pub fn input(txt: &str, deflt: &str) -> Option<String> {
    unsafe {
        let temp = CString::new(deflt.clone()).unwrap().into_raw() as *const raw::c_char;
        let txt = CString::new(txt).unwrap();
        let x = cfl_input(txt.into_raw() as *const raw::c_char, temp);
        if x.is_null() {
            return None;
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
pub fn password(txt: &str, deflt: &str) -> Option<String> {
    unsafe {
        let temp = CString::new(deflt.clone()).unwrap().into_raw() as *const raw::c_char;
        let txt = CString::new(txt).unwrap();
        let x = cfl_password(txt.into_raw() as *const raw::c_char, temp);
        if x.is_null() {
            return None;
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
#[derive(Debug, Clone)]
pub struct HelpDialog {
    _inner: *mut Fl_Help_Dialog,
}

impl HelpDialog {
    /// Creates a default (size and location) help dialog
    pub fn default() -> HelpDialog {
        unsafe {
            let help_dialog = Fl_Help_Dialog_new();
            assert!(!help_dialog.is_null(), "Failed to instantiate help dialog!");
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
        unsafe {
            Fl_Help_Dialog_hide(self._inner)
        }
    }
    /// Loads a file for the help dialog
    pub fn load(&mut self, file: &std::path::Path) -> i32 {
        let f = file.to_str().unwrap();
        let f = CString::new(f).unwrap();
        unsafe {
            Fl_Help_Dialog_load(self._inner, f.into_raw() as *const raw::c_char)
        }
    }
    /// Sets the position of the help dialog
    pub fn position(&mut self, x: i32, y: i32) {
        unsafe {
            Fl_Help_Dialog_position(self._inner, x, y)
        }
    }
    /// Resizes the help dialog
    pub fn resize(&mut self,  x: i32, y: i32,  w: i32, h: i32) {
        unsafe {
            Fl_Help_Dialog_resize(self._inner, x, y , w, h)
        }
    }
    /// Shows the help dialog
    pub fn show(&mut self) {
        unsafe {
            Fl_Help_Dialog_show(self._inner)
        }
    }
    /// Sets the text size
    pub fn set_text_size(&mut self, s: u32) {
        unsafe {
            Fl_Help_Dialog_set_text_size(self._inner, s as i32)
        }
    }
    /// Returns the text size
    pub fn text_size(&mut self) -> u32 {
        unsafe {
            Fl_Help_Dialog_text_size(self._inner) as u32
        }
    }
    /// Sets the value of the help dialog
    pub fn set_value(&mut self, f: &str) {
        let f = CString::new(f).unwrap();
        unsafe {
            Fl_Help_Dialog_set_value(self._inner, f.into_raw() as *const raw::c_char)
        }
    }
    /// Returns the value of the help dialog
    pub fn value(&self) -> String {
        unsafe {
            let val = Fl_Help_Dialog_value(self._inner);
            assert!(!val.is_null(), "Failed to retrieve value from help dialog!");
            CStr::from_ptr(val).to_string_lossy().to_string()
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
        unsafe {
            Fl_Help_Dialog_w(self._inner)
        }
    }
    /// Returns the height of the help dialog
    pub fn height(&mut self) -> i32 {
        unsafe {
            Fl_Help_Dialog_h(self._inner)
        }
    }
    /// Returns the x position of the help dialog
    pub fn x(&mut self) -> i32 {
        unsafe {
            Fl_Help_Dialog_x(self._inner)
        }
    }
    /// Returns the y position of the help dialog
    pub fn y(&mut self) -> i32 {
        unsafe {
            Fl_Help_Dialog_y(self._inner)
        }
    }
}
