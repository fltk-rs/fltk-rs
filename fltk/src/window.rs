#![allow(unused_imports)]

//! Window widgets
//!
//! **Multithreaded** applications should always create/show/open/close windows
//! from the main thread (This might or might not work on your specific target,
//! due to fltk calling the underlying platform's window code. If you want
//! portability, avoid it.) If you need to trigger showing a windows from
//! another thread, use [`messages`](crate::app::channel) to notify the main
//! thread that the window needs showing. An example of this can be found in the
//! [`threads_windows`](crate::examples::threads_windows) example. An alternative to that is
//! [`awake_callback`](crate::app::awake_callback)

use crate::app::screen_size;
use crate::enums::{
    Align, When, Color, Cursor, Damage, Event, Font, FrameType, LabelType, Mode,
};
use crate::image::Image;
use crate::prelude::*;
use crate::utils::FlString;
use crate::widget::Widget;
use fltk_sys::window::*;
#[cfg(feature = "rwh06")]
use rwh06::*;
use std::{
    ffi::{CStr, CString},
    mem,
    ops::{Deref, DerefMut},
    os::raw,
};

// Opaque raw window handle on 32-bit linux running on a Raspberry Pi
#[cfg(all(
    not(any(
        target_os = "windows",
        target_os = "macos",
        target_os = "ios",
        target_os = "android",
        target_os = "emscripten",
    )),
    any(
        target_arch = "arm",
        target_arch = "mips",
        target_arch = "powerpc",
        target_arch = "sparc",
        target_arch = "wasm32",
        target_arch = "x86",
    )
))]
type RawXlibHandle = u32;

/// Opaque raw window handle (`*mut c_void` to `HWND` on Windows and `NSWindow` on macOS),
/// `XID` (`u64`) raw window handle for X11, and `wl_suface *` for wayland
#[cfg(all(
    not(any(
        target_os = "windows",
        target_os = "macos",
        target_os = "ios",
        target_os = "android",
        target_os = "emscripten"
    )),
    any(
        target_arch = "aarch64",
        target_arch = "loongarch64",
        target_arch = "mips64",
        target_arch = "powerpc64",
        target_arch = "s390x",
        target_arch = "sparc64",
        target_arch = "x86_64",
    )
))]
type RawXlibHandle = u64;

/// Opaque raw window handle (`*mut c_void` to `HWND` on Windows and `NSWindow` on macOS),
/// `XID` (`u64`) raw window handle for X11
#[cfg(any(
    target_os = "windows",
    target_os = "macos",
    target_os = "ios",
    target_os = "android",
    target_os = "emscripten",
))]
pub type RawHandle = *mut raw::c_void;

/// Opaque raw window handle (`*mut c_void` to `HWND` on Windows and `NSWindow` on macOS),
/// `XID` (`u64`) raw window handle for X11
#[cfg(all(
    not(any(
        target_os = "windows",
        target_os = "macos",
        target_os = "ios",
        target_os = "android",
        target_os = "emscripten"
    )),
    feature = "no-wayland"
))]
pub type RawHandle = RawXlibHandle;

/// Opaque raw window handle (`*mut c_void` to `HWND` on Windows and `NSWindow` on macOS),
/// `XID` (`u64`) raw window handle for X11
#[cfg(all(
    not(any(
        target_os = "windows",
        target_os = "macos",
        target_os = "ios",
        target_os = "android",
        target_os = "emscripten"
    )),
    not(feature = "no-wayland")
))]
pub type RawHandle = *mut std::os::raw::c_void;


/// Creates a window widget
pub type Window = DoubleWindow;

/// Defines the window type
#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum WindowType {
    /// Single window
    Single = 240,
    /// Double window
    Double = 241,
}

crate::macros::widget::impl_widget_type!(WindowType);

macro_rules! impl_ppu {
    ($name:ident) => {
        impl $name {
            /// Returns the pixels per unit/point
            pub fn pixels_per_unit(&self) -> f32 {
                #[allow(unused_mut)]
                let mut factor = 1.0;
                #[cfg(target_os = "macos")]
                {
                    unsafe extern "C" {
                        pub fn cfltk_getScalingFactor(handle: *mut raw::c_void) -> f64;
                    }
                    let mac_version = unsafe { fltk_sys::fl::Fl_mac_os_version() };
                    if mac_version >= 100700 {
                        factor = unsafe { cfltk_getScalingFactor(self.raw_handle()) };
                    }
                }
                let s = crate::app::screen_scale(self.screen_num());
                s * factor as f32
            }

            /// Gets the window's width in pixels
            pub fn pixel_w(&self) -> i32 {
                (self.pixels_per_unit() * self.w() as f32) as i32
            }

            /// Gets the window's height in pixels
            pub fn pixel_h(&self) -> i32 {
                (self.pixels_per_unit() * self.h() as f32) as i32
            }
        }
    };
}

