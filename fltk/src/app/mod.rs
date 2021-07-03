use crate::enums::Mode;
use crate::prelude::*;
use crate::window::Window;
use fltk_sys::fl;
use std::{mem, os::raw, panic, path, ptr, sync::atomic::Ordering};

mod channel;
pub use channel::*;
mod event;
pub use event::*;
mod font;
pub use font::*;
mod rt;
pub use rt::*;
mod screen;
pub use screen::*;
mod style;
pub use style::*;
mod version;
pub use version::*;

/// Alias Widget ptr
pub type WidgetPtr = *mut fltk_sys::widget::Fl_Widget;

/// Basic Application struct, used to instatiate, set the scheme and run the event loop
#[derive(Debug, Copy, Clone)]
pub struct App {}

impl App {
    /// Instantiates an App type
    pub fn default() -> App {
        init_all();
        App {}
    }

    /// Sets the scheme of the application
    pub fn set_scheme(&mut self, scheme: Scheme) {
        set_scheme(scheme);
    }

    /// Sets the scheme of the application
    pub fn with_scheme(self, scheme: Scheme) -> App {
        set_scheme(scheme);
        self
    }

    /// Gets the scheme of the application

    pub fn scheme(self) -> Scheme {
        scheme()
    }

    /// Runs the event loop
    /// # Errors
    /// Can error on failure to run the application
    pub fn run(self) -> Result<(), FltkError> {
        run()
    }

    /// Wait for incoming messages.
    /// Calls to redraw within wait require an explicit sleep
    pub fn wait(self) -> bool {
        wait()
    }

    /// Loads system fonts
    pub fn load_system_fonts(self) -> Self {
        *FONTS.lock().unwrap() = get_font_names();
        self
    }

    /**
        Loads a font from a path.
        On success, returns a String with the ttf Font Family name. The font's index is always 16.
        As such only one font can be loaded at a time.
        The font name can be used with `Font::by_name`, and index with `Font::by_index`.
        # Examples
        ```rust,no_run
        use fltk::{prelude::*, *};
        let app = app::App::default();
        let font = app.load_font("font.ttf").unwrap();
        let mut frame = frame::Frame::new(0, 0, 400, 100, "Hello");
        frame.set_label_font(enums::Font::by_name(&font));
        ```
        # Errors
        Returns `ResourceNotFound` if the Font file was not found
    */
    pub fn load_font<P: AsRef<path::Path>>(self, path: P) -> Result<String, FltkError> {
        Self::load_font_(path.as_ref())
    }

    fn load_font_(path: &path::Path) -> Result<String, FltkError> {
        if !path.exists() {
            return Err::<String, FltkError>(FltkError::Internal(FltkErrorKind::ResourceNotFound));
        }
        if let Some(p) = path.to_str() {
            let name = load_font(p)?;
            Ok(name)
        } else {
            Err(FltkError::Internal(FltkErrorKind::ResourceNotFound))
        }
    }

    /// Set the visual of the application
    /// # Errors
    /// Returns `FailedOperation` if FLTK failed to set the visual mode
    pub fn set_visual(self, mode: Mode) -> Result<(), FltkError> {
        set_visual(mode)
    }

    /// Redraws the app
    pub fn redraw(self) {
        redraw()
    }

    /// Quit the application
    pub fn quit(self) {
        quit()
    }
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
        lock().expect("fltk-rs requires threading support!");
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

/// Registers a function that will be called by the main thread during the next message handling cycle
pub fn awake_callback<F: FnMut() + 'static>(cb: F) {
    unsafe {
        unsafe extern "C" fn shim(data: *mut raw::c_void) {
            let a: *mut Box<dyn FnMut()> = data as *mut Box<dyn FnMut()>;
            let f: &mut (dyn FnMut()) = &mut **a;
            let _ = panic::catch_unwind(panic::AssertUnwindSafe(|| f()));
        }
        let a: *mut Box<dyn FnMut()> = Box::into_raw(Box::new(Box::new(cb)));
        let data: *mut raw::c_void = a as *mut raw::c_void;
        let callback: fl::Fl_Awake_Handler = Some(shim);
        fl::Fl_awake_callback(callback, data);
    }
}

/// Get the grabbed window
pub fn grab() -> Option<impl WindowExt> {
    unsafe {
        let ptr = fl::Fl_grab();
        if ptr.is_null() {
            None
        } else {
            Some(crate::window::Window::from_widget_ptr(ptr as *mut _))
        }
    }
}

