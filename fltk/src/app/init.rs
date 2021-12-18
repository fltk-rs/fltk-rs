use fltk_sys::fl;
use std::sync::{
    atomic::{AtomicBool, AtomicI32, Ordering},
    Arc, Mutex,
};

/// Basically a check for global locking
pub(crate) static mut IS_INIT: AtomicBool = AtomicBool::new(false);

/// Currently loaded fonts
pub(crate) static LOADED_FONT: Option<&'static str> = None;

/// The currently chosen font
pub(crate) static CURRENT_FONT: AtomicI32 = AtomicI32::new(0);

/// The currently chosen frame type
pub(crate) static CURRENT_FRAME: AtomicI32 = AtomicI32::new(2);

/// The fonts associated with the application
pub(crate) static mut FONTS: Option<Arc<Mutex<Vec<String>>>> = None;

pub(crate) static mut UI_THREAD: Option<std::thread::ThreadId> = None;

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
        fl::Fl_init_all();
        if fl::Fl_lock() != 0 {
            panic!("fltk-rs requires threading support!");
        }
        register_images();
        UI_THREAD = Some(std::thread::current().id());
        // This should never appear!
        FONTS = Some(Arc::from(Mutex::from(vec![
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
        ])));
        #[cfg(feature = "enable-glwindow")]
        {
            extern "C" {
                pub fn open_gl() -> i32;
            }
            open_gl();
        }
        if !IS_INIT.load(Ordering::Relaxed) {
            IS_INIT.store(true, Ordering::Relaxed);
        }
    }
}

/// Check whether we're in the ui thread
pub(crate) fn is_ui_thread() -> bool {
    unsafe { UI_THREAD.unwrap() == std::thread::current().id() }
}