macro_rules! impl_top_win {
    ($name:ident) => {
        impl $name {
            /// Find an `Fl_Window` through a raw handle. The window must have been instantiated by the app.
            /// `void *` to: (Windows: `HWND`, X11: `Xid` (`u64`), macOS: `NSWindow`)
            /// # Safety
            /// The data must be valid and is OS-dependent.
            pub unsafe fn find_by_handle(handle: RawHandle) -> Option<impl WindowExt> {
                unsafe {
                    let ptr = Fl_Window_find_by_handle(handle as *const raw::c_void as *mut _);
                    if ptr.is_null() {
                        None
                    } else {
                        Some(Window::from_widget_ptr(
                            ptr as *mut fltk_sys::widget::Fl_Widget,
                        ))
                    }
                }
            }

            /// Use FLTK specific arguments for the application:
            /// More info [here](https://www.fltk.org/doc-1.3/classFl.html#a1576b8c9ca3e900daaa5c36ca0e7ae48).
            /// The options are:
            /// - `-bg2 color`
            /// - `-bg color`
            /// - `-di[splay] host:n.n`
            /// - `-dn[d]`
            /// - `-fg color`
            /// - `-g[eometry] WxH+X+Y`
            /// - `-i[conic]`
            /// - `-k[bd]`
            /// - `-na[me] classname`
            /// - `-nod[nd]`
            /// - `-nok[bd]`
            /// - `-not[ooltips]`
            /// - `-s[cheme] scheme`
            /// - `-ti[tle] windowtitle`
            /// - `-to[oltips]`
            pub fn show_with_env_args(&mut self) {
                unsafe {
                    let args: Vec<String> = std::env::args().collect();
                    let len = args.len() as i32;
                    let mut v: Vec<*mut raw::c_char> = vec![];
                    for arg in args {
                        let c = CString::safe_new(arg.as_str());
                        v.push(c.into_raw() as *mut raw::c_char);
                    }
                    let mut v = mem::ManuallyDrop::new(v);
                    Fl_Window_show_with_args(
                        self.inner.widget() as *mut Fl_Window,
                        len,
                        v.as_mut_ptr(),
                    )
                }
            }

            /// Use FLTK specific arguments for the application:
            /// More info [here](https://www.fltk.org/doc-1.3/classFl.html#a1576b8c9ca3e900daaa5c36ca0e7ae48).
            /// The options are:
            /// - `-bg2 color`
            /// - `-bg color`
            /// - `-di[splay] host:n.n`
            /// - `-dn[d]`
            /// - `-fg color`
            /// - `-g[eometry] WxH+X+Y`
            /// - `-i[conic]`
            /// - `-k[bd]`
            /// - `-na[me] classname`
            /// - `-nod[nd]`
            /// - `-nok[bd]`
            /// - `-not[ooltips]`
            /// - `-s[cheme] scheme`
            /// - `-ti[tle] windowtitle`
            /// - `-to[oltips]`
            pub fn show_with_args(&mut self, args: &[&str]) {
                unsafe {
                    let mut temp = vec![""];
                    temp.extend(args);
                    let len = temp.len() as i32;
                    let mut v: Vec<*mut raw::c_char> = vec![];
                    for arg in temp {
                        let c = CString::safe_new(arg);
                        v.push(c.into_raw() as *mut raw::c_char);
                    }
                    let mut v = mem::ManuallyDrop::new(v);
                    Fl_Window_show_with_args(
                        self.inner.widget() as *mut Fl_Window,
                        len,
                        v.as_mut_ptr(),
                    )
                }
            }

            /// Set the window to be on top of other windows.
            /// Must only be called after the window has been shown.
            pub fn set_on_top(&mut self) {
                assert!(self.raw_handle() as isize != 0);
                #[cfg(target_os = "macos")]
                {
                    unsafe extern "C" {
                        pub fn cfltk_setOnTop(handle: *mut raw::c_void);
                    }
                    unsafe {
                        cfltk_setOnTop(self.raw_handle());
                    }
                }
                #[cfg(target_os = "windows")]
                {
                    unsafe extern "system" {
                        fn SetWindowPos(
                            hwnd: *mut raw::c_void,
                            insert_after: isize,
                            x: i32,
                            y: i32,
                            cx: i32,
                            cy: i32,
                            flags: u32,
                        ) -> bool;
                    }
                    const TOP_MOST: isize = -1;
                    const SWP_NOSIZE: u32 = 1;
                    const SWP_NOMOVE: u32 = 2;
                    unsafe {
                        SetWindowPos(
                            self.raw_handle(),
                            TOP_MOST,
                            0,
                            0,
                            0,
                            0,
                            SWP_NOSIZE | SWP_NOMOVE,
                        );
                    }
                }
                #[cfg(not(any(
                    target_os = "macos",
                    target_os = "android",
                    target_os = "windows",
                    target_os = "emscripten"
                )))]
                {
                    unsafe extern "C" {
                        pub fn cfltk_setOnTop(handle: RawXlibHandle);
                    }
                    if !crate::app::using_wayland() {
                        unsafe {
                            cfltk_setOnTop(self.raw_handle() as RawXlibHandle);
                        }
                    }
                }
            }

            /// Maximize the window
            pub fn maximize(&mut self) {
                unsafe { Fl_Window_maximize(self.inner.widget() as _) }
            }

            /// Unmaximize the window
            pub fn un_maximize(&mut self) {
                unsafe { Fl_Window_un_maximize(self.inner.widget() as _) }
            }

            /// Checks whether the window is maximized
            pub fn maximize_active(&self) -> bool {
                unsafe { Fl_Window_maximize_active(self.inner.widget() as _) != 0 }
            }

            /// Get the default XA_WM_CLASS property for all windows of your application
            pub fn default_xclass() -> Option<String> {
                unsafe {
                    let ptr = Fl_Single_Window_default_xclass();
                    if ptr.is_null() {
                        None
                    } else {
                        Some(CStr::from_ptr(ptr).to_string_lossy().to_string())
                    }
                }
            }

            /// Set the default XA_WM_CLASS property for all windows of your application.
            /// This should be called before showing with window
            pub fn set_default_xclass(s: &str) {
                let s = CString::safe_new(s);
                unsafe { Fl_Single_Window_set_default_xclass(s.as_ptr()) }
            }
        }
    };
}

