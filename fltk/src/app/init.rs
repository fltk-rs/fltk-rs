use fltk_sys::fl;
use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        Mutex,
    },
};

lazy_static! {
    /// The currently chosen font
    pub(crate) static ref CURRENT_FONT: Mutex<i32> = Mutex::new(0);

    /// Currently loaded fonts
    pub(crate) static ref LOADED_FONT: Option<&'static str> = None;

    /// The fonts associated with the application
    pub(crate) static ref FONTS: Mutex<Vec<String>> = Mutex::new(Vec::new());

    /// The currently chosen frame type
    pub(crate) static ref CURRENT_FRAME: Mutex<i32> = Mutex::new(2);

    /// Basically a check for global locking
    pub(crate) static ref IS_INIT: AtomicBool = AtomicBool::new(false);
}

/// Registers all images supported by `SharedImage`
pub(crate) fn register_images() {
    unsafe { fltk_sys::image::Fl_register_images() }
}

/// Inits all styles, fonts and images available to FLTK.
/// Also initializes global locking
/// # Panics
/// If the current environment lacks threading support. Practically this should never happen!
pub fn init_all() {
    unsafe {
        fltk_sys::fl::Fl_init_all();
        if fl::Fl_lock() != 0 {
            panic!("fltk-rs requires threading support!");
        }
        register_images();
        // This should never appear!
        *FONTS.lock().unwrap() = vec![
            "Helvetica".to_owned(),
            "HelveticaBold".to_owned(),
            "HelveticaItalic".to_owned(),
            "HelveticaBoldItalic".to_owned(),
            "Courier".to_owned(),
            "CourierBold".to_owned(),
            "CourierItalic".to_owned(),
            "CourierBoldItalic".to_owned(),
            "Times".to_owned(),
            "TimesBold".to_owned(),
            "TimesItalic".to_owned(),
            "TimesBoldItalic".to_owned(),
            "Symbol".to_owned(),
            "Screen".to_owned(),
            "ScreenBold".to_owned(),
            "Zapfdingbats".to_owned(),
        ];
        #[cfg(feature = "enable-glwindow")]
        {
            gl_loader::init_gl();
        }
        if !IS_INIT.load(Ordering::Relaxed) {
            IS_INIT.store(true, Ordering::Relaxed);
        }
    }
}
