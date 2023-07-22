# fltk-sys

Raw bindings for FLTK. These are generated using bindgen on the cfltk headers.

## Usage
```toml
[dependencies]
fltk-sys = "1.4"
```

Example code:
```rust,no_run
use fltk_sys::*;
use std::os::raw::*;

unsafe extern "C" fn cb(_wid: *mut button::Fl_Widget, data: *mut c_void) {
    let frame = data as *mut frame::Fl_Box;
    frame::Fl_Box_set_label(frame, "Hello World\0".as_ptr() as *const _);
}

fn main() {
    unsafe {
        fl::Fl_init_all();
        image::Fl_register_images();
        fl::Fl_lock();
        let win = window::Fl_Window_new(100, 100, 400, 300, "Window\0".as_ptr() as *const _);
        let frame = frame::Fl_Box_new(0, 0, 400, 200, std::ptr::null());
        let but = button::Fl_Button_new(160, 220, 80, 40, "Click\0".as_ptr() as *const _);
        window::Fl_Window_end(win);
        window::Fl_Window_show(win);

        button::Fl_Button_set_callback(but, Some(cb), frame as *mut _);

        fl::Fl_run();
    }
}
```

## Dependencies
CMake > 3.14, git and a C++17 compiler. The dev dependencies are basically the same as for [fltk-rs](https://github.com/fltk-rs/fltk-rs#dependencies).

## Why you might want to use fltk-sys directly
- If you need an abi stable cdylib that you can call into (as a plugin system for example).
- To create your own wrapper around certain elements if you don't need the whole fltk crate.
- fltk-sys, although memory and thread unsafe, is panic-safe.
- You need a no-std gui library, in such case, you can replace the `std::` prefix with the `libc` via bindgen (requires adding libc as a dependency).
- Wrapping a 3rd-party widget like in [fltk-flow](https://github.com/fltk-rs/fltk-flow).