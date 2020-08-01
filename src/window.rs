use crate::app::*;
use crate::image::Image;
pub use crate::prelude::*;
use crate::widget::*;
use fltk_sys::window::*;
use std::{
    ffi::{CStr, CString},
    mem,
    os::raw,
};

/// Opaque raw window handle (*mut c_void to HWND on Windows and NSWindow on MacOS)
/// XID (u64) raw window handle for X11
#[cfg(any(
    target_os = "windows",
    target_os = "macos",
    target_os = "ios",
    target_os = "android"
))]
pub type RawHandle = *mut raw::c_void;

/// Opaque raw window handle (*mut c_void to HWND on Windows and NSWindow on MacOS)
/// XID (u64) raw window handle for X11
#[cfg(any(
    target_os = "linux",
    target_os = "dragonfly",
    target_os = "freebsd",
    target_os = "netbsd",
    target_os = "openbsd",
))]
pub type RawHandle = u64;

/// Creates a window widget
#[derive(WidgetExt, GroupExt, WindowExt, Debug)]
pub struct Window {
    _inner: *mut Fl_Window,
    _tracker: *mut fltk_sys::fl::Fl_Widget_Tracker,
}

impl Window {
    /// Find an Fl_Window through a raw handle. The window must have been instatiated by the app
    /// void pointer to: (Windows: HWND, X11: Xid (u64), MacOS: NSWindow)
    /// # Safety
    /// The data must be valid and is OS-dependent.
    pub unsafe fn find_by_handle(handle: RawHandle) -> Option<Window> {
        let ptr = Fl_Window_find_by_handle(mem::transmute(&handle));
        if ptr.is_null() {
            None
        } else {
            Some(Window::from_widget_ptr(
                ptr as *mut fltk_sys::widget::Fl_Widget,
            ))
        }
    }

    /// Use FLTK specific arguments for the application:
    /// More info: https://www.fltk.org/doc-1.3/classFl.html#a1576b8c9ca3e900daaa5c36ca0e7ae48
    /// The options are:
    /// -bg2 color
    /// -bg color
    /// -di[splay] host:n.n
    /// -dn[d]
    /// -fg color
    /// -g[eometry] WxH+X+Y
    /// -i[conic]
    /// -k[bd]
    /// -na[me] classname
    /// -nod[nd]
    /// -nok[bd]
    /// -not[ooltips]
    /// -s[cheme] scheme
    /// -ti[tle] windowtitle
    /// -to[oltips]
    pub fn show_with_env_args(&mut self) {
        assert!(!self.was_deleted());
        unsafe {
            let args: Vec<String> = std::env::args().collect();
            let len = args.len() as i32;
            let mut v: Vec<*mut raw::c_char> = vec![];
            for arg in args {
                v.push(CString::new(arg.as_str()).unwrap().into_raw() as *mut raw::c_char);
            }
            let mut v = mem::ManuallyDrop::new(v);
            Fl_Window_show_with_args(self._inner as *mut Fl_Window, len, v.as_mut_ptr())
        }
    }

    /// Use FLTK specific arguments for the application:
    /// More info: https://www.fltk.org/doc-1.3/classFl.html#a1576b8c9ca3e900daaa5c36ca0e7ae48
    /// The options are:
    /// -bg2 color
    /// -bg color
    /// -di[splay] host:n.n
    /// -dn[d]
    /// -fg color
    /// -g[eometry] WxH+X+Y
    /// -i[conic]
    /// -k[bd]
    /// -na[me] classname
    /// -nod[nd]
    /// -nok[bd]
    /// -not[ooltips]
    /// -s[cheme] scheme
    /// -ti[tle] windowtitle
    /// -to[oltips]
    pub fn show_with_args(&mut self, args: &[&str]) {
        assert!(!self.was_deleted());
        unsafe {
            let mut temp = vec![""];
            temp.extend(args);
            let len = temp.len() as i32;
            let mut v: Vec<*mut raw::c_char> = vec![];
            for arg in temp {
                v.push(CString::new(arg).unwrap().into_raw() as *mut raw::c_char);
            }
            let mut v = mem::ManuallyDrop::new(v);
            Fl_Window_show_with_args(self._inner as *mut Fl_Window, len, v.as_mut_ptr())
        }
    }
}

/// Defines the window type, can be set dynamically using the set_type() method
#[repr(i32)]
#[derive(WidgetType, Debug, Copy, Clone, PartialEq)]
pub enum WindowType {
    Normal = 240,
    Double = 241,
}

/// Creates a single (buffered) window widget
#[derive(WidgetExt, GroupExt, WindowExt, Debug)]
pub struct SingleWindow {
    _inner: *mut Fl_Single_Window,
    _tracker: *mut fltk_sys::fl::Fl_Widget_Tracker,
}