/// Creates a single (buffered) window widget
#[derive(Debug)]
pub struct SingleWindow {
    inner: crate::widget::WidgetTracker,
    is_derived: bool,
}

crate::macros::widget::impl_widget_ext!(SingleWindow, Fl_Single_Window);
crate::macros::widget::impl_widget_base!(SingleWindow, Fl_Single_Window);
crate::macros::group::impl_group_ext!(SingleWindow, Fl_Single_Window);
crate::macros::window::impl_window_ext!(SingleWindow, Fl_Single_Window);

impl SingleWindow {
    /// Creates a new window, with title as its window title if the window is decorated
    pub fn new<'a, T: Into<Option<&'a str>>>(x: i32, y: i32, w: i32, h: i32, title: T) -> Self {
        let temp = if let Some(title) = title.into() {
            CString::safe_new(title).into_raw()
        } else {
            std::ptr::null_mut()
        };
        unsafe {
            let widget_ptr = Fl_Single_Window_new(x, y, w, h, temp);
            assert!(!widget_ptr.is_null());
            assert!(crate::app::is_ui_thread());
            let tracker = crate::widget::WidgetTracker::new(widget_ptr as _);
            unsafe extern "C" fn shim(wid: *mut Fl_Widget, _data: *mut std::os::raw::c_void) {
                unsafe {
                    let user_data = Fl_Single_Window_user_data(wid as _);
                    let draw_data = Fl_Single_Window_draw_data(wid as _);
                    let handle_data = Fl_Single_Window_handle_data(wid as _);
                    crate::app::add_timeout(0., move |h| {
                        if !user_data.is_null() {
                            let _x = Box::from_raw(user_data as *mut Box<dyn FnMut()>);
                        }
                        if !draw_data.is_null() {
                            let _x = Box::from_raw(draw_data as *mut Box<dyn FnMut()>);
                        }
                        if !handle_data.is_null() {
                            let _x = Box::from_raw(handle_data as *mut Box<dyn FnMut()>);
                        }
                        crate::app::remove_timeout(h);
                    });
                }
            }
            Fl_Single_Window_set_deletion_callback(widget_ptr, Some(shim), std::ptr::null_mut());
            Self {
                inner: tracker,
                is_derived: true,
            }
        }
    }
}

impl Default for SingleWindow {
    fn default() -> Self {
        assert!(crate::app::is_ui_thread());
        let mut win = SingleWindow::new(0, 0, 0, 0, None);
        win.free_position();
        win
    }
}

impl_top_win!(SingleWindow);
impl_ppu!(SingleWindow);

/// Creates a double (buffered) window widget
#[derive(Debug)]
pub struct DoubleWindow {
    inner: crate::widget::WidgetTracker,
    is_derived: bool,
}

crate::macros::widget::impl_widget_ext!(DoubleWindow, Fl_Double_Window);
crate::macros::widget::impl_widget_base!(DoubleWindow, Fl_Double_Window);
crate::macros::group::impl_group_ext!(DoubleWindow, Fl_Double_Window);
crate::macros::window::impl_window_ext!(DoubleWindow, Fl_Double_Window);

impl Default for DoubleWindow {
    fn default() -> Self {
        assert!(crate::app::is_ui_thread());
        let mut win = DoubleWindow::new(0, 0, 0, 0, None);
        win.free_position();
        win
    }
}

impl_top_win!(DoubleWindow);
impl_ppu!(DoubleWindow);

impl DoubleWindow {
    /// Creates a new window, with title as its window title if the window is decorated
    pub fn new<'a, T: Into<Option<&'a str>>>(x: i32, y: i32, w: i32, h: i32, title: T) -> Self {
        let temp = if let Some(title) = title.into() {
            CString::safe_new(title).into_raw()
        } else {
            std::ptr::null_mut()
        };
        unsafe {
            let widget_ptr = Fl_Double_Window_new(x, y, w, h, temp);
            assert!(!widget_ptr.is_null());
            assert!(crate::app::is_ui_thread());
            let tracker = crate::widget::WidgetTracker::new(widget_ptr as _);
            unsafe extern "C" fn shim(wid: *mut Fl_Widget, _data: *mut std::os::raw::c_void) {
                unsafe {
                    let user_data = Fl_Double_Window_user_data(wid as _);
                    let draw_data = Fl_Double_Window_draw_data(wid as _);
                    let handle_data = Fl_Double_Window_handle_data(wid as _);
                    crate::app::add_timeout(0., move |h| {
                        if !user_data.is_null() {
                            let _x = Box::from_raw(user_data as *mut Box<dyn FnMut()>);
                        }
                        if !draw_data.is_null() {
                            let _x = Box::from_raw(draw_data as *mut Box<dyn FnMut()>);
                        }
                        if !handle_data.is_null() {
                            let _x = Box::from_raw(handle_data as *mut Box<dyn FnMut()>);
                        }
                        crate::app::remove_timeout(h);
                    });
                }
            }
            Fl_Double_Window_set_deletion_callback(widget_ptr, Some(shim), std::ptr::null_mut());
            Self {
                inner: tracker,
                is_derived: true,
            }
        }
    }
    /// Forces the window to be drawn, this window is also made current and calls draw()
    pub fn flush(&mut self) {
        unsafe { Fl_Double_Window_flush(self.inner.widget() as _) }
    }

