// This is an example of using c_style callbacks.

use fltk::*;
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

fn main() {
    let app = app::App::default();
    let mut wind = window::Window::new(100, 100, 400, 300, "Hello from rust");
    let frame = frame::Frame::new(0, 0, 400, 200, "");
    let mut but = button::Button::new(160, 210, 80, 40, "Click me!");

    wind.end();
    wind.show();

    unsafe {
        // If no data needs to be passed, you can pass 0 as *mut _
        app::set_raw_callback(&mut but, frame.as_widget_ptr() as *mut _, Some(cb));
        // // Using a closure also works
        // app::set_raw_callback(&mut but, frame.as_widget_ptr() as *mut _, Some(|_ , _| { println!("Also works!")});
    }

    app.run().unwrap();
}