impl SingleWindow {
    /// Use FLTK specific arguments for the application:
    /// More info: https://www.fltk.org/doc-1.3/classFl.html#a1576b8c9ca3e900daaa5c36ca0e7ae48
    /// The options are:
    /// -bg2 color
    /// -bg color
    /// -di[splay] host:n.n
    /// -dn[d]
    /// -fg color
    /// -g[eometry] WxH+X+Y
    /// -i[conic]
    /// -k[bd]
    /// -na[me] classname
    /// -nod[nd]
    /// -nok[bd]
    /// -not[ooltips]
    /// -s[cheme] scheme
    /// -ti[tle] windowtitle
    /// -to[oltips]
    pub fn show_with_env_args(&mut self) {
        assert!(!self.was_deleted());
        unsafe {
            let args: Vec<String> = std::env::args().collect();
            let len = args.len() as i32;
            let mut v: Vec<*mut raw::c_char> = vec![];
            for arg in args {
                v.push(CString::new(arg.as_str()).unwrap().into_raw() as *mut raw::c_char);
            }
            let mut v = mem::ManuallyDrop::new(v);
            Fl_Window_show_with_args(self._inner as *mut Fl_Window, len, v.as_mut_ptr())
        }
    }

    /// Use FLTK specific arguments for the application:
    /// More info: https://www.fltk.org/doc-1.3/classFl.html#a1576b8c9ca3e900daaa5c36ca0e7ae48
    /// The options are:
    /// -bg2 color
    /// -bg color
    /// -di[splay] host:n.n
    /// -dn[d]
    /// -fg color
    /// -g[eometry] WxH+X+Y
    /// -i[conic]
    /// -k[bd]
    /// -na[me] classname
    /// -nod[nd]
    /// -nok[bd]
    /// -not[ooltips]
    /// -s[cheme] scheme
    /// -ti[tle] windowtitle
    /// -to[oltips]
    pub fn show_with_args(&mut self, args: &[&str]) {
        assert!(!self.was_deleted());
        unsafe {
            let mut temp = vec![""];
            temp.extend(args);
            let len = temp.len() as i32;
            let mut v: Vec<*mut raw::c_char> = vec![];
            for arg in temp {
                v.push(CString::new(arg).unwrap().into_raw() as *mut raw::c_char);
            }
            let mut v = mem::ManuallyDrop::new(v);
            Fl_Window_show_with_args(self._inner as *mut Fl_Window, len, v.as_mut_ptr())
        }
    }
}

/// Creates a double (buffered) window widget
#[derive(WidgetExt, GroupExt, WindowExt, Debug)]
pub struct DoubleWindow {
    _inner: *mut Fl_Double_Window,
    _tracker: *mut fltk_sys::fl::Fl_Widget_Tracker,
}

impl DoubleWindow {
    /// Use FLTK specific arguments for the application:
    /// More info: https://www.fltk.org/doc-1.3/classFl.html#a1576b8c9ca3e900daaa5c36ca0e7ae48
    /// The options are:
    /// -bg2 color
    /// -bg color
    /// -di[splay] host:n.n
    /// -dn[d]
    /// -fg color
    /// -g[eometry] WxH+X+Y
    /// -i[conic]
    /// -k[bd]
    /// -na[me] classname
    /// -nod[nd]
    /// -nok[bd]
    /// -not[ooltips]
    /// -s[cheme] scheme
    /// -ti[tle] windowtitle
    /// -to[oltips]
    pub fn show_with_env_args(&mut self) {
        assert!(!self.was_deleted());
        unsafe {
            let args: Vec<String> = std::env::args().collect();
            let len = args.len() as i32;
            let mut v: Vec<*mut raw::c_char> = vec![];
            for arg in args {
                v.push(CString::new(arg.as_str()).unwrap().into_raw() as *mut raw::c_char);
            }
            let mut v = mem::ManuallyDrop::new(v);
            Fl_Window_show_with_args(self._inner as *mut Fl_Window, len, v.as_mut_ptr())
        }
    }

    /// Use FLTK specific arguments for the application:
    /// More info: https://www.fltk.org/doc-1.3/classFl.html#a1576b8c9ca3e900daaa5c36ca0e7ae48
    /// The options are:
    /// -bg2 color
    /// -bg color
    /// -di[splay] host:n.n
    /// -dn[d]
    /// -fg color
    /// -g[eometry] WxH+X+Y
    /// -i[conic]
    /// -k[bd]
    /// -na[me] classname
    /// -nod[nd]
    /// -nok[bd]
    /// -not[ooltips]
    /// -s[cheme] scheme
    /// -ti[tle] windowtitle
    /// -to[oltips]
    pub fn show_with_args(&mut self, args: &[&str]) {
        assert!(!self.was_deleted());
        unsafe {
            let mut temp = vec![""];
            temp.extend(args);
            let len = temp.len() as i32;
            let mut v: Vec<*mut raw::c_char> = vec![];
            for arg in temp {
                v.push(CString::new(arg).unwrap().into_raw() as *mut raw::c_char);
            }
            let mut v = mem::ManuallyDrop::new(v);
            Fl_Window_show_with_args(self._inner as *mut Fl_Window, len, v.as_mut_ptr())
        }
    }
}

