use crate::app::widget::first_window;
use crate::enums::{Event, Key, Shortcut};
use crate::prelude::*;
use crate::utils::FlString;
use fltk_sys::fl;
use std::{
    cmp,
    ffi::{CStr, CString},
    mem,
    os::raw,
    panic,
};

/// Alias Window ptr
pub type WindowPtr = *mut fltk_sys::window::Fl_Window;

/// Returns the latest captured event
pub fn event() -> Event {
    unsafe { mem::transmute(fl::Fl_event()) }
}

/// Returns the pressed key
pub fn event_key() -> Key {
    unsafe { mem::transmute(fl::Fl_event_key()) }
}

/// Returns the original key
pub fn event_original_key() -> Key {
    unsafe { mem::transmute(fl::Fl_event_original_key()) }
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

/// Returns false for a single click and true for more
pub fn event_clicks() -> bool {
    unsafe { fl::Fl_event_clicks() != 0 }
}

/// Returns the number of clicks - 1
pub fn event_clicks_num() -> i32 {
    unsafe { fl::Fl_event_clicks() }
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
/// Returns [`MouseWheel::None`], `Right` or `Left`
pub fn event_dx() -> MouseWheel {
    match 0.cmp(unsafe { &fl::Fl_event_dx() }) {
        cmp::Ordering::Greater => MouseWheel::Right,
        cmp::Ordering::Equal => MouseWheel::None,
        cmp::Ordering::Less => MouseWheel::Left,
    }
}

/// Returns the current horizontal mouse scrolling associated with the Mousewheel event.
/// Returns [`MouseWheel::None`], `Up` or `Down`.
/// Doesn't indicate scrolling direction which depends on system preferences
pub fn event_dy() -> MouseWheel {
    match 0.cmp(unsafe { &fl::Fl_event_dy() }) {
        cmp::Ordering::Greater => MouseWheel::Down,
        cmp::Ordering::Equal => MouseWheel::None,
        cmp::Ordering::Less => MouseWheel::Up,
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

/// Returns whether an event occurred within a widget
pub fn event_inside_widget<Wid: WidgetExt>(wid: &Wid) -> bool {
    assert!(!wid.was_deleted());
    let x = wid.x();
    let y = wid.y();
    let w = wid.width();
    let h = wid.height();
    unsafe { fl::Fl_event_inside(x, y, w, h) != 0 }
}

/// Returns whether an event occurred within a region
pub fn event_inside(x: i32, y: i32, w: i32, h: i32) -> bool {
    unsafe { fl::Fl_event_inside(x, y, w, h) != 0 }
}

/**
    Gets the widget that is below the mouse cursor.
    This returns an Option<impl WidgetExt> which can be specified in the function call
    ```rust,no_run
    use fltk::app;
    use fltk::widget;
    let w = app::belowmouse::<widget::Widget>(); // or by specifying a more concrete type
    ```
*/
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

/// Initiate dnd action
pub fn dnd() {
    unsafe {
        fl::Fl_dnd();
    }
}

/// Get the clipboard content if it's an image
pub fn event_clipboard_image() -> Option<crate::image::RgbImage> {
    unsafe {
        let image = fl::Fl_event_clipboard();
        if image.is_null() {
            None
        } else {
            use std::sync::atomic::AtomicUsize;
            Some(crate::image::RgbImage {
                inner: image as _,
                refcount: AtomicUsize::new(1),
            })
        }
    }
}

/// The event clipboard type
#[derive(Debug, Clone)]
pub enum ClipboardEvent {
    /// Text paste event
    Text(String),
    /// image paste event
    Image(Option<crate::image::RgbImage>),
}

/// Get the clipboard content type
pub fn event_clipboard() -> Option<ClipboardEvent> {
    unsafe {
        let txt = fl::Fl_event_clipboard_type();
        let txt = CStr::from_ptr(txt).to_string_lossy().to_string();
        if txt == "text/plain" {
            Some(ClipboardEvent::Text(event_text()))
        } else if txt == "image" {
            Some(ClipboardEvent::Image(event_clipboard_image()))
        } else {
            None
        }
    }
}

#[allow(clippy::missing_safety_doc)]
/**
    Send a signal to a window pointer from event_dispatch.
    Returns true if the event was handled.
    Returns false if the event was not handled or ignored.
    ```rust,no_run
    use fltk::{prelude::*, *};
    const CHANGE_FRAME: i32 = 100;
    let mut wind = window::Window::default();
    let mut but = button::Button::default();
    let mut frame = frame::Frame::default();
    wind.end();
    wind.show();
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
    unsafe {
        app::event_dispatch(|ev, winptr| {
            if ev == CHANGE_FRAME.into() {
                false // ignore CHANGE_FRAME event
            } else {
                app::handle_raw(ev, winptr)
            }
        });
    }
    ```
    # Safety
    The window pointer must be valid
*/
pub unsafe fn handle_raw(event: Event, w: WindowPtr) -> bool {
    fl::Fl_handle_(event.bits(), w as _) != 0
}

#[allow(clippy::missing_safety_doc)]
/**
    The event dispatch function is called after native events are converted to
    FLTK events, but before they are handled by FLTK. If the dispatch function
    handler is set, it is up to the dispatch function to call
    [`app::handle_raw(Event, WindowPtr)`](crate::app::handle_raw) or to ignore the event.

    The dispatch function itself must return false if it ignored the event,
    or true if it used the event. If you call [`app::handle_raw()`](crate::app::handle_raw),
    then this will return the correct value.
    # Safety
    The window pointer must not be invalidated
*/
pub unsafe fn event_dispatch(f: fn(Event, WindowPtr) -> bool) {
    fl::Fl_event_dispatch(mem::transmute(f));
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

/// Text editing widget should call this for each FL_KEYBOARD event.
pub fn compose() -> Option<i32> {
    unsafe {
        let mut del = 0;
        if fl::Fl_compose(&mut del) != 0 {
            Some(del)
        } else {
            None
        }
    }
}

/// Reset the length of bytes of app::compose()
pub fn compose_reset() {
    unsafe {
        fl::Fl_compose_reset();
    }
}

/// Return the length of bytes written in app::compose()
pub fn compose_state() -> i32 {
    unsafe { fl::Fl_compose_state() }
}

/// Copy text to the clipboard
pub fn copy(stuff: &str) {
    unsafe {
        fl::Fl_open_display();
        let len = stuff.len();
        let stuff = CString::safe_new(stuff);
        fl::Fl_copy(stuff.as_ptr() as _, len as _, 1);
    }
}

/// Copy text to the selection buffer
pub fn copy2(stuff: &str) {
    unsafe {
        fl::Fl_open_display();
        let len = stuff.len();
        let stuff = CString::safe_new(stuff);
        fl::Fl_copy(stuff.as_ptr() as _, len as _, 0);
    }
}

/// Types of Clipboard contents
#[derive(Debug, Clone, Copy)]
pub enum ClipboardContent {
    /// Textual content
    Text,
    /// Image content
    Image,
}

/// Check the contents of the clipboard
pub fn clipboard_contains(content: ClipboardContent) -> bool {
    use ClipboardContent::*;
    let txt = match content {
        Text => "text/plain",
        Image => "image",
    };
    let txt = CString::new(txt).unwrap();
    unsafe { fl::Fl_clipboard_contains(txt.as_ptr()) != 0 }
}

/// Pastes content from the clipboard
pub fn paste<T>(widget: &T)
where
    T: WidgetExt,
{
    assert!(!widget.was_deleted());
    if clipboard_contains(ClipboardContent::Text) {
        paste_text(widget)
    } else if clipboard_contains(ClipboardContent::Image) {
        paste_image(widget)
    } else {
        // Do nothing!
    }
}

/// Pastes textual content from the clipboard
pub fn paste_text<T>(widget: &T)
where
    T: WidgetExt,
{
    assert!(!widget.was_deleted());
    unsafe {
        fl::Fl_paste_text(widget.as_widget_ptr() as *mut fltk_sys::fl::Fl_Widget, 1);
    }
}

/// Pastes textual content from the selection buffer
pub fn paste_text2<T>(widget: &T)
where
    T: WidgetExt,
{
    assert!(!widget.was_deleted());
    unsafe {
        fl::Fl_paste_text(widget.as_widget_ptr() as *mut fltk_sys::fl::Fl_Widget, 0);
    }
}

/// Pastes image content from the clipboard
pub fn paste_image<T>(widget: &T)
where
    T: WidgetExt,
{
    assert!(!widget.was_deleted());
    unsafe {
        fl::Fl_paste_image(widget.as_widget_ptr() as *mut fltk_sys::fl::Fl_Widget, 1);
    }
}

/// Pastes image content from the selection buffer
pub fn paste_image2<T>(widget: &T)
where
    T: WidgetExt,
{
    assert!(!widget.was_deleted());
    unsafe {
        fl::Fl_paste_image(widget.as_widget_ptr() as *mut fltk_sys::fl::Fl_Widget, 0);
    }
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
pub fn handle<I: Into<Event> + Copy + PartialEq + PartialOrd, W: WindowExt>(
    msg: I,
    w: &W,
) -> Result<bool, FltkError> {
    let val = msg.into();
    let ret = unsafe { fl::Fl_handle(val.bits(), w.as_widget_ptr() as _) != 0 };
    Ok(ret)
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
pub fn handle_main<I: Into<Event> + Copy + PartialEq + PartialOrd>(
    msg: I,
) -> Result<bool, FltkError> {
    let val = msg.into();
    first_window().map_or(
        Err(FltkError::Internal(FltkErrorKind::FailedOperation)),
        |win| {
            let ret = unsafe { fl::Fl_handle(val.bits(), win.as_widget_ptr() as _) != 0 };
            Ok(ret)
        },
    )
}

#[cfg(target_os = "macos")]
/// Register a function called for each file dropped onto an application icon.
/// This function is effective only on the Mac OS X platform. 
/// cb will be called with a single Unix-style file name and path. 
/// If multiple files were dropped, cb will be called multiple times.
/// ```rust,no_run
/// use fltk::{app, dialog};
/// app::raw_open_callback(Some(|s| {
///    let name = unsafe { std::ffi::CStr::from_ptr(s).to_string_lossy().to_string() };
///    dialog::message_default(&format!("You dropped {}", name));
/// }));
/// ```
pub fn raw_open_callback(cb: Option<fn(*const raw::c_char)>) {
    unsafe {
        if let Some(cb) = cb {
            fl::Fl_open_callback(Some(mem::transmute(cb)))
        } else {
            fl::Fl_open_callback(None)
        }
    }
}
