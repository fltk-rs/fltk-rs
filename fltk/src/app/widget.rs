use crate::prelude::*;
use crate::window::Window;
use fltk_sys::fl;
use std::{mem, os::raw, panic, ptr};

/// Alias Widget ptr
pub type WidgetPtr = *mut fltk_sys::widget::Fl_Widget;

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

#[deprecated = "use app::set_grab(None) instead"]
/// Unset the currently grabbed window
pub fn release() {
    unsafe { fl::Fl_release() }
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
        let mut _old_data = None;
        if widget.is_derived() {
            _old_data = widget.user_data();
        }
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

/// Returns the last modal window of the application
pub fn modal() -> Option<impl WindowExt> {
    unsafe {
        let x = fl::Fl_modal();
        if x.is_null() {
            None
        } else {
            let x = Window::from_widget_ptr(x as *mut fltk_sys::widget::Fl_Widget);
            Some(x)
        }
    }
}

/// Deletes widgets and their children.
pub fn delete_widget<Wid: WidgetBase>(wid: Wid) {
    assert!(!wid.was_deleted());
    WidgetBase::delete(wid)
}

/// Sets the damage to true or false, eliciting a redraw by the application
pub fn set_damage(flag: bool) {
    unsafe { fl::Fl_set_damage(flag as i32) }
}

/// Returns whether any of the widgets were damaged
pub fn damage() -> bool {
    unsafe { fl::Fl_damage() != 0 }
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

/// Sets the widget which has focus at the start of the application
pub fn set_focus<W: WidgetExt>(wid: &W) {
    unsafe { fl::Fl_set_focus(wid.as_widget_ptr() as *mut raw::c_void) }
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
