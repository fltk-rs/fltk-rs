use crate::enums::{Color, Font};
use crate::prelude::*;
use crate::utils::FlString;
use fltk_sys::dialog::*;
use std::{
    ffi::{CStr, CString},
    mem,
    os::raw,
    path::{Path, PathBuf},
};

/// Color modes to be used with the color chooser
#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ColorMode {
    /// Rgb color mode
    Rgb = 0,
    /// Byte color mode
    Byte = 1,
    /// Hex color mode
    Hex = 2,
    /// Hsv color mode
    Hsv = 3,
}

/// FLTK's NativeFileChooser
#[derive(Debug)]
pub struct FileDialog {
    inner: *mut Fl_Native_File_Chooser,
}

/// FLTK's NativeFileChooser
pub type NativeFileChooser = FileDialog;

/// Defines the type of dialog, which can be changed dynamically using the `set_type()` method
#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum FileDialogType {
    /// Browse file
    BrowseFile = 0,
    /// Browse dir
    BrowseDir,
    /// Browse multiple files
    BrowseMultiFile,
    /// Browse multiple dirs
    BrowseMultiDir,
    /// Browse save file
    BrowseSaveFile,
    /// Browse save directory
    BrowseSaveDir,
}

crate::macros::widget::impl_widget_type!(FileDialogType);

/// Alias for `NativeFileChooserType`
pub type NativeFileChooserType = FileDialogType;

/// Defines the File dialog options, which can be set using the `set_option()` method.
#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum FileDialogOptions {
    /// No options
    NoOptions = 0,
    /// Confirm on save as
    SaveAsConfirm = 1,
    /// New folder option
    NewFolder = 2,
    /// Enable preview
    Preview = 4,
    /// Use extension filter
    UseFilterExt = 8,
}

crate::macros::widget::impl_widget_type!(FileDialogOptions);

/// Alias to `NativeFileChooserOptions`
pub type NativeFileChooserOptions = FileDialogOptions;

impl std::ops::BitOr<FileDialogOptions> for FileDialogOptions {
    type Output = FileDialogOptions;
    fn bitor(self, other: FileDialogOptions) -> Self::Output {
        unsafe { std::mem::transmute(self as i32 | other as i32) }
    }
}

impl FileDialog {
    /// Creates an new file dialog
    pub fn new(op: FileDialogType) -> FileDialog {
        unsafe {
            let file_dialog = Fl_Native_File_Chooser_new(mem::transmute(op));
            assert!(!file_dialog.is_null());
            FileDialog { inner: file_dialog }
        }
    }

    /// Returns the chosen file name
    pub fn filename(&self) -> PathBuf {
        assert!(!self.inner.is_null());
        unsafe {
            let cnt = Fl_Native_File_Chooser_count(self.inner);
            if cnt == 0 {
                return PathBuf::from("");
            }
            let x = Fl_Native_File_Chooser_filenames(self.inner, 0);
            PathBuf::from(
                CStr::from_ptr(x as *mut raw::c_char)
                    .to_string_lossy()
                    .to_string(),
            )
        }
    }

    /// Returns the chosen file names
    pub fn filenames(&self) -> Vec<PathBuf> {
        assert!(!self.inner.is_null());
        unsafe {
            let cnt = Fl_Native_File_Chooser_count(self.inner);
            let mut names: Vec<PathBuf> = vec![];
            for i in 0..cnt {
                let x = Fl_Native_File_Chooser_filenames(self.inner, i);
                names.push(PathBuf::from(
                    CStr::from_ptr(x as *mut raw::c_char)
                        .to_string_lossy()
                        .to_string(),
                ));
            }
            names
        }
    }

    /// Returns the preset directory
    pub fn directory(&self) -> PathBuf {
        assert!(!self.inner.is_null());
        unsafe {
            let x = Fl_Native_File_Chooser_directory(self.inner);
            if x.is_null() {
                PathBuf::from("")
            } else {
                PathBuf::from(
                    CStr::from_ptr(x as *mut raw::c_char)
                        .to_string_lossy()
                        .to_string(),
                )
            }
        }
    }

    /// Sets the starting directory
    /// # Errors
    /// Errors on non-existent path
    pub fn set_directory<P: AsRef<Path>>(&mut self, dir: &P) -> Result<(), FltkError> {
        assert!(!self.inner.is_null());
        self.set_directory_(dir.as_ref())
    }

    fn set_directory_(&mut self, dir: &Path) -> Result<(), FltkError> {
        assert!(!self.inner.is_null());
        let dir = CString::new(dir.to_str().ok_or_else(|| {
            FltkError::Unknown(String::from("Failed to convert path to string"))
        })?)?;
        unsafe { Fl_Native_File_Chooser_set_directory(self.inner, dir.as_ptr()) }
        Ok(())
    }

    /// Shows the file dialog
    pub fn show(&mut self) {
        assert!(!self.inner.is_null());
        unsafe {
            Fl_Native_File_Chooser_show(self.inner);
        }
    }