    /// Show a window after it had been hidden. Works on Windows and X11 systems
    pub fn platform_show(&self) {
        #[allow(unused_unsafe)]
        unsafe {
            #[cfg(target_os = "windows")]
            {
                unsafe extern "system" {
                    fn ShowWindow(hwnd: *mut raw::c_void, nCmdShow: raw::c_int) -> raw::c_int;
                }
                ShowWindow(self.raw_handle(), 9);
            }
            #[cfg(target_os = "macos")]
            {
                unsafe extern "C" {
                    fn cfltk_winShow(xid: *mut raw::c_void);
                }
                cfltk_winShow(self.raw_handle());
            }
            #[cfg(not(any(
                target_os = "macos",
                target_os = "android",
                target_os = "windows",
                target_os = "emscripten"
            )))]
            {
                if !crate::app::using_wayland() {
                    unsafe extern "C" {
                        fn cfltk_platform_show(proxy: *mut raw::c_void);
                    }
                    cfltk_platform_show(self.raw_handle() as *mut raw::c_void);
                } else {
                    Fl_Double_Window_show(self.inner.widget() as _);
                }
            }
        }
    }

    /// Hide a window using the platforms hide call. Works on Windows and X11 systems
    pub fn platform_hide(&self) {
        #[allow(unused_unsafe)]
        unsafe {
            #[cfg(target_os = "windows")]
            {
                unsafe extern "system" {
                    fn ShowWindow(hwnd: *mut raw::c_void, nCmdShow: raw::c_int) -> raw::c_int;
                }
                ShowWindow(self.raw_handle(), 0);
            }
            #[cfg(target_os = "macos")]
            {
                unsafe extern "C" {
                    fn cfltk_winHide(xid: *mut raw::c_void);
                }
                cfltk_winHide(self.raw_handle());
            }
            #[cfg(not(any(
                target_os = "macos",
                target_os = "android",
                target_os = "windows",
                target_os = "emscripten"
            )))]
            {
                if !crate::app::using_wayland() {
                    unsafe extern "C" {
                        fn cfltk_platform_hide(proxy: *mut raw::c_void);
                    }
                    cfltk_platform_hide(self.raw_handle() as *mut raw::c_void);
                } else {
                    Fl_Double_Window_hide(self.inner.widget() as _);
                }
            }
        }
    }
}

/// Creates a Menu window widget
#[derive(Debug)]
pub struct MenuWindow {
    inner: crate::widget::WidgetTracker,
    is_derived: bool,
}

crate::macros::widget::impl_widget_ext!(MenuWindow, Fl_Menu_Window);
crate::macros::widget::impl_widget_base!(MenuWindow, Fl_Menu_Window);
crate::macros::group::impl_group_ext!(MenuWindow, Fl_Menu_Window);
crate::macros::window::impl_window_ext!(MenuWindow, Fl_Menu_Window);

impl Default for MenuWindow {
    fn default() -> Self {
        assert!(crate::app::is_ui_thread());
        let mut win = MenuWindow::new(0, 0, 0, 0, None);
        win.free_position();
        win
    }
}

impl MenuWindow {
    /// Creates a new window, with title as its window title if the window is decorated
    pub fn new<'a, T: Into<Option<&'a str>>>(x: i32, y: i32, w: i32, h: i32, title: T) -> Self {
        let temp = if let Some(title) = title.into() {
            CString::safe_new(title).into_raw()
        } else {
            std::ptr::null_mut()
        };
        unsafe {
            let widget_ptr = Fl_Menu_Window_new(x, y, w, h, temp);
            assert!(!widget_ptr.is_null());
            assert!(crate::app::is_ui_thread());
            let tracker = crate::widget::WidgetTracker::new(widget_ptr as _);
            unsafe extern "C" fn shim(wid: *mut Fl_Widget, _data: *mut std::os::raw::c_void) {
                unsafe {
                    let user_data = Fl_Menu_Window_user_data(wid as _);
                    let draw_data = Fl_Menu_Window_draw_data(wid as _);
                    let handle_data = Fl_Menu_Window_handle_data(wid as _);
                    crate::app::add_timeout(0., move |h| {
                        if !user_data.is_null() {
                            let _x = Box::from_raw(user_data as *mut Box<dyn FnMut()>);
                        }
                        if !draw_data.is_null() {
                            let _x = Box::from_raw(draw_data as *mut Box<dyn FnMut()>);
                        }
                        if !handle_data.is_null() {
                            let _x = Box::from_raw(handle_data as *mut Box<dyn FnMut()>);
                        }
                        crate::app::remove_timeout(h);
                    });
                }
            }
            Fl_Menu_Window_set_deletion_callback(widget_ptr, Some(shim), std::ptr::null_mut());
            Self {
                inner: tracker,
                is_derived: true,
            }
        }
    }
}