/// Set the current grab
pub fn set_grab<W: WindowExt>(win: Option<W>) {
    unsafe {
        win.map_or_else(
            || fl::Fl_set_grab(ptr::null_mut()),
            |w| fl::Fl_set_grab(w.as_widget_ptr() as *mut _),
        )
    }
}

/// Sets the callback of a widget
pub fn set_callback<F, W>(widget: &mut W, cb: F)
where
    F: FnMut(&mut dyn WidgetExt),
    W: WidgetExt,
{
    assert!(!widget.was_deleted());
    unsafe {
        unsafe extern "C" fn shim(wid: *mut fltk_sys::widget::Fl_Widget, data: *mut raw::c_void) {
            let a: *mut Box<dyn FnMut(&mut dyn WidgetExt)> =
                data as *mut Box<dyn FnMut(&mut dyn WidgetExt)>;
            let f: &mut (dyn FnMut(&mut dyn WidgetExt)) = &mut **a;
            let mut wid = crate::widget::Widget::from_widget_ptr(wid);
            let _ = panic::catch_unwind(panic::AssertUnwindSafe(|| f(&mut wid)));
        }
        let _old_data = widget.user_data();
        let a: *mut Box<dyn FnMut(&mut dyn WidgetExt)> = Box::into_raw(Box::new(Box::new(cb)));
        let data: *mut raw::c_void = a as *mut raw::c_void;
        let callback: fltk_sys::widget::Fl_Callback = Some(shim);
        fltk_sys::widget::Fl_Widget_set_callback(widget.as_widget_ptr(), callback, data);
    }
}

#[allow(clippy::missing_safety_doc)]
/**
    Set a widget callback using a C style API
    ```rust,no_run
    use fltk::{prelude::*, *};
    use std::os::raw::*;
    // data can be anything, even a different widget
    fn cb(w: app::WidgetPtr, data: *mut c_void) {
        // To access the button
        let mut btn = unsafe { button::Button::from_widget_ptr(w) }; // Gets a Widget
        btn.set_label("Works!");
        // To access the frame
        let mut frm = unsafe { widget::Widget::from_widget_ptr(data as app::WidgetPtr) };
        frm.set_label("Works!");
    }
    let mut but = button::Button::default();
    let mut frame = frame::Frame::default();
    unsafe {
        // If no data needs to be passed, you can pass 0 as *mut _
        app::set_raw_callback(&mut but, frame.as_widget_ptr() as *mut _, Some(cb));
        // Using a closure also works
        app::set_raw_callback(&mut but, frame.as_widget_ptr() as *mut _, Some(|_ , _| { println!("Also works!")}));
    }
    ```
    # Safety
    The function involves dereferencing externally provided raw pointers
*/
pub unsafe fn set_raw_callback<W>(
    widget: &mut W,
    data: *mut raw::c_void,
    cb: Option<fn(WidgetPtr, *mut raw::c_void)>,
) where
    W: WidgetExt,
{
    assert!(!widget.was_deleted());
    let cb: Option<unsafe extern "C" fn(WidgetPtr, *mut raw::c_void)> = mem::transmute(cb);
    fltk_sys::widget::Fl_Widget_set_callback(widget.as_widget_ptr(), cb, data);
}

/// Add an idle callback to run within the event loop.
/// Calls to `WidgetExt::redraw` within the callback require an explicit sleep
pub fn add_idle<F: FnMut() + 'static>(cb: F) {
    unsafe {
        unsafe extern "C" fn shim(data: *mut raw::c_void) {
            let a: *mut Box<dyn FnMut()> = data as *mut Box<dyn FnMut()>;
            let f: &mut (dyn FnMut()) = &mut **a;
            let _ = panic::catch_unwind(panic::AssertUnwindSafe(|| f()));
        }
        let a: *mut Box<dyn FnMut()> = Box::into_raw(Box::new(Box::new(cb)));
        let data: *mut raw::c_void = a as *mut raw::c_void;
        let callback: Option<unsafe extern "C" fn(arg1: *mut raw::c_void)> = Some(shim);
        fl::Fl_add_idle(callback, data);
    }
}