/// Creates a Menu window widget
#[derive(WidgetExt, GroupExt, WindowExt, Debug)]
pub struct MenuWindow {
    _inner: *mut Fl_Menu_Window,
    _tracker: *mut fltk_sys::fl::Fl_Widget_Tracker,
}

/// A wrapper around a raw OpenGL context
pub type GlContext = *mut raw::c_void;

/// Creates a OpenGL window widget
#[cfg(feature = "enable-glwindow")]
#[derive(WidgetExt, GroupExt, WindowExt, Debug)]
pub struct GlWindow {
    _inner: *mut Fl_Gl_Window,
    _tracker: *mut fltk_sys::fl::Fl_Widget_Tracker,
}

#[cfg(feature = "enable-glwindow")]
impl GlWindow {
    /// Flush window content
    pub fn flush(&mut self) {
        assert!(!self.was_deleted());
        unsafe { Fl_Gl_Window_flush(self._inner) }
    }

    /// Returns whether the OpeGL context is still valid
    pub fn valid(&self) -> bool {
        assert!(!self.was_deleted());
        unsafe {
            match Fl_Gl_Window_valid(self._inner) {
                0 => false,
                _ => true,
            }
        }
    }

    /// Mark the OpeGL context as still valid
    pub fn set_valid(&mut self, v: bool) {
        assert!(!self.was_deleted());
        unsafe { Fl_Gl_Window_set_valid(self._inner, v as raw::c_char) }
    }

    /// Returns whether the context is valid upon creation
    pub fn context_valid(&self) -> bool {
        assert!(!self.was_deleted());
        unsafe {
            match Fl_Gl_Window_context_valid(self._inner) {
                0 => false,
                _ => true,
            }
        }
    }

    /// Mark the context as valid upon creation
    pub fn set_context_valid(&mut self, v: bool) {
        assert!(!self.was_deleted());
        unsafe { Fl_Gl_Window_set_context_valid(self._inner, v as raw::c_char) }
    }

    /// Returns the GlContext
    pub fn context(&self) -> Option<GlContext> {
        assert!(!self.was_deleted());
        unsafe {
            let ctx = Fl_Gl_Window_context(self._inner);
            if ctx.is_null() {
                None
            } else {
                Some(ctx)
            }
        }
    }

    /// Sets the GlContext
    pub fn set_context(&mut self, ctx: GlContext, destroy_flag: bool) {
        assert!(!self.was_deleted());
        assert!(!ctx.is_null());
        unsafe { Fl_Gl_Window_set_context(self._inner, ctx, destroy_flag as i32) }
    }

    /// Swaps the back and front buffers
    pub fn swap_buffers(&mut self) {
        assert!(!self.was_deleted());
        unsafe { Fl_Gl_Window_swap_buffers(self._inner) }
    }

    /// Sets the projection so 0,0 is in the lower left of the window
    /// and each pixel is 1 unit wide/tall.
    pub fn ortho(&mut self) {
        assert!(!self.was_deleted());
        unsafe { Fl_Gl_Window_ortho(self._inner) }
    }

    /// Returns whether the GlWindow can do overlay
    pub fn can_do_overlay(&mut self) -> bool {
        assert!(!self.was_deleted());
        unsafe {
            match Fl_Gl_Window_can_do_overlay(self._inner) {
                0 => false,
                _ => true,
            }
        }
    }

    /// Redraws the overlay
    pub fn redraw_overlay(&mut self) {
        assert!(!self.was_deleted());
        unsafe { Fl_Gl_Window_redraw_overlay(self._inner) }
    }

    /// Hides the overlay
    pub fn hide_overlay(&mut self) {
        assert!(!self.was_deleted());
        unsafe { Fl_Gl_Window_hide_overlay(self._inner) }
    }

    /// Makes the overlay current
    pub fn make_overlay_current(&mut self) {
        assert!(!self.was_deleted());
        unsafe { Fl_Gl_Window_make_overlay_current(self._inner) }
    }

    /// Returns the pixels per unit
    pub fn pixels_per_unit(&mut self) -> f32 {
        assert!(!self.was_deleted());
        unsafe { Fl_Gl_Window_pixels_per_unit(self._inner) }
    }

    /// Gets the window's width in pixels
    pub fn pixel_w(&mut self) -> i32 {
        assert!(!self.was_deleted());
        unsafe { Fl_Gl_Window_pixel_w(self._inner) }
    }

    /// Gets the window's height in pixels
    pub fn pixel_h(&mut self) -> i32 {
        assert!(!self.was_deleted());
        unsafe { Fl_Gl_Window_pixel_h(self._inner) }
    }

    /// Get the Mode of the GlWindow
    pub fn mode(&self) -> Mode {
        assert!(!self.was_deleted());
        unsafe { mem::transmute(Fl_Gl_Window_mode(self._inner)) }
    }

    /// Set the Mode of the GlWindow
    pub fn set_mode(&mut self, mode: Mode) {
        assert!(!self.was_deleted());
        unsafe {
            Fl_Gl_Window_set_mode(self._inner, mode as i32);
        }
    }
}