/// Creates an overlay (buffered) window widget
#[derive(Debug)]
pub struct OverlayWindow {
    inner: crate::widget::WidgetTracker,
    is_derived: bool,
}

crate::macros::widget::impl_widget_ext!(OverlayWindow, Fl_Overlay_Window);
crate::macros::widget::impl_widget_base!(OverlayWindow, Fl_Overlay_Window);
crate::macros::group::impl_group_ext!(OverlayWindow, Fl_Overlay_Window);
crate::macros::window::impl_window_ext!(OverlayWindow, Fl_Overlay_Window);

impl Default for OverlayWindow {
    fn default() -> Self {
        assert!(crate::app::is_ui_thread());
        OverlayWindow::new(0, 0, 0, 0, None)
    }
}

impl_top_win!(OverlayWindow);
impl_ppu!(OverlayWindow);

impl OverlayWindow {
    /// Creates a new window, with title as its window title if the window is decorated
    pub fn new<'a, T: Into<Option<&'a str>>>(x: i32, y: i32, w: i32, h: i32, title: T) -> Self {
        let temp = if let Some(title) = title.into() {
            CString::safe_new(title).into_raw()
        } else {
            std::ptr::null_mut()
        };
        unsafe {
            let widget_ptr = Fl_Overlay_Window_new(x, y, w, h, temp);
            assert!(!widget_ptr.is_null());
            assert!(crate::app::is_ui_thread());
            let tracker = crate::widget::WidgetTracker::new(widget_ptr as _);
            unsafe extern "C" fn shim(wid: *mut Fl_Widget, _data: *mut std::os::raw::c_void) {
                unsafe {
                    let user_data = Fl_Overlay_Window_user_data(wid as _);
                    let draw_data = Fl_Overlay_Window_draw_data(wid as _);
                    let handle_data = Fl_Overlay_Window_handle_data(wid as _);
                    crate::app::add_timeout(0., move |h| {
                        if !user_data.is_null() {
                            let _x = Box::from_raw(user_data as *mut Box<dyn FnMut()>);
                        }
                        if !draw_data.is_null() {
                            let _x = Box::from_raw(draw_data as *mut Box<dyn FnMut()>);
                        }
                        if !handle_data.is_null() {
                            let _x = Box::from_raw(handle_data as *mut Box<dyn FnMut()>);
                        }
                        crate::app::remove_timeout(h);
                    });
                }
            }
            Fl_Overlay_Window_set_deletion_callback(widget_ptr, Some(shim), std::ptr::null_mut());
            Self {
                inner: tracker,
                is_derived: true,
            }
        }
    }
    /// Forces the window to be drawn, this window is also made current and calls draw()
    pub fn flush(&mut self) {
        unsafe { Fl_Double_Window_flush(self.inner.widget() as _) }
    }

    /// Draw overlay
    pub fn draw_overlay<F: FnMut(&mut Self) + 'static>(&mut self, cb: F) {
        assert!(self.is_derived);
        unsafe {
            unsafe extern "C" fn shim(wid: *mut Fl_Widget, data: *mut raw::c_void) {
                unsafe {
                    let mut wid = OverlayWindow::from_widget_ptr(wid as *mut _);
                    wid.assume_derived();
                    let a: *mut Box<dyn FnMut(&mut OverlayWindow)> =
                        data as *mut Box<dyn FnMut(&mut OverlayWindow)>;
                    let f: &mut (dyn FnMut(&mut OverlayWindow)) = &mut **a;
                    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| f(&mut wid)));
                }
            }
            let mut _old_data = None;
            if self.is_derived {
                _old_data = self.draw_data();
            }
            let a: *mut Box<dyn FnMut(&mut Self)> = Box::into_raw(Box::new(Box::new(cb)));
            let data: *mut raw::c_void = a as *mut raw::c_void;
            let callback: custom_draw_callback = Some(shim);
            Fl_Overlay_Window_draw_overlay(self.inner.widget() as _, callback, data);
        }
    }

    /// Redraw overlay
    pub fn redraw_overlay(&self) {
        unsafe { Fl_Overlay_Window_redraw_overlay(self.inner.widget() as _) }
    }

    /// Returns whether the overlay window can do hardware backed overlay
    pub fn can_do_overlay(&self) -> bool {
        unsafe { Fl_Overlay_Window_can_do_overlay(self.inner.widget() as _) != 0 }
    }
}

/// A wrapper around a raw OpenGL context
pub type GlContext = *mut raw::c_void;

/// Creates a OpenGL Glut window widget
#[cfg(feature = "enable-glwindow")]
#[derive(Debug)]
pub struct GlutWindow {
    inner: crate::widget::WidgetTracker,
    is_derived: bool,
}