    /// Sets the option for the dialog
    pub fn set_option(&mut self, opt: FileDialogOptions) {
        assert!(!self.inner.is_null());
        unsafe { Fl_Native_File_Chooser_set_option(self.inner, opt as i32) }
    }

    /// Sets the type for the dialog
    pub fn set_type(&mut self, op: FileDialogType) {
        assert!(!self.inner.is_null());
        unsafe { Fl_Native_File_Chooser_set_type(self.inner, op as i32) }
    }

    /// Sets the title for the dialog
    pub fn set_title(&mut self, title: &str) {
        assert!(!self.inner.is_null());
        let title = CString::safe_new(title);
        unsafe { Fl_Native_File_Chooser_set_title(self.inner, title.as_ptr()) }
    }

    /// Sets the filter for the dialog, can be:
    /// A single wildcard (e.g. `"*.txt"`).
    /// Multiple wildcards (e.g. `"*.{cxx,h,H}"`).
    /// A descriptive name followed by a `\t` and a wildcard (e.g. `"Text Files\t*.txt"`).
    /// A list of separate wildcards with a `\n` between each (e.g. `"*.{cxx,H}\n*.txt"`).
    /// A list of descriptive names and wildcards (e.g. `"C++ Files\t*.{cxx,H}\nTxt Files\t*.txt"`)
    pub fn set_filter(&mut self, f: &str) {
        assert!(!self.inner.is_null());
        let f = CString::safe_new(f);
        unsafe { Fl_Native_File_Chooser_set_filter(self.inner, f.as_ptr()) }
    }

    /// Sets the preset filter for the dialog
    pub fn set_preset_file(&mut self, f: &str) {
        assert!(!self.inner.is_null());
        let f = CString::safe_new(f);
        unsafe { Fl_Native_File_Chooser_set_preset_file(self.inner, f.as_ptr()) }
    }

