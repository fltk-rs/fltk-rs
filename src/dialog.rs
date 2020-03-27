pub use crate::prelude::*;
use fltk_sys::dialog::*;
use std::{ffi, mem, os::raw};

/// Creates a file button
#[derive(Debug, Clone)]
pub struct FileDialog {
    _inner: *mut Fl_Native_File_Chooser,
}

/// Defines the type of dialog, which can be changed dynamically using the set_type() method
#[repr(i32)]
#[derive(WidgetType, Debug, Copy, Clone)]
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
#[derive(WidgetType, Debug, Copy, Clone)]
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
        FileDialog {
            _inner: unsafe { Fl_Native_File_Chooser_new(mem::transmute(op)) },
        }
    }

    /// Returns the chosen file name
    pub fn filename(&self) -> String {
        unsafe {
            let cnt = Fl_Native_File_Chooser_count(self._inner);
            if cnt == 0 {
                return String::from("");
            }
            let x = Fl_Native_File_Chooser_filenames(self._inner, 0);
            ffi::CString::from_raw(x as *mut raw::c_char)
                .to_string_lossy()
                .to_string()
        }
    }

    /// Returns the chosen file names
    pub fn filenames(&self) -> Vec<String> {
        unsafe {
            let cnt = Fl_Native_File_Chooser_count(self._inner);
            let mut names: Vec<String> = vec![];
            if cnt == 0 {
                return names;
            } else {
                for i in 0..cnt {
                    let x = Fl_Native_File_Chooser_filenames(self._inner, i);
                    names.push(
                        ffi::CString::from_raw(x as *mut raw::c_char)
                            .to_string_lossy()
                            .to_string(),
                    )
                }
                names
            }
        }
    }

    /// Returns the preset directory
    pub fn directory(&self) -> String {
        unsafe {
            let x = Fl_Native_File_Chooser_directory(self._inner);
            if !x.is_null() {
                ffi::CString::from_raw(x as *mut raw::c_char)
                    .to_string_lossy()
                    .to_string()
            } else {
                String::from("")
            }
        }
    }

    /// Sets the starting directory
    pub fn set_directory(&mut self, dir: &str) {
        unsafe {
            Fl_Native_File_Chooser_set_directory(
                self._inner,
                ffi::CString::new(dir).unwrap().into_raw(),
            )
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
        let title = std::ffi::CString::new(title).unwrap();
        unsafe {
            Fl_Native_File_Chooser_set_title(self._inner, title.into_raw() as *const raw::c_char)
        }
    }

    /// Sets the filter for the dialog
    pub fn set_filter(&mut self, f: &str) {
        let f = std::ffi::CString::new(f).unwrap();
        unsafe {
            Fl_Native_File_Chooser_set_filter(self._inner, f.into_raw() as *const raw::c_char)
        }
    }

    /// Sets the preset filter for the dialog
    pub fn set_preset_file(&mut self, f: &str) {
        let f = std::ffi::CString::new(f).unwrap();
        unsafe {
            Fl_Native_File_Chooser_set_preset_file(self._inner, f.into_raw() as *const raw::c_char)
        }
    }

    /// returns the error message from the file dialog
    pub fn error_message(&self) -> String {
        unsafe {
            ffi::CString::from_raw(Fl_Native_File_Chooser_errmsg(self._inner) as *mut raw::c_char)
                .to_string_lossy()
                .to_string()
        }
    }
}

/// Displays a message box
pub fn message(txt: &str) {
    unsafe {
        let txt = ffi::CString::new(txt).unwrap();
        cfl_message(txt.into_raw() as *const raw::c_char)
    }
}

/// Displays an alert box
pub fn alert(txt: &str) {
    unsafe {
        let txt = ffi::CString::new(txt).unwrap();
        cfl_alert(txt.into_raw() as *const raw::c_char)
    }
}

/// Displays a choice box with upto three choices
/// An empty choice will not be shown
pub fn choice(txt: &str, b0: &str, b1: &str, b2: &str) -> usize {
    unsafe {
        let txt = ffi::CString::new(txt).unwrap();
        let b0 = ffi::CString::new(b0).unwrap();
        let b1 = ffi::CString::new(b1).unwrap();
        let b2 = ffi::CString::new(b2).unwrap();
        cfl_choice(
            txt.into_raw() as *const raw::c_char,
            b0.into_raw() as *const raw::c_char,
            b1.into_raw() as *const raw::c_char,
            b2.into_raw() as *const raw::c_char,
        ) as usize
    }
}

/// Displays an input box, which returns the inputted string.
/// Can be used for gui io
pub fn input(txt: &str, deflt: &str) -> Option<String> {
    unsafe {
        let temp = ffi::CString::new(deflt.clone()).unwrap().into_raw() as *const raw::c_char;
        let txt = ffi::CString::new(txt).unwrap();
        let x = cfl_input(txt.into_raw() as *const raw::c_char, temp);
        if x.is_null() {
            return None;
        } else {
            Some(ffi::CStr::from_ptr(x as *const raw::c_char)
                .to_string_lossy()
                .to_string())
        }
    }
}

/// Shows an input box, but with hidden string
pub fn password(txt: &str, deflt: &str) -> Option<String> {
    unsafe {
        let temp = ffi::CString::new(deflt.clone()).unwrap().into_raw() as *const raw::c_char;
        let txt = ffi::CString::new(txt).unwrap();
        let x = cfl_password(txt.into_raw() as *const raw::c_char, temp);
        if x.is_null() {
            return None;
        } else {
            Some(ffi::CStr::from_ptr(x as *const raw::c_char)
                .to_string_lossy()
                .to_string())
        }
    }
}