#[cfg(feature = "enable-glwindow")]
crate::macros::widget::impl_widget_ext!(GlutWindow, Fl_Glut_Window);
#[cfg(feature = "enable-glwindow")]
crate::macros::widget::impl_widget_base!(GlutWindow, Fl_Glut_Window);
#[cfg(feature = "enable-glwindow")]
crate::macros::group::impl_group_ext!(GlutWindow, Fl_Glut_Window);
#[cfg(feature = "enable-glwindow")]
crate::macros::window::impl_window_ext!(GlutWindow, Fl_Glut_Window);

#[cfg(feature = "enable-glwindow")]
impl_top_win!(GlutWindow);

#[cfg(feature = "enable-glwindow")]
impl Default for GlutWindow {
    fn default() -> GlutWindow {
        assert!(crate::app::is_ui_thread());
        let mut win = GlutWindow::new(0, 0, 0, 0, None);
        win.free_position();
        win
    }
}

#[cfg(feature = "enable-glwindow")]
impl GlutWindow {
    /// Creates a new window, with title as its window title if the window is decorated
    pub fn new<'a, T: Into<Option<&'a str>>>(x: i32, y: i32, w: i32, h: i32, title: T) -> Self {
        let temp = if let Some(title) = title.into() {
            CString::safe_new(title).into_raw()
        } else {
            std::ptr::null_mut()
        };
        unsafe {
            let widget_ptr = Fl_Glut_Window_new(x, y, w, h, temp);
            assert!(!widget_ptr.is_null());
            assert!(crate::app::is_ui_thread());
            let tracker = crate::widget::WidgetTracker::new(widget_ptr as _);
            unsafe extern "C" fn shim(wid: *mut Fl_Widget, _data: *mut std::os::raw::c_void) {
                unsafe {
                    let user_data = Fl_Glut_Window_user_data(wid as _);
                    let draw_data = Fl_Glut_Window_draw_data(wid as _);
                    let handle_data = Fl_Glut_Window_handle_data(wid as _);
                    crate::app::add_timeout(0., move |h| {
                        if !user_data.is_null() {
                            let _x = Box::from_raw(user_data as *mut Box<dyn FnMut()>);
                        }
                        if !draw_data.is_null() {
                            let _x = Box::from_raw(draw_data as *mut Box<dyn FnMut()>);
                        }
                        if !handle_data.is_null() {
                            let _x = Box::from_raw(handle_data as *mut Box<dyn FnMut()>);
                        }
                        crate::app::remove_timeout(h);
                    });
                }
            }
            Fl_Glut_Window_set_deletion_callback(widget_ptr, Some(shim), std::ptr::null_mut());
            Self {
                inner: tracker,
                is_derived: true,
            }
        }
    }

    /// Gets an opengl function address
    pub fn get_proc_address(&self, s: &str) -> *const raw::c_void {
        unsafe extern "C" {
            pub fn get_proc(name: *const raw::c_char) -> *mut raw::c_void;
        }
        let s = CString::safe_new(s);
        let ret = unsafe { get_proc(s.as_ptr() as _) };
        if !ret.is_null() {
            ret as *const _
        } else {
            unsafe {
                Fl_Glut_Window_get_proc_address(self.inner.widget() as _, s.as_ptr()) as *const _
            }
        }
    }

    /// Forces the window to be drawn, this window is also made current and calls draw()
    pub fn flush(&mut self) {
        unsafe { Fl_Glut_Window_flush(self.inner.widget() as _) }
    }

    /// Returns whether the OpeGL context is still valid
    pub fn valid(&self) -> bool {
        unsafe { Fl_Glut_Window_valid(self.inner.widget() as _) != 0 }
    }

    /// Mark the OpeGL context as still valid
    pub fn set_valid(&mut self, v: bool) {
        unsafe { Fl_Glut_Window_set_valid(self.inner.widget() as _, v as raw::c_char) }
    }

    /// Returns whether the context is valid upon creation
    pub fn context_valid(&self) -> bool {
        unsafe { Fl_Glut_Window_context_valid(self.inner.widget() as _) != 0 }
    }

    /// Mark the context as valid upon creation
    pub fn set_context_valid(&mut self, v: bool) {
        unsafe { Fl_Glut_Window_set_context_valid(self.inner.widget() as _, v as raw::c_char) }
    }

    /// Returns the GlContext
    pub fn context(&self) -> Option<GlContext> {
        unsafe {
            let ctx = Fl_Glut_Window_context(self.inner.widget() as _);
            if ctx.is_null() { None } else { Some(ctx) }
        }
    }

    #[allow(clippy::not_unsafe_ptr_arg_deref)]
    /// Sets the GlContext
    pub fn set_context(&mut self, ctx: GlContext, destroy_flag: bool) {
        assert!(!ctx.is_null());
        unsafe { Fl_Glut_Window_set_context(self.inner.widget() as _, ctx, destroy_flag as i32) }
    }

    /// Swaps the back and front buffers
    pub fn swap_buffers(&mut self) {
        unsafe { Fl_Glut_Window_swap_buffers(self.inner.widget() as _) }
    }

    /// Gets the swap interval
    pub fn swap_interval(&self) -> i32 {
        unsafe { Fl_Glut_Window_swap_interval(self.inner.widget() as _) }
    }