/// Remove an idle function
pub fn remove_idle<F: FnMut() + 'static>(cb: F) {
    unsafe {
        unsafe extern "C" fn shim(data: *mut raw::c_void) {
            let a: *mut Box<dyn FnMut()> = data as *mut Box<dyn FnMut()>;
            let f: &mut (dyn FnMut()) = &mut **a;
            let _ = panic::catch_unwind(panic::AssertUnwindSafe(|| f()));
        }
        let a: *mut Box<dyn FnMut()> = Box::into_raw(Box::new(Box::new(cb)));
        let data: *mut raw::c_void = a as *mut raw::c_void;
        let callback: Option<unsafe extern "C" fn(arg1: *mut raw::c_void)> = Some(shim);
        fl::Fl_remove_idle(callback, data);
    }
}

/// Checks whether an idle function is installed
pub fn has_idle<F: FnMut() + 'static>(cb: F) -> bool {
    unsafe {
        unsafe extern "C" fn shim(data: *mut raw::c_void) {
            let a: *mut Box<dyn FnMut()> = data as *mut Box<dyn FnMut()>;
            let f: &mut (dyn FnMut()) = &mut **a;
            let _ = panic::catch_unwind(panic::AssertUnwindSafe(|| f()));
        }
        let a: *mut Box<dyn FnMut()> = Box::into_raw(Box::new(Box::new(cb)));
        let data: *mut raw::c_void = a as *mut raw::c_void;
        let callback: Option<unsafe extern "C" fn(arg1: *mut raw::c_void)> = Some(shim);
        fl::Fl_has_idle(callback, data) != 0
    }
}

/// Returns the first window of the application
pub fn first_window() -> Option<impl WindowExt> {
    unsafe {
        let x = fl::Fl_first_window();
        if x.is_null() {
            None
        } else {
            let x = Window::from_widget_ptr(x as *mut fltk_sys::widget::Fl_Widget);
            Some(x)
        }
    }
}

/// Returns the next window in order
pub fn next_window<W: WindowExt>(w: &W) -> Option<impl WindowExt> {
    unsafe {
        let x = fl::Fl_next_window(w.as_widget_ptr() as *const raw::c_void);
        if x.is_null() {
            None
        } else {
            let x = Window::from_widget_ptr(x as *mut fltk_sys::widget::Fl_Widget);
            Some(x)
        }
    }
}

/// Quit the app
pub fn quit() {
    if let Some(loaded_font) = *LOADED_FONT {
        // Shouldn't fail
        unload_font(loaded_font).unwrap_or(());
    }
    if let Some(wins) = windows() {
        for mut i in wins {
            if i.shown() {
                i.hide();
            }
        }
    }
}

/**
    Adds a one-shot timeout callback. The timeout duration `tm` is indicated in seconds
    Example:
    ```rust,no_run
    use fltk::{prelude::*, *};
    fn callback() {
        println!("TICK");
        app::repeat_timeout(1.0, callback);
    }
    fn main() {
        let app = app::App::default();
        let mut wind = window::Window::new(100, 100, 400, 300, "");
        wind.show();
        app::add_timeout(1.0, callback);
        app.run().unwrap();
    }
    ```
*/
pub fn add_timeout<F: FnMut() + 'static>(tm: f64, cb: F) {
    unsafe {
        unsafe extern "C" fn shim(data: *mut raw::c_void) {
            let a: *mut Box<dyn FnMut()> = data as *mut Box<dyn FnMut()>;
            let f: &mut (dyn FnMut()) = &mut **a;
            let _ = panic::catch_unwind(panic::AssertUnwindSafe(|| f()));
        }
        let a: *mut Box<dyn FnMut()> = Box::into_raw(Box::new(Box::new(cb)));
        let data: *mut raw::c_void = a as *mut raw::c_void;
        let callback: Option<unsafe extern "C" fn(arg1: *mut raw::c_void)> = Some(shim);
        fltk_sys::fl::Fl_add_timeout(tm, callback, data);
    }
}

/**
    Repeats a timeout callback from the expiration of the previous timeout.
    You may only call this method inside a timeout callback.
    The timeout duration `tm` is indicated in seconds
    Example:
    ```rust,no_run
    use fltk::{prelude::*, *};
    fn callback() {
        println!("TICK");
        app::repeat_timeout(1.0, callback);
    }
    fn main() {
        let app = app::App::default();
        let mut wind = window::Window::new(100, 100, 400, 300, "");
        wind.show();
        app::add_timeout(1.0, callback);
        app.run().unwrap();
    }
    ```
*/
pub fn repeat_timeout<F: FnMut() + 'static>(tm: f64, cb: F) {
    unsafe {
        unsafe extern "C" fn shim(data: *mut raw::c_void) {
            let a: *mut Box<dyn FnMut()> = data as *mut Box<dyn FnMut()>;
            let f: &mut (dyn FnMut()) = &mut **a;
            let _ = panic::catch_unwind(panic::AssertUnwindSafe(|| f()));
        }
        let a: *mut Box<dyn FnMut()> = Box::into_raw(Box::new(Box::new(cb)));
        let data: *mut raw::c_void = a as *mut raw::c_void;
        let callback: Option<unsafe extern "C" fn(arg1: *mut raw::c_void)> = Some(shim);
        fltk_sys::fl::Fl_repeat_timeout(tm, callback, data);
    }
}

