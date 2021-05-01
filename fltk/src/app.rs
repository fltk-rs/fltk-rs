use crate::enums::{Event, Font, FrameType, Key, Mode, Shortcut};
use crate::prelude::*;
use crate::utils::FlString;
use crate::window::Window;
use fltk_sys::fl;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::{
    any, cmp,
    ffi::{CStr, CString},
    marker, mem,
    os::raw,
    panic, path, ptr,
    sync::{
        atomic::{AtomicBool, Ordering},
        Mutex,
    },
    thread, time,
};

/// Alias Widget ptr
pub type WidgetPtr = *mut fltk_sys::widget::Fl_Widget;

lazy_static! {
    /// The currently chosen font
    static ref CURRENT_FONT: Mutex<i32> = Mutex::new(0);

    /// The currently chosen frame type
    static ref CURRENT_FRAME: Mutex<i32> = Mutex::new(2);

    /// Currently loaded fonts
    static ref LOADED_FONT: Option<&'static str> = None;

    /// Basically a check for global locking
    static ref IS_INIT: AtomicBool = AtomicBool::new(false);

    /// The fonts associated with the application
    pub(crate) static ref FONTS: Mutex<Vec<String>> = Mutex::new(Vec::new());
}

/// Runs the event loop
/// # Errors
/// Returns `FailedToRun`, this is fatal to the app
pub fn run() -> Result<(), FltkError> {
    unsafe {
        if !IS_INIT.load(Ordering::Relaxed) {
            init_all();
        }
        match fl::Fl_run() {
            0 => Ok(()),
            _ => Err(FltkError::Internal(FltkErrorKind::FailedToRun)),
        }
    }
}

/// Locks the main UI thread
/// # Errors
/// Returns `FailedToLock` if locking is unsopported. This is fatal to the app
pub fn lock() -> Result<(), FltkError> {
    unsafe {
        match fl::Fl_lock() {
            0 => Ok(()),
            _ => Err(FltkError::Internal(FltkErrorKind::FailedToLock)),
        }
    }
}

/// Set the app scheme
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Scheme {
    /// Base fltk scheming
    Base,
    /// inspired by the Aqua user interface on Mac OS X
    Plastic,
    /// inspired by the GTK+ theme
    Gtk,
    /// inspired by the Clearlooks Glossy scheme
    Gleam,
}

/// sets the scheme of the application
pub fn set_scheme(scheme: Scheme) {
    let name_str = match scheme {
        Scheme::Base => "base",
        Scheme::Gtk => "gtk+",
        Scheme::Gleam => "gleam",
        Scheme::Plastic => "plastic",
    };
    let name_str = CString::safe_new(name_str);
    unsafe { fl::Fl_set_scheme(name_str.as_ptr()) }
}

/// Gets the scheme of the application
pub fn scheme() -> Scheme {
    unsafe {
        use Scheme::{Base, Gleam, Gtk, Plastic};
        match fl::Fl_scheme() {
            0 => Base,
            1 => Gtk,
            2 => Gleam,
            3 => Plastic,
            _ => unreachable!(),
        }
    }
}

/// Alias Scheme to `AppScheme`
pub type AppScheme = Scheme;