    /// Sets the swap interval
    pub fn set_swap_interval(&mut self, frames: i32) {
        unsafe { Fl_Glut_Window_set_swap_interval(self.inner.widget() as _, frames) }
    }

    /// Sets the projection so 0,0 is in the lower left of the window
    /// and each pixel is 1 unit wide/tall.
    pub fn ortho(&mut self) {
        unsafe { Fl_Glut_Window_ortho(self.inner.widget() as _) }
    }

    /// Returns whether the GlutWindow can do overlay
    pub fn can_do_overlay(&self) -> bool {
        unsafe { Fl_Glut_Window_can_do_overlay(self.inner.widget() as _) != 0 }
    }

    /// Redraws the overlay
    pub fn redraw_overlay(&mut self) {
        unsafe { Fl_Glut_Window_redraw_overlay(self.inner.widget() as _) }
    }

    /// Hides the overlay
    pub fn hide_overlay(&mut self) {
        unsafe { Fl_Glut_Window_hide_overlay(self.inner.widget() as _) }
    }

    /// Makes the overlay current
    pub fn make_overlay_current(&mut self) {
        unsafe { Fl_Glut_Window_make_overlay_current(self.inner.widget() as _) }
    }

    /// Returns the pixels per unit/point
    pub fn pixels_per_unit(&self) -> f32 {
        unsafe { Fl_Glut_Window_pixels_per_unit(self.inner.widget() as _) }
    }

    /// Gets the window's width in pixels
    pub fn pixel_w(&self) -> i32 {
        unsafe { Fl_Glut_Window_pixel_w(self.inner.widget() as _) }
    }

    /// Gets the window's height in pixels
    pub fn pixel_h(&self) -> i32 {
        unsafe { Fl_Glut_Window_pixel_h(self.inner.widget() as _) }
    }

    /// Get the Mode of the GlutWindow
    pub fn mode(&self) -> Mode {
        unsafe { mem::transmute(Fl_Glut_Window_mode(self.inner.widget() as _)) }
    }

    /// Set the Mode of the GlutWindow
    pub fn set_mode(&mut self, mode: Mode) {
        unsafe {
            Fl_Glut_Window_set_mode(self.inner.widget() as _, mode.bits());
        }
    }
}

#[cfg(feature = "enable-glwindow")]
/// Alias GlutWindow as GlWindow
pub type GlWindow = GlutWindow;

#[doc(hidden)]
/// An experimental space for unstable Windowing widgets, currently the GlWidgetWindow
pub mod experimental {
    use super::*;

    /// Creates a OpenGL Glut window widget
    #[cfg(feature = "enable-glwindow")]
    #[derive(Debug)]
    pub struct GlWidgetWindow {
        inner: crate::widget::WidgetTracker,
        is_derived: bool,
    }

    #[cfg(feature = "enable-glwindow")]
    crate::macros::widget::impl_widget_ext!(GlWidgetWindow, Fl_Gl_Window);
    #[cfg(feature = "enable-glwindow")]
    crate::macros::widget::impl_widget_base!(GlWidgetWindow, Fl_Gl_Window);
    #[cfg(feature = "enable-glwindow")]
    crate::macros::group::impl_group_ext!(GlWidgetWindow, Fl_Gl_Window);
    #[cfg(feature = "enable-glwindow")]
    crate::macros::window::impl_window_ext!(GlWidgetWindow, Fl_Gl_Window);

    #[cfg(feature = "enable-glwindow")]
    impl_top_win!(GlWidgetWindow);

    #[cfg(feature = "enable-glwindow")]
    impl Default for GlWidgetWindow {
        fn default() -> GlWidgetWindow {
            assert!(crate::app::is_ui_thread());
            let mut win = GlWidgetWindow::new(0, 0, 0, 0, None);
            win.free_position();
            win.set_frame(FrameType::FlatBox);
            win.begin();
            win
        }
    }

    #[cfg(feature = "enable-glwindow")]
    impl GlWidgetWindow {
        /// Creates a new window, with title as its window title if the window is decorated
        pub fn new<'a, T: Into<Option<&'a str>>>(x: i32, y: i32, w: i32, h: i32, title: T) -> Self {
            let temp = if let Some(title) = title.into() {
                CString::safe_new(title).into_raw()
            } else {
                std::ptr::null_mut()
            };
            unsafe {
                let widget_ptr = Fl_Gl_Window_new(x, y, w, h, temp);
                assert!(!widget_ptr.is_null());
                assert!(crate::app::is_ui_thread());
                let tracker = crate::widget::WidgetTracker::new(widget_ptr as _);
                unsafe extern "C" fn shim(wid: *mut Fl_Widget, _data: *mut std::os::raw::c_void) { unsafe {
                    let user_data = Fl_Gl_Window_user_data(wid as _);
                    let draw_data = Fl_Gl_Window_draw_data(wid as _);
                    let handle_data = Fl_Gl_Window_handle_data(wid as _);
                    crate::app::add_timeout(0., move |h| {
                        if !user_data.is_null() {
                            let _x = Box::from_raw(user_data as *mut Box<dyn FnMut()>);
                        }
                        if !draw_data.is_null() {
                            let _x = Box::from_raw(draw_data as *mut Box<dyn FnMut()>);
                        }
                        if !handle_data.is_null() {
                            let _x = Box::from_raw(handle_data as *mut Box<dyn FnMut()>);
                        }
                        crate::app::remove_timeout(h);
                    });
                }}
                Fl_Gl_Window_set_deletion_callback(widget_ptr, Some(shim), std::ptr::null_mut());
                Self {
                    inner: tracker,
                    is_derived: true,
                }
            }
        }