    /// returns the error message from the file dialog
    pub fn error_message(&self) -> Option<String> {
        assert!(!self.inner.is_null());
        unsafe {
            let err_msg = Fl_Native_File_Chooser_errmsg(self.inner);
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
        if !self.inner.is_null() {
            unsafe { Fl_Native_File_Chooser_delete(self.inner) }
            self.inner = std::ptr::null_mut();
        }
    }
}

/// Displays a message box
pub fn message(x: i32, y: i32, txt: &str) {
    unsafe {
        let txt = CString::safe_new(txt);
        Fl_message(x, y, txt.as_ptr())
    }
}

/// Displays an alert box
pub fn alert(x: i32, y: i32, txt: &str) {
    unsafe {
        let txt = CString::safe_new(txt);
        Fl_alert(x, y, txt.as_ptr())
    }
}

#[deprecated(since = "1.3.1", note = "please use `choice2` instead")]
/// Displays a choice box with up to three choices. Choosing a value returns its index from the arguments
pub fn choice(x: i32, y: i32, txt: &str, b0: &str, b1: &str, b2: &str) -> i32 {
    unsafe {
        let txt = CString::safe_new(txt);
        let b0 = CString::safe_new(b0);
        let b1 = CString::safe_new(b1);
        let b2 = CString::safe_new(b2);
        Fl_choice(x, y, txt.as_ptr(), b0.as_ptr(), b1.as_ptr(), b2.as_ptr()) as i32
    }
}

/// Displays a choice box with up to three choices.
/// Closing the dialog returns None. Choosing a value returns its index from the arguments.
pub fn choice2(x: i32, y: i32, txt: &str, b0: &str, b1: &str, b2: &str) -> Option<i32> {
    unsafe {
        let txt = CString::safe_new(txt);
        let b0 = CString::safe_new(b0);
        let b1 = CString::safe_new(b1);
        let b2 = CString::safe_new(b2);
        let ret = Fl_choice_n(x, y, txt.as_ptr(), b0.as_ptr(), b1.as_ptr(), b2.as_ptr()) as i32;
        if ret < 0 {
            None
        } else {
            Some(ret)
        }
    }
}

/// Displays an input box, which returns the inputted string.
/// Can be used for gui io
pub fn input(x: i32, y: i32, txt: &str, deflt: &str) -> Option<String> {
    unsafe {
        let temp = CString::safe_new(deflt);
        let txt = CString::safe_new(txt);
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
        let temp = CString::safe_new(deflt);
        let txt = CString::safe_new(txt);
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

/// Displays a message box, the dialog is positioned at the pointer hotspot
pub fn message_default(txt: &str) {
    unsafe {
        let txt = CString::safe_new(txt);
        Fl_message2(txt.as_ptr())
    }
}

/// Displays an alert box, the dialog is positioned at the pointer hotspot
pub fn alert_default(txt: &str) {
    unsafe {
        let txt = CString::safe_new(txt);
        Fl_alert2(txt.as_ptr())
    }
}

#[deprecated(since = "1.3.1", note = "please use `choice2_default` instead")]
/// Displays a choice box with up to three choices.
/// The dialog is positioned at the pointer hotspot
pub fn choice_default(txt: &str, b0: &str, b1: &str, b2: &str) -> i32 {
    unsafe {
        let txt = CString::safe_new(txt);
        let b0 = CString::safe_new(b0);
        let b1 = CString::safe_new(b1);
        let b2 = CString::safe_new(b2);
        Fl_choice2(txt.as_ptr(), b0.as_ptr(), b1.as_ptr(), b2.as_ptr()) as i32
    }
}

/// Displays a choice box with up to three choices.
/// An empty choice will not be shown. Closing the dialog returns None. Choosing a value returns its index from the arguments.
/// The dialog is positioned at the pointer hotspot
pub fn choice2_default(txt: &str, b0: &str, b1: &str, b2: &str) -> Option<i32> {
    unsafe {
        let txt = CString::safe_new(txt);
        let b0 = CString::safe_new(b0);
        let b1 = CString::safe_new(b1);
        let b2 = CString::safe_new(b2);
        let ret = Fl_choice2_n(txt.as_ptr(), b0.as_ptr(), b1.as_ptr(), b2.as_ptr()) as i32;
        if ret < 0 {
            None
        } else {
            Some(ret)
        }
    }
}

/// Displays an input box, which returns the inputted string.
/// Can be used for gui io.
/// The dialog is positioned at the pointer hotspot
pub fn input_default(txt: &str, deflt: &str) -> Option<String> {
    unsafe {
        let temp = CString::safe_new(deflt);
        let txt = CString::safe_new(txt);
        let x = Fl_input2(txt.as_ptr(), temp.as_ptr());
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

/// Shows an input box, but with hidden string.
/// The dialog is positioned at the pointer hotspot
pub fn password_default(txt: &str, deflt: &str) -> Option<String> {
    unsafe {
        let temp = CString::safe_new(deflt);
        let txt = CString::safe_new(txt);
        let x = Fl_password2(txt.as_ptr(), temp.as_ptr());
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
    inner: *mut Fl_Help_Dialog,
}

impl HelpDialog {
    /// Creates a default (size and location) help dialog
    pub fn default() -> HelpDialog {
        unsafe {
            let help_dialog = Fl_Help_Dialog_new();
            assert!(!help_dialog.is_null());
            HelpDialog { inner: help_dialog }
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
        unsafe { Fl_Help_Dialog_hide(self.inner) }
    }

    /// Loads a file for the help dialog
    /// # Errors
    /// Errors on non-existent path
    pub fn load<P: AsRef<Path>>(&mut self, file: P) -> Result<(), FltkError> {
        self.load_(file.as_ref())
    }

    fn load_(&mut self, file: &Path) -> Result<(), FltkError> {
        let f = file
            .to_str()
            .ok_or_else(|| FltkError::Unknown(String::from("Failed to convert path to string")))?;
        let f = CString::new(f)?;
        unsafe {
            match Fl_Help_Dialog_load(self.inner, f.as_ptr()) {
                0 => Ok(()),
                _ => Err(FltkError::Internal(FltkErrorKind::ResourceNotFound)),
            }
        }
    }

    /// Sets the position of the help dialog
    pub fn position(&mut self, x: i32, y: i32) {
        unsafe { Fl_Help_Dialog_position(self.inner, x, y) }
    }

    /// Resizes the help dialog
    pub fn resize(&mut self, x: i32, y: i32, w: i32, h: i32) {
        unsafe { Fl_Help_Dialog_resize(self.inner, x, y, w, h) }
    }

    /// Shows the help dialog
    pub fn show(&mut self) {
        unsafe { Fl_Help_Dialog_show(self.inner) }
    }

    /// Sets the text size
    pub fn set_text_size(&mut self, s: i32) {
        unsafe { Fl_Help_Dialog_set_text_size(self.inner, s as i32) }
    }

    /// Returns the text size
    pub fn text_size(&self) -> i32 {
        unsafe { Fl_Help_Dialog_text_size(self.inner) as i32 }
    }

    /// Sets the value of the help dialog
    pub fn set_value(&mut self, f: &str) {
        let f = CString::safe_new(f);
        unsafe { Fl_Help_Dialog_set_value(self.inner, f.as_ptr()) }
    }

    /// Returns the value of the help dialog
    pub fn value(&self) -> Option<String> {
        unsafe {
            let val = Fl_Help_Dialog_value(self.inner);
            if val.is_null() {
                None
            } else {
                Some(CStr::from_ptr(val).to_string_lossy().to_string())
            }
        }
    }

    /// Returns whether the help dialog is visible
    pub fn visible(&self) -> bool {
        unsafe { Fl_Help_Dialog_visible(self.inner) != 0 }
    }

    /// Returns whether the help dialog is visible
    pub fn shown(&self) -> bool {
        unsafe { Fl_Help_Dialog_visible(self.inner) != 0 }
    }

    /// Returns the width of the help dialog
    pub fn width(&self) -> i32 {
        unsafe { Fl_Help_Dialog_w(self.inner) }
    }

    /// Returns the height of the help dialog
    pub fn height(&self) -> i32 {
        unsafe { Fl_Help_Dialog_h(self.inner) }
    }

    /// Returns the width of the help dialog
    pub fn w(&self) -> i32 {
        unsafe { Fl_Help_Dialog_w(self.inner) }
    }

    /// Returns the height of the help dialog
    pub fn h(&self) -> i32 {
        unsafe { Fl_Help_Dialog_h(self.inner) }
    }

    /// Returns the x position of the help dialog
    pub fn x(&self) -> i32 {
        unsafe { Fl_Help_Dialog_x(self.inner) }
    }

    /// Returns the y position of the help dialog
    pub fn y(&self) -> i32 {
        unsafe { Fl_Help_Dialog_y(self.inner) }
    }
}

impl Drop for HelpDialog {
    fn drop(&mut self) {
        unsafe { Fl_Help_Dialog_delete(self.inner) }
    }
}

/// Defines the type of beep to be passed to the beep function
#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum BeepType {
    /// Default beep
    Default = 0,
    /// Message beep
    Message,
    /// Error beep
    Error,
    /// Question beep
    Question,
    /// Password sound
    Password,
    /// Notification sound
    Notification,
}

/// Emits a beep
pub fn beep(tp: BeepType) {
    unsafe { Fl_beep(tp as i32) }
}

/**
    FLTK's own `FileChooser`. Which differs for the Native `FileDialog`
    Example:
    ```rust,no_run
    use fltk::{prelude::*, *};
    fn main() {
        let app = app::App::default();
        let mut win = window::Window::default().with_size(900, 300);
        let mut chooser = dialog::FileChooser::new(
            ".",                    // directory
            "*",                    // filter or pattern
            dialog::FileChooserType::Multi, // chooser type
            "Title Of Chooser",     // title
        );
        chooser.show();
        chooser.window().set_pos(300, 300);
        // Block until user picks something.
        //     (The other way to do this is to use a callback())
        //
        while chooser.shown() {
            app::wait();
        }
        // User hit cancel?
        if chooser.value(1).is_none() {
            println!("(User hit 'Cancel')");
            return;
        }
        // Print what the user picked
        println!("--------------------");
        println!("DIRECTORY: '{}'", chooser.directory().unwrap());
        println!("    VALUE: '{}'", chooser.value(1).unwrap()); // value starts at 1!
        println!("    COUNT: {} files selected", chooser.count());
        // Multiple files? Show all of them
        if chooser.count() > 1 {
            for t in 1..=chooser.count() {
                println!(" VALUE[{}]: '{}'", t, chooser.value(t).unwrap());
            }
        }
        win.end();
        win.show();
        app.run().unwrap();
    }
    ```
*/
pub struct FileChooser {
    inner: *mut Fl_File_Chooser,
}

bitflags::bitflags! {
    /// The types of FileChooser
    pub struct FileChooserType: i32 {
        /// Single file
        const Single = 0;
        /// Multiple files
        const Multi = 1;
        /// Allow creation of file/dir
        const Create = 2;
        /// Directory
        const Directory = 4;
    }
}

impl FileChooser {
    /// Instantiates a new `FileChooser`
    pub fn new<P: AsRef<Path>>(
        dir: P,
        pattern: &str,
        typ: FileChooserType,
        title: &str,
    ) -> FileChooser {
        Self::new_(dir.as_ref(), pattern, typ, title)
    }

    fn new_(dir: &Path, pattern: &str, typ: FileChooserType, title: &str) -> FileChooser {
        let dir = if let Some(dir) = dir.to_str() {
            dir
        } else {
            "."
        };
        let dir = CString::safe_new(dir);
        let pattern = CString::safe_new(pattern);
        let title = CString::safe_new(title);
        unsafe {
            let ptr = Fl_File_Chooser_new(
                dir.as_ptr(),
                pattern.as_ptr(),
                typ.bits as i32,
                title.into_raw(),
            );
            assert!(!ptr.is_null());
            FileChooser { inner: ptr }
        }
    }
    /// Deletes a `FileChooser`
    /// # Safety
    /// Can invalidate the underlying pointer
    pub unsafe fn delete(dlg: Self) {
        Fl_File_Chooser_delete(dlg.inner)
    }

    /// Gets the new button of the `FileChooser`
    pub fn new_button(&self) -> Option<impl ButtonExt> {
        assert!(!self.inner.is_null());
        unsafe {
            let ptr = Fl_File_Chooser_newButton(self.inner);
            if ptr.is_null() {
                None
            } else {
                Some(crate::button::Button::from_widget_ptr(ptr as *mut _))
            }
        }
    }

    /// Gets the preview button of the `FileChooser`
    pub fn preview_button(&self) -> Option<impl ButtonExt> {
        assert!(!self.inner.is_null());
        unsafe {
            let ptr = Fl_File_Chooser_previewButton(self.inner);
            if ptr.is_null() {
                None
            } else {
                Some(crate::button::CheckButton::from_widget_ptr(
                    ptr as *mut fltk_sys::widget::Fl_Widget,
                ))
            }
        }
    }

    /// Gets the show hidden button of the `FileChooser`
    pub fn show_hidden_button(&self) -> Option<impl ButtonExt> {
        assert!(!self.inner.is_null());
        unsafe {
            let ptr = Fl_File_Chooser_showHiddenButton(self.inner);
            if ptr.is_null() {
                None
            } else {
                Some(crate::button::CheckButton::from_widget_ptr(
                    ptr as *mut fltk_sys::widget::Fl_Widget,
                ))
            }
        }
    }

    /// Sets the callback of the `FileChooser`
    pub fn set_callback<F: FnMut(&mut Self) + 'static>(&mut self, cb: F) {
        assert!(!self.inner.is_null());
        unsafe {
            unsafe extern "C" fn shim(arg1: *mut Fl_File_Chooser, data: *mut raw::c_void) {
                let mut wid = FileChooser { inner: arg1 };
                let a: *mut Box<dyn FnMut(&mut FileChooser)> =
                    data as *mut Box<dyn FnMut(&mut FileChooser)>;
                let f: &mut (dyn FnMut(&mut FileChooser)) = &mut **a;
                let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| f(&mut wid)));
            }
            let _old_data = self.user_data();
            let a: *mut Box<dyn FnMut(&mut Self)> = Box::into_raw(Box::new(Box::new(cb)));
            let data: *mut raw::c_void = a as *mut raw::c_void;
            let callback: Option<
                unsafe extern "C" fn(arg1: *mut Fl_File_Chooser, data: *mut raw::c_void),
            > = Some(shim);
            Fl_File_Chooser_set_callback(self.inner, callback, data)
        }
    }

    /// Sets the color of the `FileChooser`
    pub fn set_color(&mut self, c: Color) {
        assert!(!self.inner.is_null());
        unsafe { Fl_File_Chooser_set_color(self.inner, c.bits() as u32) }
    }

    /// Gets the color of the `FileChooser`
    pub fn color(&self) -> Color {
        assert!(!self.inner.is_null());
        unsafe { mem::transmute(Fl_File_Chooser_color(self.inner)) }
    }

    /// Gets the count of chosen items
    pub fn count(&self) -> i32 {
        assert!(!self.inner.is_null());
        unsafe { Fl_File_Chooser_count(self.inner) as i32 }
    }

    /// Sets the directory of the `FileChooser`
    pub fn set_directory<P: AsRef<Path>>(&mut self, dir: P) {
        self.set_directory_(dir.as_ref())
    }

    fn set_directory_(&mut self, dir: &Path) {
        assert!(!self.inner.is_null());
        if let Some(dir) = dir.to_str() {
            let dir = CString::safe_new(dir);
            unsafe { Fl_File_Chooser_set_directory(self.inner, dir.as_ptr()) }
        }
    }

    /// Gets the directory of the `FileChooser`
    pub fn directory(&self) -> Option<String> {
        assert!(!self.inner.is_null());
        unsafe {
            let ptr = Fl_File_Chooser_directory(self.inner);
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

    /// Sets the filter for the dialog, can be:
    /// Multiple patterns can be used by separating them with tabs, like "*.jpg\t*.png\t*.gif\t*".
    /// In addition, you can provide human-readable labels with the patterns inside parenthesis,
    /// like "JPEG Files (*.jpg)\tPNG Files (*.png)\tGIF Files (*.gif)\tAll Files (*)
    /// And "Rust Files (*.{rs,txt,toml})"
    pub fn set_filter(&mut self, pattern: &str) {
        assert!(!self.inner.is_null());
        let pattern = CString::safe_new(pattern);
        unsafe { Fl_File_Chooser_set_filter(self.inner, pattern.as_ptr()) }
    }

    /// Gets the filter of the `FileChooser`
    pub fn filter(&self) -> Option<String> {
        assert!(!self.inner.is_null());
        unsafe {
            let ptr = Fl_File_Chooser_filter(self.inner);
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
    pub fn filter_value(&self) -> i32 {
        assert!(!self.inner.is_null());
        unsafe { Fl_File_Chooser_filter_value(self.inner) as i32 }
    }

    /// Sets the filter value using an index to the '\t'separated filters
    pub fn set_filter_value(&mut self, f: i32) {
        assert!(!self.inner.is_null());
        unsafe { Fl_File_Chooser_set_filter_value(self.inner, f as i32) }
    }

    /// Hides the file chooser
    pub fn hide(&mut self) {
        assert!(!self.inner.is_null());
        unsafe { Fl_File_Chooser_hide(self.inner) }
    }

    /// Sets the icon size of the `FileChooser`
    pub fn set_icon_size(&mut self, s: u8) {
        assert!(!self.inner.is_null());
        unsafe { Fl_File_Chooser_set_iconsize(self.inner, s) }
    }

    /// Gets the icon size of the `FileChooser`
    pub fn icon_size(&self) -> u8 {
        assert!(!self.inner.is_null());
        unsafe { Fl_File_Chooser_iconsize(self.inner) }
    }

    /// Sets the label of the `FileChooser`
    pub fn set_label(&mut self, l: &str) {
        assert!(!self.inner.is_null());
        let l = CString::safe_new(l);
        let _old = unsafe { CString::from_raw(Fl_File_Chooser_label(self.inner) as _) };
        unsafe { Fl_File_Chooser_set_label(self.inner, l.into_raw()) }
    }

    /// Gets the label of the `FileChooser`
    pub fn label(&self) -> String {
        assert!(!self.inner.is_null());
        unsafe {
            let ptr = Fl_File_Chooser_label(self.inner);
            if ptr.is_null() {
                String::from("")
            } else {
                CStr::from_ptr(ptr as *mut raw::c_char)
                    .to_string_lossy()
                    .to_string()
            }
        }
    }

    /// Sets the label of the Ok button
    pub fn set_ok_label(&mut self, l: &'static str) {
        assert!(!self.inner.is_null());
        let l = CString::safe_new(l);
        unsafe { Fl_File_Chooser_set_ok_label(self.inner, l.into_raw()) }
    }

    /// Gets the label of the Ok button
    pub fn ok_label(&self) -> String {
        assert!(!self.inner.is_null());
        unsafe {
            let ptr = Fl_File_Chooser_ok_label(self.inner);
            if ptr.is_null() {
                String::from("")
            } else {
                CStr::from_ptr(ptr as *mut raw::c_char)
                    .to_string_lossy()
                    .to_string()
            }
        }
    }

    /// Add preview to the `FileChooser`
    pub fn set_preview(&mut self, e: bool) {
        assert!(!self.inner.is_null());
        unsafe { Fl_File_Chooser_set_preview(self.inner, e as i32) }
    }

    /// Returns whether preview is enabled for the `FileChooser`
    pub fn preview(&self) -> bool {
        assert!(!self.inner.is_null());
        unsafe { Fl_File_Chooser_preview(self.inner) != 0 }
    }

    /// Rescan the directory
    pub fn rescan(&mut self) {
        assert!(!self.inner.is_null());
        unsafe { Fl_File_Chooser_rescan(self.inner) }
    }

    /// Rescan the directory while keeping the file name
    pub fn rescan_keep_filename(&mut self) {
        assert!(!self.inner.is_null());
        unsafe { Fl_File_Chooser_rescan_keep_filename(self.inner) }
    }

    /// Shows the File Chooser
    pub fn show(&mut self) {
        assert!(!self.inner.is_null());
        unsafe { Fl_File_Chooser_show(self.inner) }
    }

    /// Returns whether the file chooser is shown
    pub fn shown(&self) -> bool {
        assert!(!self.inner.is_null());
        unsafe { Fl_File_Chooser_shown(self.inner) != 0 }
    }

    /// Sets the text color of the file chooser
    pub fn set_text_color(&mut self, c: Color) {
        assert!(!self.inner.is_null());
        unsafe { Fl_File_Chooser_set_text_color(self.inner, c.bits() as u32) }
    }

    /// Gets the text color of the file chooser
    pub fn text_color(&self) -> Color {
        assert!(!self.inner.is_null());
        unsafe { mem::transmute(Fl_File_Chooser_text_color(self.inner)) }
    }

    /// Sets the text font of the file chooser
    pub fn set_text_font(&mut self, f: Font) {
        assert!(!self.inner.is_null());
        unsafe { Fl_File_Chooser_set_text_font(self.inner, f.bits() as i32) }
    }

    /// Gets the text font of the file chooser
    pub fn text_font(&self) -> Font {
        assert!(!self.inner.is_null());
        unsafe { mem::transmute(Fl_File_Chooser_text_font(self.inner)) }
    }

    /// Sets the text size of the file chooser
    pub fn set_text_size(&mut self, s: i32) {
        assert!(!self.inner.is_null());
        unsafe { Fl_File_Chooser_set_text_size(self.inner, s as i32) }
    }

    /// Gets the text size of the file chooser
    pub fn text_size(&self) -> i32 {
        assert!(!self.inner.is_null());
        unsafe { Fl_File_Chooser_text_size(self.inner) as i32 }
    }

    /// Sets the type of the `FileChooser`
    pub fn set_type(&mut self, t: FileChooserType) {
        assert!(!self.inner.is_null());
        unsafe { Fl_File_Chooser_set_type(self.inner, t.bits as i32) }
    }

    /// Gets the type of the `FileChooser`
    pub fn get_type(&self) -> FileChooserType {
        assert!(!self.inner.is_null());
        unsafe { mem::transmute(Fl_File_Chooser_type(self.inner)) }
    }

    /// Gets the user data of the `FileChooser`
    /// # Safety
    /// Can invalidate the user data while the `FileChooser` is in use
    pub unsafe fn user_data(&self) -> Option<Box<dyn FnMut()>> {
        let ptr = Fl_File_Chooser_user_data(self.inner);
        if ptr.is_null() {
            None
        } else {
            let x = ptr as *mut Box<dyn FnMut()>;
            let x = Box::from_raw(x);
            Fl_File_Chooser_set_callback(self.inner, None, std::ptr::null_mut());
            Some(*x)
        }
    }

    /// Gets the file or dir name chosen by the `FileChooser`
    pub fn value(&mut self, f: i32) -> Option<String> {
        assert!(!self.inner.is_null());
        let f = if f == 0 { 1 } else { f };
        unsafe {
            let ptr = Fl_File_Chooser_value(self.inner, f as i32);
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

    /// Sets the file or dir name chosen by the `FileChooser`
    pub fn set_value(&mut self, filename: &str) {
        assert!(!self.inner.is_null());
        let filename = CString::safe_new(filename);
        unsafe { Fl_File_Chooser_set_value(self.inner, filename.as_ptr()) }
    }

    /// Returns whether the `FileChooser` is visible or not
    pub fn visible(&self) -> bool {
        assert!(!self.inner.is_null());
        unsafe { Fl_File_Chooser_visible(self.inner) != 0 }
    }

    /// Return dialog window
    pub fn window(&self) -> impl WindowExt {
        // Shouldn't fail
        unsafe {
            let win_ptr = self
                .new_button()
                .unwrap()
                .parent()
                .unwrap()
                .parent()
                .unwrap()
                .as_widget_ptr();
            crate::window::Window::from_widget_ptr(win_ptr)
        }
    }

    /// Set "Add favorites" label
    pub fn set_add_favorites_label(msg: &'static str) {
        let msg = CString::safe_new(msg);
        unsafe { Fl_File_Chooser_set_add_favorites_label(msg.into_raw()) }
    }

    /// Set "All Files" label
    pub fn set_all_files_label(msg: &'static str) {
        let msg = CString::safe_new(msg);
        unsafe { Fl_File_Chooser_set_all_files_label(msg.into_raw()) }
    }

    /// Set "Custom Filter" label
    pub fn set_custom_filter_label(msg: &'static str) {
        let msg = CString::safe_new(msg);
        unsafe { Fl_File_Chooser_set_custom_filter_label(msg.into_raw()) }
    }

    /// Set "Existing file" label
    pub fn set_existing_file_label(msg: &'static str) {
        let msg = CString::safe_new(msg);
        unsafe { Fl_File_Chooser_set_existing_file_label(msg.into_raw()) }
    }

    /// Set "Favorites" label
    pub fn set_favorites_label(msg: &'static str) {
        let msg = CString::safe_new(msg);
        unsafe { Fl_File_Chooser_set_favorites_label(msg.into_raw()) }
    }

    /// Set "Filename" label
    pub fn set_filename_label(msg: &'static str) {
        let msg = CString::safe_new(msg);
        unsafe { Fl_File_Chooser_set_filename_label(msg.into_raw()) }
    }

    /// Set "Filesystems" label
    pub fn set_filesystems_label(msg: &'static str) {
        let msg = CString::safe_new(msg);
        unsafe { Fl_File_Chooser_set_filesystems_label(msg.into_raw()) }
    }

    /// Set "Manage favorites" label
    pub fn set_manage_favorites_label(msg: &'static str) {
        let msg = CString::safe_new(msg);
        unsafe { Fl_File_Chooser_set_manage_favorites_label(msg.into_raw()) }
    }

    /// Set "New directory" label
    pub fn set_new_directory_label(msg: &'static str) {
        let msg = CString::safe_new(msg);
        unsafe { Fl_File_Chooser_set_new_directory_label(msg.into_raw()) }
    }

    /// Set "New directory" tooltip
    pub fn set_new_directory_tooltip(msg: &'static str) {
        let msg = CString::safe_new(msg);
        unsafe { Fl_File_Chooser_set_new_directory_tooltip(msg.into_raw()) }
    }

    /// Set "Preview" label
    pub fn set_preview_label(msg: &'static str) {
        let msg = CString::safe_new(msg);
        unsafe { Fl_File_Chooser_set_preview_label(msg.into_raw()) }
    }

    /// Set "Save" label
    pub fn set_save_label(msg: &'static str) {
        let msg = CString::safe_new(msg);
        unsafe { Fl_File_Chooser_set_save_label(msg.into_raw()) }
    }

    /// Set "Show" label
    pub fn set_show_label(msg: &'static str) {
        let msg = CString::safe_new(msg);
        unsafe { Fl_File_Chooser_set_show_label(msg.into_raw()) }
    }

    /// Set "hidden" label
    pub fn set_hidden_label(msg: &'static str) {
        let msg = CString::safe_new(msg);
        unsafe { Fl_File_Chooser_set_hidden_label(msg.into_raw()) }
    }

    /// Set the position of the file chooser
    pub fn set_position(&mut self, x: i32, y: i32) {
        assert!(!self.inner.is_null());
        unsafe { Fl_File_Chooser_set_position(self.inner, x, y) }
    }

    /// Set the size of the file chooser
    pub fn set_size(&mut self, w: i32, h: i32) {
        assert!(!self.inner.is_null());
        unsafe { Fl_File_Chooser_set_size(self.inner, w, h) }
    }

    /// Get the x pos of the file chooser
    pub fn x(&self) -> i32 {
        assert!(!self.inner.is_null());
        unsafe { Fl_File_Chooser_x(self.inner) }
    }

    /// Get the y pos of the file chooser
    pub fn y(&self) -> i32 {
        assert!(!self.inner.is_null());
        unsafe { Fl_File_Chooser_y(self.inner) }
    }

    /// Get the width of the file chooser
    pub fn w(&self) -> i32 {
        assert!(!self.inner.is_null());
        unsafe { Fl_File_Chooser_w(self.inner) }
    }

    /// Get the width of the file chooser
    pub fn h(&self) -> i32 {
        assert!(!self.inner.is_null());
        unsafe { Fl_File_Chooser_h(self.inner) }
    }

    /// Get the size of the file chooser
    pub fn size(&self) -> (i32, i32) {
        (self.w(), self.h())
    }

    /// Get the position of the file chooser
    pub fn pos(&self) -> (i32, i32) {
        (self.x(), self.y())
    }
}

impl Drop for FileChooser {
    fn drop(&mut self) {
        unsafe { Fl_File_Chooser_delete(self.inner) }
    }
}

/// Shows a directory chooser returning a String
pub fn dir_chooser(message: &str, fname: &str, relative: bool) -> Option<String> {
    unsafe {
        let message = CString::safe_new(message);
        let fname = CString::safe_new(fname);
        let ptr = Fl_dir_chooser(message.as_ptr(), fname.as_ptr(), relative as i32);
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

/**
    Shows a file chooser returning a String.
    The pattern field takes the same argument the [`FileChooser::set_filter`](`crate::dialog::FileChooser::set_filter`) method.
    Example:
    ```rust,no_run
    use fltk::{prelude::*, *};
    fn main() {
        let app = app::App::default();
        let mut win = window::Window::default().with_size(900, 300);
        let file = dialog::file_chooser("Choose File", "*.rs", ".", true).unwrap();
        println!("{}", file);
        win.end();
        win.show();
        app.run().unwrap();
    }
    ```
*/
pub fn file_chooser<P: AsRef<Path>>(
    message: &str,
    pattern: &str,
    dir: P,
    relative: bool,
) -> Option<String> {
    file_chooser_(message, pattern, dir.as_ref(), relative)
}

fn file_chooser_(message: &str, pattern: &str, dir: &Path, relative: bool) -> Option<String> {
    if let Some(dir) = dir.to_str() {
        let message = CString::safe_new(message);
        let pattern = CString::safe_new(pattern);
        let dir = CString::safe_new(dir);
        unsafe {
            let ptr = Fl_file_chooser(
                message.as_ptr(),
                pattern.as_ptr(),
                dir.as_ptr(),
                relative as i32,
            );
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
    } else {
        None
    }
}

/// Spawns a color chooser dialog.
pub fn color_chooser(name: &str, cmode: ColorMode) -> Option<(u8, u8, u8)> {
    unsafe {
        let name = CString::safe_new(name);
        let mut r = 255;
        let mut g = 255;
        let mut b = 255;
        let ret = Fl_color_chooser(name.as_ptr(), &mut r, &mut g, &mut b, cmode as i32);
        if ret == 0 {
            None
        } else {
            Some((r, g, b))
        }
    }
}

/// Spawns a color chooser dialog.
pub fn color_chooser_with_default(name: &str, cmode: ColorMode, col: (u8, u8, u8)) -> (u8, u8, u8) {
    unsafe {
        let name = CString::safe_new(name);
        let mut r = col.0;
        let mut g = col.1;
        let mut b = col.2;
        let ret = Fl_color_chooser(name.as_ptr(), &mut r, &mut g, &mut b, cmode as i32);
        if ret == 0 {
            col
        } else {
            (r, g, b)
        }
    }
}

/// Set the next dialog's title
pub fn message_title(title: &str) {
    let title = CString::safe_new(title);
    unsafe { Fl_message_title(title.as_ptr() as _) }
}

/// Set the default title for a dialog
pub fn message_title_default(title: &str) {
    let title = CString::safe_new(title);
    unsafe { Fl_message_title_default(title.as_ptr() as _) }
}

/// Get the frame holding the icon of FLTK's dialog boxes
pub fn message_icon() -> impl WidgetExt {
    unsafe { crate::frame::Frame::from_widget_ptr(Fl_message_icon() as _) }
}

/// Set whether hotspot is enabled for FLTK's dialog boxes
pub fn message_set_hotspot(enabled: bool) {
    unsafe { Fl_message_set_hotspot(enabled as _) }
}

/// Get whether hotspot is enabled for FLTK's dialog boxes
pub fn message_hotspot() -> bool {
    unsafe { Fl_message_hotspot() != 0 }
}

/// Set the font and font size of FLTK's dialog boxes
pub fn message_set_font(font: Font, sz: i32) {
    unsafe { Fl_message_set_font(font.bits(), sz) }
}

/// Set the next dialog's icon label
pub fn message_icon_label(label: &str) {
    let label = CString::safe_new(label);
    unsafe { Fl_message_icon_label(label.into_raw() as _) }
}