/// Unlocks the main UI thread
pub fn unlock() {
    unsafe {
        fl::Fl_unlock();
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

/// Trigger event loop handling in the main thread
pub fn awake() {
    unsafe { fl::Fl_awake() }
}

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

    /// Loads a font from a path.
    /// On success, returns a String with the ttf Font Family name. The font's index is always 16.
    /// As such only one font can be loaded at a time.
    /// The font name can be used with `Font::by_name`, and index with `Font::by_index`.
    /// # Examples
    /// ```rust,no_run
    /// use fltk::{prelude::*, *};
    /// let app = app::App::default();
    /// let font = app.load_font("font.ttf").unwrap();
    /// let mut frame = frame::Frame::new(0, 0, 400, 100, "Hello");
    /// frame.set_label_font(enums::Font::by_name(&font));
    /// ```
    /// # Errors
    /// Returns `ResourceNotFound` if the Font file was not found
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

/// Set the application's scrollbar size
pub fn set_scrollbar_size(sz: i32) {
    unsafe { fl::Fl_set_scrollbar_size(sz as i32) }
}

/// Get the app's scrollbar size
pub fn scrollbar_size() -> i32 {
    unsafe { fl::Fl_scrollbar_size() as i32 }
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

/// Returns the latest captured event
pub fn event() -> Event {
    unsafe {
        let x = fl::Fl_event();
        let x: Event = mem::transmute(x);
        x
    }
}

/// Returns the presed key
pub fn event_key() -> Key {
    unsafe {
        let x = fl::Fl_event_key();
        mem::transmute(x)
    }
}

/// Returns whether the  key is pressed or held down during the last event
pub fn event_key_down(key: Key) -> bool {
    unsafe { fl::Fl_event_key_down(mem::transmute(key)) != 0 }
}

/// Returns a textual representation of the latest event
pub fn event_text() -> String {
    unsafe {
        let text = fl::Fl_event_text();
        if text.is_null() {
            String::from("")
        } else {
            CStr::from_ptr(text as *mut raw::c_char)
                .to_string_lossy()
                .to_string()
        }
    }
}

/// Returns the captured button event.
/// 1 for left key, 2 for middle, 3 for right
pub fn event_button() -> i32 {
    unsafe { fl::Fl_event_button() }
}

/// Defines Mouse buttons
#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq)]
#[non_exhaustive]
pub enum MouseButton {
    /// Left mouse button
    Left = 1,
    /// Middle mouse button
    Middle = 2,
    /// Right mouse button
    Right = 3,
}

/// Returns the captured button event
pub fn event_mouse_button() -> MouseButton {
    unsafe { mem::transmute(fl::Fl_event_button()) }
}

/// Returns the number of clicks
pub fn event_clicks() -> bool {
    unsafe { fl::Fl_event_clicks() != 0 }
}

/// Gets the x coordinate of the mouse in the window
pub fn event_x() -> i32 {
    unsafe { fl::Fl_event_x() }
}

/// Gets the y coordinate of the mouse in the window
pub fn event_y() -> i32 {
    unsafe { fl::Fl_event_y() }
}

/// Gets the x coordinate of the mouse in the screen
pub fn event_x_root() -> i32 {
    unsafe { fl::Fl_event_x_root() }
}

/// Gets the y coordinate of the mouse in the screen
pub fn event_y_root() -> i32 {
    unsafe { fl::Fl_event_y_root() }
}

/// Event direction with Mousewheel event
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum MouseWheel {
    /// No movement
    None,
    /// Right movement
    Right,
    /// Left movement
    Left,
    /// Up movement
    Up,
    /// Down movement
    Down,
}

/// Returns the current horizontal mouse scrolling associated with the Mousewheel event.
/// Returns `MouseWheel::None`, `Right` or `Left`
pub fn event_dx() -> MouseWheel {
    match 0.cmp(unsafe { &fl::Fl_event_dx() }) {
        cmp::Ordering::Greater => MouseWheel::Right,
        cmp::Ordering::Equal => MouseWheel::None,
        cmp::Ordering::Less => MouseWheel::Left,
    }
}

/// Returns the current horizontal mouse scrolling associated with the Mousewheel event.
/// Returns `MouseWheel::None`, `Up` or `Down`.
/// Doesn't indicate scrolling direction which depends on system preferences
pub fn event_dy() -> MouseWheel {
    match 0.cmp(unsafe { &fl::Fl_event_dy() }) {
        cmp::Ordering::Greater => MouseWheel::Down,
        cmp::Ordering::Equal => MouseWheel::None,
        cmp::Ordering::Less => MouseWheel::Up,
    }
}

/// Gets the mouse coordinates relative to the screen
pub fn get_mouse() -> (i32, i32) {
    unsafe {
        let mut x: i32 = 0;
        let mut y: i32 = 0;
        fl::Fl_get_mouse(&mut x, &mut y);
        (x, y)
    }
}

/// Returns the x and y coordinates of the captured event
pub fn event_coords() -> (i32, i32) {
    unsafe { (fl::Fl_event_x(), fl::Fl_event_y()) }
}

/// Determines whether an event was a click
pub fn event_is_click() -> bool {
    unsafe { fl::Fl_event_is_click() != 0 }
}

/// Returns the duration of an event
pub fn event_length() -> i32 {
    unsafe { fl::Fl_event_length() as i32 }
}

/// Returns the state of the event
pub fn event_state() -> Shortcut {
    unsafe { mem::transmute(fl::Fl_event_state()) }
}

/// Returns a pair of the width and height of the screen
pub fn screen_size() -> (f64, f64) {
    unsafe {
        (
            (fl::Fl_screen_w() as f64),
            (fl::Fl_screen_h() as f64),
        )
    }
}

/// Returns a pair of the x & y coords of the screen
pub fn screen_coords() -> (i32, i32) {
    unsafe {
        (fl::Fl_screen_x(), fl::Fl_screen_y())
    }
}

/// Used for widgets implementing the `InputExt`, pastes content from the clipboard
pub fn paste<T>(widget: &T)
where
    T: WidgetBase + InputExt,
{
    assert!(!widget.was_deleted());
    unsafe {
        fl::Fl_paste(widget.as_widget_ptr() as *mut fltk_sys::fl::Fl_Widget, 1);
    }
}

/// Sets the callback of a widget
pub fn set_callback<F, W>(widget: &mut W, cb: F)
where
    F: FnMut(),
    W: WidgetExt,
{
    assert!(!widget.was_deleted());
    unsafe {
        unsafe extern "C" fn shim(_wid: *mut fltk_sys::widget::Fl_Widget, data: *mut raw::c_void) {
            let a: *mut Box<dyn FnMut()> = data as *mut Box<dyn FnMut()>;
            let f: &mut (dyn FnMut()) = &mut **a;
            let _ = panic::catch_unwind(panic::AssertUnwindSafe(|| f()));
        }
        let _old_data = widget.user_data();
        let a: *mut Box<dyn FnMut()> = Box::into_raw(Box::new(Box::new(cb)));
        let data: *mut raw::c_void = a as *mut raw::c_void;
        let callback: fltk_sys::widget::Fl_Callback = Some(shim);
        fltk_sys::widget::Fl_Widget_set_callback(widget.as_widget_ptr(), callback, data);
    }
}

/// Set a widget callback using a C style API
/// ```rust,no_run
/// use fltk::{prelude::*, *};
/// use std::os::raw::*;
/// // data can be anything, even a different widget
/// fn cb(w: app::WidgetPtr, data: *mut c_void) {
///     // To access the button
///     let mut btn = unsafe { button::Button::from_widget_ptr(w) }; // Gets a Widget
///     btn.set_label("Works!");
///     // To access the frame
///     let mut frm = unsafe { widget::Widget::from_widget_ptr(data as app::WidgetPtr) };
///     frm.set_label("Works!");
/// }
/// let mut but = button::Button::default();
/// let mut frame = frame::Frame::default();
/// unsafe {
///     // If no data needs to be passed, you can pass 0 as *mut _
///     app::set_raw_callback(&mut but, frame.as_widget_ptr() as *mut _, Some(cb));
///     // Using a closure also works
///     app::set_raw_callback(&mut but, frame.as_widget_ptr() as *mut _, Some(|_ , _| { println!("Also works!")}));
/// }
/// ```
/// # Safety
/// The function involves dereferencing externally provided raw pointers
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

/// Return whether visible focus is shown
pub fn visible_focus() -> bool {
    unsafe { fl::Fl_visible_focus() != 0 }
}

/// Show focus around widgets
pub fn set_visible_focus(flag: bool) {
    unsafe { fl::Fl_set_visible_focus(flag as i32) }
}

/// Set the app's default frame type
pub fn set_frame_type(new_frame: FrameType) {
    unsafe {
        let new_frame = new_frame as i32;
        let mut curr = CURRENT_FRAME.lock().unwrap();
        fl::Fl_set_box_type(56, *curr);
        fl::Fl_set_box_type(*curr, new_frame);
        fl::Fl_set_box_type(new_frame, 56);
        *curr = new_frame;
    }
}

/// Set the app's font
pub fn set_font(new_font: Font) {
    unsafe {
        let new_font = new_font.bits() as i32;
        let mut f = CURRENT_FONT.lock().unwrap();
        fl::Fl_set_font(15, *f);
        fl::Fl_set_font(0, new_font);
        fl::Fl_set_font(new_font, *f);
        *f = new_font;
    }
}

/// Set the app's font size
pub fn set_font_size(sz: u8) {
    unsafe { fl::Fl_set_font_size(sz as i32) }
}

/// Get the font's name
pub fn get_font(font: Font) -> String {
    unsafe {
        CStr::from_ptr(fl::Fl_get_font(font.bits() as i32))
            .to_string_lossy()
            .to_string()
    }
}

/// Initializes loaded fonts of a certain pattern `name`
pub fn set_fonts(name: &str) -> u8 {
    let name = CString::safe_new(name);
    unsafe { fl::Fl_set_fonts(name.as_ptr() as *mut raw::c_char) as u8 }
}

/// Gets the name of a font through its index
pub fn font_name(idx: usize) -> Option<String> {
    let f = FONTS.lock().unwrap();
    Some(f[idx].clone())
}

/// Returns a list of available fonts to the application
pub fn get_font_names() -> Vec<String> {
    let mut vec: Vec<String> = vec![];
    let cnt = set_fonts("*") as usize;
    for i in 0..cnt {
        let temp = unsafe {
            CStr::from_ptr(fl::Fl_get_font(i as i32))
                .to_string_lossy()
                .to_string()
        };
        vec.push(temp);
    }
    vec
}

/// Finds the index of a font through its name
pub fn font_index(name: &str) -> Option<usize> {
    let f = FONTS.lock().unwrap();
    f.iter().position(|i| i == name)
}

/// Gets the number of loaded fonts
pub fn font_count() -> usize {
    (*FONTS.lock().unwrap()).len()
}

/// Gets a Vector<String> of loaded fonts
pub fn fonts() -> Vec<String> {
    (*FONTS.lock().unwrap()).clone()
}

/// Adds a custom handler for unhandled events
pub fn add_handler(cb: fn(Event) -> bool) {
    unsafe {
        let callback: Option<unsafe extern "C" fn(ev: raw::c_int) -> raw::c_int> =
            Some(mem::transmute(move |ev| {
                let _ = panic::catch_unwind(panic::AssertUnwindSafe(|| cb(ev) as i32));
            }));
        fl::Fl_add_handler(callback);
    }
}

/// Starts waiting for events.
/// Calls to redraw within wait require an explicit sleep
pub fn wait() -> bool {
    unsafe {
        if !IS_INIT.load(Ordering::Relaxed) {
            init_all();
        }
        fl::Fl_wait() != 0
    }
}

/// Put the thread to sleep for `dur` seconds
pub fn sleep(dur: f64) {
    let dur = dur * 1000.;
    thread::sleep(time::Duration::from_millis(dur as u64));
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

/// Waits a maximum of `dur` seconds or until "something happens".
/// Returns true if an event happened (always true on windows).
/// Returns false if nothing happened.
/// # Errors
/// Can error out on X11 system if interrupted by a signal
pub fn wait_for(dur: f64) -> Result<bool, FltkError> {
    unsafe {
        if !IS_INIT.load(Ordering::Relaxed) {
            init_all();
        }
        match fl::Fl_wait_for(dur) as i32 {
            0 => Ok(false),
            1 => Ok(true),
            _ => Err(FltkError::Unknown(String::from(
                "An unknown error occured!",
            ))),
        }
    }
}

/// Sends a custom message
/// # Safety
/// The type must be Send and Sync safe
pub unsafe fn awake_msg<T>(msg: T) {
    fl::Fl_awake_msg(Box::into_raw(Box::from(msg)) as *mut raw::c_void);
}

/// Receives a custom message
/// ```rust,no_run
/// use fltk::{prelude::*, *};
/// if let Some(msg) = unsafe { app::thread_msg::<i32>() } { /* do something */ }
/// ```
/// # Safety
/// The type must correspond to the received message
pub unsafe fn thread_msg<T>() -> Option<T> {
    let msg = fl::Fl_thread_msg();
    if msg.is_null() {
        None
    } else {
        let msg = Box::from_raw(msg as *const _ as *mut T);
        Some(*msg)
    }
}

#[repr(C)]
struct Message<T: Send + Sync> {
    hash: u64,
    sz: usize,
    msg: T,
}

/// Creates a sender struct
#[derive(Debug, Clone, Copy)]
pub struct Sender<T: Send + Sync> {
    data: marker::PhantomData<T>,
    hash: u64,
    sz: usize,
}

impl<T: Send + Sync> Sender<T> {
    /// Sends a message
    pub fn send(&self, val: T) {
        let msg = Message {
            hash: self.hash,
            sz: self.sz,
            msg: val,
        };
        unsafe { awake_msg(msg) }
    }
}

/// Creates a receiver struct
#[derive(Debug, Clone, Copy)]
pub struct Receiver<T: Send + Sync> {
    data: marker::PhantomData<T>,
    hash: u64,
    sz: usize,
}

impl<T: Send + Sync> Receiver<T> {
    /// Receives a message
    pub fn recv(&self) -> Option<T> {
        let data: Option<Message<T>> = unsafe { thread_msg() };
        data.and_then(|data| {
            if data.sz == self.sz && data.hash == self.hash {
                Some(data.msg)
            } else {
                None
            }
        })
    }
}

/// Creates a channel returning a Sender and Receiver structs (mpsc)
// The implementation could really use generic statics
pub fn channel<T: Send + Sync>() -> (Sender<T>, Receiver<T>) {
    let msg_sz = mem::size_of::<T>();
    let type_name = any::type_name::<T>();
    let mut hasher = DefaultHasher::new();
    type_name.hash(&mut hasher);
    let type_hash = hasher.finish();

    let s = Sender {
        data: marker::PhantomData,
        hash: type_hash,
        sz: msg_sz,
    };
    let r = Receiver {
        data: marker::PhantomData,
        hash: type_hash,
        sz: msg_sz,
    };
    (s, r)
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

/// Adds a one-shot timeout callback. The timeout duration `tm` is indicated in seconds
/// Example:
/// ```rust,no_run
/// use fltk::{prelude::*, *};
/// fn callback() {
///     println!("TICK");
///     app::repeat_timeout(1.0, callback);
/// }
/// fn main() {
///     let app = app::App::default();
///     let mut wind = window::Window::new(100, 100, 400, 300, "");
///     wind.show();
///     app::add_timeout(1.0, callback);
///     app.run().unwrap();
/// }
/// ```
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

/// Repeats a timeout callback from the expiration of the previous timeout.
/// You may only call this method inside a timeout callback.
/// The timeout duration `tm` is indicated in seconds
/// Example:
/// ```rust,no_run
/// use fltk::{prelude::*, *};
/// fn callback() {
///     println!("TICK");
///     app::repeat_timeout(1.0, callback);
/// }
/// fn main() {
///     let app = app::App::default();
///     let mut wind = window::Window::new(100, 100, 400, 300, "");
///     wind.show();
///     app::add_timeout(1.0, callback);
///     app.run().unwrap();
/// }
/// ```
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

/// Returns whether a quit signal was sent
pub fn should_program_quit() -> bool {
    unsafe { fl::Fl_should_program_quit() != 0 }
}

/// Determines whether a program should quit
pub fn program_should_quit(flag: bool) {
    unsafe { fl::Fl_program_should_quit(flag as i32) }
}

/// Returns whether an event occured within a widget
pub fn event_inside_widget<Wid: WidgetExt>(wid: &Wid) -> bool {
    assert!(!wid.was_deleted());
    let x = wid.x();
    let y = wid.y();
    let w = wid.width();
    let h = wid.height();
    unsafe { fl::Fl_event_inside(x, y, w, h) != 0 }
}

/// Returns whether an event occured within a region
pub fn event_inside(x: i32, y: i32, w: i32, h: i32) -> bool {
    unsafe { fl::Fl_event_inside(x, y, w, h) != 0 }
}

/// Gets the widget that is below the mouse cursor.
/// This returns an Option<impl WidgetExt> which can be specified in the function call
/// ```rust,no_run
/// use fltk::app;
/// use fltk::widget;
/// let w = app::belowmouse::<widget::Widget>(); // or by specifying a more concrete type
/// ```
pub fn belowmouse<Wid: WidgetExt>() -> Option<impl WidgetExt> {
    unsafe {
        let x = fl::Fl_belowmouse() as *mut fltk_sys::fl::Fl_Widget;
        if x.is_null() {
            None
        } else {
            Some(crate::widget::Widget::from_widget_ptr(
                x as *mut fltk_sys::widget::Fl_Widget,
            ))
        }
    }
}

/// Deletes widgets and their children.
pub fn delete_widget<Wid: WidgetBase>(wid: Wid) {
    assert!(!wid.was_deleted());
    WidgetBase::delete(wid)
}

/// Registers all images supported by `SharedImage`
pub fn register_images() {
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

/// Redraws everything
pub fn redraw() {
    unsafe { fl::Fl_redraw() }
}

/// Returns whether the event is a shift press
pub fn is_event_shift() -> bool {
    unsafe { fl::Fl_event_shift() != 0 }
}

/// Returns whether the event is a control key press
pub fn is_event_ctrl() -> bool {
    unsafe { fl::Fl_event_ctrl() != 0 }
}

/// Returns whether the event is a command key press
pub fn is_event_command() -> bool {
    unsafe { fl::Fl_event_command() != 0 }
}

/// Returns whether the event is a alt key press
pub fn is_event_alt() -> bool {
    unsafe { fl::Fl_event_alt() != 0 }
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

/// Makes FLTK use its own colormap. This may make FLTK display better
pub fn own_colormap() {
    unsafe { fl::Fl_own_colormap() }
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

/// Gets FLTK version
pub fn version() -> f64 {
    unsafe { fl::Fl_version() }
}

/// Gets FLTK API version
pub fn api_version() -> i32 {
    unsafe { fl::Fl_api_version() }
}

/// Gets FLTK ABI version
pub fn abi_version() -> i32 {
    unsafe { fl::Fl_abi_version() }
}

/// Gets FLTK crate version
pub fn crate_version() -> &'static str {
    env!("CARGO_PKG_VERSION")
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

/// Initiate dnd action
pub fn dnd() {
    unsafe {
        fl::Fl_dnd();
    }
}

/// Load a font from a file
fn load_font(path: &str) -> Result<String, FltkError> {
    unsafe {
        let path = CString::new(path)?;
        if let Some(load_font) = *LOADED_FONT {
            unload_font(load_font)?;
        }
        let ptr = fl::Fl_load_font(path.as_ptr());
        if ptr.is_null() {
            Err::<String, FltkError>(FltkError::Internal(FltkErrorKind::FailedOperation))
        } else {
            let name = CStr::from_ptr(ptr as *mut _).to_string_lossy().to_string();
            let mut f = FONTS.lock().unwrap();
            if f.len() < 17 {
                f.push(name.clone());
            } else {
                f[16] = name.clone();
            }
            Ok(name)
        }
    }
}

/// Unload a loaded font
fn unload_font(path: &str) -> Result<(), FltkError> {
    unsafe {
        let check = path::Path::new(path);
        if !check.exists() {
            return Err::<(), FltkError>(FltkError::Internal(FltkErrorKind::ResourceNotFound));
        }
        let path = CString::new(path)?;
        fl::Fl_unload_font(path.as_ptr());
        Ok(())
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

/// Set the foreground color
pub fn foreground(r: u8, g: u8, b: u8) {
    unsafe { fl::Fl_foreground(r, g, b) }
}

/// Set the background color
pub fn background(r: u8, g: u8, b: u8) {
    unsafe { fl::Fl_background(r, g, b) }
}

/// Set the background color for input and text widgets
pub fn background2(r: u8, g: u8, b: u8) {
    unsafe { fl::Fl_background2(r, g, b) }
}

/// Sets the app's default selection color
pub fn set_selection_color(r: u8, g: u8, b: u8) {
    unsafe { fl::Fl_selection_color(r, g, b) }
}
/// Sets the app's default selection color
pub fn set_inactive_color(r: u8, g: u8, b: u8) {
    unsafe { fl::Fl_inactive_color(r, g, b) }
}

/// Gets the system colors
pub fn get_system_colors() {
    unsafe { fl::Fl_get_system_colors() }
}

/// Send a signal to a window.
/// Integral values from 0 to 30 are reserved.
/// Returns Ok(true) if the event was handled.
/// Returns Ok(false) if the event was not handled.
/// Returns Err on error or in use of one of the reserved values.
/// ```rust,no_run
/// use fltk::{prelude::*, *};
/// const CHANGE_FRAME: i32 = 100;
/// let mut wind = window::Window::default();
/// let mut but = button::Button::default();
/// let mut frame = frame::Frame::default();
/// but.set_callback(move |_| {
///     let _ = app::handle(CHANGE_FRAME, &wind).unwrap();
/// });
/// frame.handle(move |f, ev| {
///     if ev as i32 == CHANGE_FRAME {
///         f.set_label("Hello world");
///         true
///     } else {
///         false
///     }
/// });
/// ```
/// # Errors
/// Returns Err on error or in use of one of the reserved values.
pub fn handle<I: Into<i32> + Copy + PartialEq + PartialOrd, W: WindowExt>(
    msg: I,
    w: &W,
) -> Result<bool, FltkError> {
    let val = msg.into();
    if val >= 0 && val <= 30 {
        Err(FltkError::Internal(FltkErrorKind::FailedOperation))
    } else {
        let ret = unsafe { fl::Fl_handle(val, w.as_widget_ptr() as _) != 0 };
        Ok(ret)
    }
}

/// Send a signal to a window.
/// Integral values from 0 to 30 are reserved.
/// Returns Ok(true) if the event was handled.
/// Returns Ok(false) if the event was not handled.
/// ```rust,no_run
/// use fltk::{prelude::*, *};
/// const CHANGE_FRAME: i32 = 100;
/// let mut wind = window::Window::default();
/// let mut but = button::Button::default();
/// let mut frame = frame::Frame::default();
/// but.set_callback(move |_| {
///     let _ = app::handle_main(CHANGE_FRAME).unwrap();
/// });
/// frame.handle(move |f, ev| {
///     if ev as i32 == CHANGE_FRAME {
///         f.set_label("Hello world");
///         true
///     } else {
///         false
///     }
/// });
/// ```
/// # Errors
/// Returns Err on error or in use of one of the reserved values.
pub fn handle_main<I: Into<i32> + Copy + PartialEq + PartialOrd>(
    msg: I,
) -> Result<bool, FltkError> {
    let val = msg.into();
    if val >= 0 && val <= 30 {
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

/// Flush the main window
pub fn flush() {
    unsafe { fl::Fl_flush() }
}

/// Set the screen scale
pub fn set_screen_scale(n: i32, factor: f32) {
    unsafe { fl::Fl_set_screen_scale(n as i32, factor) }
}

/// Get the screen scale
pub fn screen_scale(n: i32) -> f32 {
    unsafe { fl::Fl_screen_scale(n as i32) }
}

/// Return whether scaling the screen is supported
pub fn screen_scaling_supported() -> bool {
    unsafe { fl::Fl_screen_scaling_supported() != 0 }
}

/// Get the screen count
pub fn screen_count() -> i32 {
    unsafe { fl::Fl_screen_count() as i32 }
}

/// Get the screen number based on its coordinates
pub fn screen_num(x: i32, y: i32) -> i32 {
    unsafe {
        fl::Fl_screen_num(x, y)
    }
}

/// Get a screen's dpi resolution
/// # Returns
/// (vertical, horizontal) resolutions
pub fn screen_dpi(screen_num: i32) -> (f32, f32) {
    let mut h: f32 = 0.;
    let mut v: f32 = 0.;
    unsafe {
        fl::Fl_screen_dpi(&mut h, &mut v, screen_num);
    }
    (h, v)
}

/// Get a screen's xywh
pub fn screen_xywh(screen_num: i32) -> (i32, i32, i32, i32) {
    let mut x = 0;
    let mut y = 0;
    let mut w = 0;
    let mut h = 0;
    unsafe {
        fl::Fl_screen_xywh(&mut x, &mut y, &mut w, &mut h, screen_num);
    }
    (x, y, w, h)
}

/// Get a screen's working area
pub fn screen_work_area(screen_num: i32) -> (i32, i32, i32, i32) {
    let mut x = 0;
    let mut y = 0;
    let mut w = 0;
    let mut h = 0;
    unsafe {
        fl::Fl_screen_work_area(&mut x, &mut y, &mut w, &mut h, screen_num);
    }
    (x, y, w, h)
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