/// Removes a timeout callback
pub fn remove_timeout<F: FnMut() + 'static>(cb: F) {
    unsafe {
        unsafe extern "C" fn shim(data: *mut raw::c_void) {
            let a: *mut Box<dyn FnMut()> = data as *mut Box<dyn FnMut()>;
            let f: &mut (dyn FnMut()) = &mut **a;
            let _ = panic::catch_unwind(panic::AssertUnwindSafe(|| f()));
        }
        let a: *mut Box<dyn FnMut()> = Box::into_raw(Box::new(Box::new(cb)));
        let data: *mut raw::c_void = a as *mut raw::c_void;
        let callback: Option<unsafe extern "C" fn(arg1: *mut raw::c_void)> = Some(shim);
        fltk_sys::fl::Fl_remove_timeout(callback, data);
    }
}

/// Check whether a timeout is installed
pub fn has_timeout<F: FnMut() + 'static>(cb: F) -> bool {
    unsafe {
        unsafe extern "C" fn shim(data: *mut raw::c_void) {
            let a: *mut Box<dyn FnMut()> = data as *mut Box<dyn FnMut()>;
            let f: &mut (dyn FnMut()) = &mut **a;
            let _ = panic::catch_unwind(panic::AssertUnwindSafe(|| f()));
        }
        let a: *mut Box<dyn FnMut()> = Box::into_raw(Box::new(Box::new(cb)));
        let data: *mut raw::c_void = a as *mut raw::c_void;
        let callback: Option<unsafe extern "C" fn(arg1: *mut raw::c_void)> = Some(shim);
        fltk_sys::fl::Fl_has_timeout(callback, data) != 0
    }
}

/// Deletes widgets and their children.
pub fn delete_widget<Wid: WidgetBase>(wid: Wid) {
    assert!(!wid.was_deleted());
    WidgetBase::delete(wid)
}

/// Redraws everything
pub fn redraw() {
    unsafe { fl::Fl_redraw() }
}

/// Sets the damage to true or false, illiciting a redraw by the application
pub fn set_damage(flag: bool) {
    unsafe { fl::Fl_set_damage(flag as i32) }
}

/// Returns whether any of the widgets were damaged
pub fn damage() -> bool {
    unsafe { fl::Fl_damage() != 0 }
}

/// Sets the visual mode of the application
/// # Errors
/// Returns Err(FailedOperation) if FLTK failed to set the visual mode
pub fn set_visual(mode: Mode) -> Result<(), FltkError> {
    unsafe {
        match fl::Fl_visual(mode.bits() as i32) {
            0 => Err(FltkError::Internal(FltkErrorKind::FailedOperation)),
            _ => Ok(()),
        }
    }
}

/// Gets the widget which was pushed
pub fn pushed() -> Option<impl WidgetExt> {
    unsafe {
        let ptr = fl::Fl_pushed();
        if ptr.is_null() {
            None
        } else {
            Some(crate::widget::Widget::from_widget_ptr(ptr as *mut _))
        }
    }
}

/// Gets the widget which has focus
pub fn focus() -> Option<impl WidgetExt> {
    unsafe {
        let ptr = fl::Fl_focus();
        if ptr.is_null() {
            None
        } else {
            Some(crate::widget::Widget::from_widget_ptr(
                ptr as *mut fltk_sys::widget::Fl_Widget,
            ))
        }
    }
}

/// Sets the widget which has focus
pub fn set_focus<W: WidgetExt>(wid: &W) {
    unsafe { fl::Fl_set_focus(wid.as_widget_ptr() as *mut raw::c_void) }
}

/// The current graphics context of the app, `fl_gc`.
/// `*mut c_void` to `HDC` on Windows, `CGContextRef` on macOS, `_XGC` on X11
pub type GraphicsContext = *mut raw::c_void;