        /// Gets an opengl function address
        pub fn get_proc_address(&self, s: &str) -> *const raw::c_void {
            unsafe extern "C" {
                pub fn get_proc(name: *const raw::c_char) -> *mut raw::c_void;
            }
            let s = CString::safe_new(s);
            let ret = unsafe { get_proc(s.as_ptr() as _) };
            if !ret.is_null() {
                ret as *const _
            } else {
                unsafe {
                    Fl_Gl_Window_get_proc_address(self.inner.widget() as _, s.as_ptr()) as *const _
                }
            }
        }

        /// Forces the window to be drawn, this window is also made current and calls draw()
        pub fn flush(&mut self) {
            unsafe { Fl_Gl_Window_flush(self.inner.widget() as _) }
        }

        /// Returns whether the OpeGL context is still valid
        pub fn valid(&self) -> bool {
            unsafe { Fl_Gl_Window_valid(self.inner.widget() as _) != 0 }
        }

        /// Mark the OpeGL context as still valid
        pub fn set_valid(&mut self, v: bool) {
            unsafe { Fl_Gl_Window_set_valid(self.inner.widget() as _, v as raw::c_char) }
        }

        /// Returns whether the context is valid upon creation
        pub fn context_valid(&self) -> bool {
            unsafe { Fl_Gl_Window_context_valid(self.inner.widget() as _) != 0 }
        }

        /// Mark the context as valid upon creation
        pub fn set_context_valid(&mut self, v: bool) {
            unsafe { Fl_Gl_Window_set_context_valid(self.inner.widget() as _, v as raw::c_char) }
        }

        /// Returns the GlContext
        pub fn context(&self) -> Option<GlContext> {
            unsafe {
                let ctx = Fl_Gl_Window_context(self.inner.widget() as _);
                if ctx.is_null() { None } else { Some(ctx) }
            }
        }

        #[allow(clippy::not_unsafe_ptr_arg_deref)]
        /// Sets the GlContext
        pub fn set_context(&mut self, ctx: GlContext, destroy_flag: bool) {
            assert!(!ctx.is_null());
            unsafe { Fl_Gl_Window_set_context(self.inner.widget() as _, ctx, destroy_flag as i32) }
        }

        /// Swaps the back and front buffers
        pub fn swap_buffers(&mut self) {
            unsafe { Fl_Gl_Window_swap_buffers(self.inner.widget() as _) }
        }

        /// Gets the swap interval
        pub fn swap_interval(&self) -> i32 {
            unsafe { Fl_Gl_Window_swap_interval(self.inner.widget() as _) }
        }

        /// Sets the swap interval
        pub fn set_swap_interval(&mut self, frames: i32) {
            unsafe { Fl_Gl_Window_set_swap_interval(self.inner.widget() as _, frames) }
        }

        /// Sets the projection so 0,0 is in the lower left of the window
        /// and each pixel is 1 unit wide/tall.
        pub fn ortho(&mut self) {
            unsafe { Fl_Gl_Window_ortho(self.inner.widget() as _) }
        }

        /// Returns whether the GlutWindow can do overlay
        pub fn can_do_overlay(&self) -> bool {
            unsafe { Fl_Gl_Window_can_do_overlay(self.inner.widget() as _) != 0 }
        }

        /// Redraws the overlay
        pub fn redraw_overlay(&mut self) {
            unsafe { Fl_Gl_Window_redraw_overlay(self.inner.widget() as _) }
        }

        /// Hides the overlay
        pub fn hide_overlay(&mut self) {
            unsafe { Fl_Gl_Window_hide_overlay(self.inner.widget() as _) }
        }

        /// Makes the overlay current
        pub fn make_overlay_current(&mut self) {
            unsafe { Fl_Gl_Window_make_overlay_current(self.inner.widget() as _) }
        }

        /// Returns the pixels per unit/point
        pub fn pixels_per_unit(&self) -> f32 {
            unsafe { Fl_Gl_Window_pixels_per_unit(self.inner.widget() as _) }
        }

        /// Gets the window's width in pixels
        pub fn pixel_w(&self) -> i32 {
            unsafe { Fl_Gl_Window_pixel_w(self.inner.widget() as _) }
        }

        /// Gets the window's height in pixels
        pub fn pixel_h(&self) -> i32 {
            unsafe { Fl_Gl_Window_pixel_h(self.inner.widget() as _) }
        }

        /// Get the Mode of the GlutWindow
        pub fn mode(&self) -> Mode {
            unsafe { mem::transmute(Fl_Gl_Window_mode(self.inner.widget() as _)) }
        }

        /// Set the Mode of the GlutWindow
        pub fn set_mode(&mut self, mode: Mode) {
            unsafe {
                Fl_Gl_Window_set_mode(self.inner.widget() as _, mode.bits());
            }
        }
    }
}