/// Get the graphics context, `fl_gc`
pub fn graphics_context() -> GraphicsContext {
    unsafe {
        let ctx = fltk_sys::window::Fl_gc();
        assert!(!ctx.is_null());
        ctx
    }
}

/// The display global variable, `fl_display`.
/// `_XDisplay` on X11, `HINSTANCE` on Windows.
pub type Display = *mut raw::c_void;

/// Gets the display global variable, `fl_display`.
/// `_XDisplay` on X11, `HINSTANCE` on Windows.
pub fn display() -> Display {
    unsafe {
        let disp = fltk_sys::window::Fl_display();
        assert!(!disp.is_null());
        disp
    }
}

/// Returns the apps windows.
pub fn windows() -> Option<Vec<impl WindowExt>> {
    let mut v: Vec<Window> = vec![];
    if let Some(first) = first_window() {
        let first: Window = unsafe { first.into_widget() };
        v.push(first.clone());
        let mut win = first;
        while let Some(wind) = next_window(&win) {
            let w = unsafe { wind.into_widget::<Window>() };
            v.push(w.clone());
            win = w;
        }
        Some(v)
    } else {
        None
    }
}

/**
    Send a signal to a window.
    Integral values from 0 to 30 are reserved.
    Returns Ok(true) if the event was handled.
    Returns Ok(false) if the event was not handled.
    Returns Err on error or in use of one of the reserved values.
    ```rust,no_run
    use fltk::{prelude::*, *};
    const CHANGE_FRAME: i32 = 100;
    let mut wind = window::Window::default();
    let mut but = button::Button::default();
    let mut frame = frame::Frame::default();
    but.set_callback(move |_| {
        let _ = app::handle(CHANGE_FRAME, &wind).unwrap();
    });
    frame.handle(move |f, ev| {
        if ev == CHANGE_FRAME.into() {
            f.set_label("Hello world");
            true
        } else {
            false
        }
    });
    ```
    # Errors
    Returns Err on error or in use of one of the reserved values.
*/
pub fn handle<I: Into<i32> + Copy + PartialEq + PartialOrd, W: WindowExt>(
    msg: I,
    w: &W,
) -> Result<bool, FltkError> {
    let val = msg.into();
    if (0..=30).contains(&val) {
        Err(FltkError::Internal(FltkErrorKind::FailedOperation))
    } else {
        let ret = unsafe { fl::Fl_handle(val, w.as_widget_ptr() as _) != 0 };
        Ok(ret)
    }
}

/**
    Send a signal to the main window.
    Integral values from 0 to 30 are reserved.
    Returns Ok(true) if the event was handled.
    Returns Ok(false) if the event was not handled.
    ```rust,no_run
    use fltk::{prelude::*, *};
    const CHANGE_FRAME: i32 = 100;
    let mut wind = window::Window::default();
    let mut but = button::Button::default();
    let mut frame = frame::Frame::default();
    but.set_callback(move |_| {
        let _ = app::handle_main(CHANGE_FRAME).unwrap();
    });
    frame.handle(move |f, ev| {
        if ev == CHANGE_FRAME.into() {
            f.set_label("Hello world");
            true
        } else {
            false
        }
    });
    ```
    # Errors
    Returns Err on error or in use of one of the reserved values.
*/
pub fn handle_main<I: Into<i32> + Copy + PartialEq + PartialOrd>(
    msg: I,
) -> Result<bool, FltkError> {
    let val = msg.into();
    if (0..=30).contains(&val) {
        Err(FltkError::Internal(FltkErrorKind::FailedOperation))
    } else {
        first_window().map_or(
            Err(FltkError::Internal(FltkErrorKind::FailedOperation)),
            |win| {
                let ret = unsafe { fl::Fl_handle(val, win.as_widget_ptr() as _) != 0 };
                Ok(ret)
            },
        )
    }
}

/// Causes all the windows that need it to be redrawn and graphics forced out through the pipes.
pub fn flush() {
    unsafe { fl::Fl_flush() }
}

/// Open the current display
/// # Safety
/// A correct visual must be set prior to opening the display
pub unsafe fn open_display() {
    fl::Fl_open_display()
}

/// Close the current display
/// # Safety
/// The display shouldn't be closed while a window is shown
pub unsafe fn close_display() {
    fl::Fl_close_display()
}

/// Unset the currently grabbed window
pub fn release() {
    unsafe { fl::Fl_release() }
}
