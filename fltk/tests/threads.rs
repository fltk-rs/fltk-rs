// fltk-rs surrounds all mutating calls to widgets with a lock on the C++ wrapper side.
// Normally you wouldn't have to call app::lock() and app::unlock().
// This depends however on the support of recursive mutexes in your system.
// If you notice haning in multithreaded applications,
// you might have to initialize threads (like xlib threads) by calling app::lock() once in your main thread.
// In that case, you can wrap widgets in an Arc<Mutex>
// or surround widget-mutating functions/methods with an app::lock and app::unlock.
// But that should rarely be required.

use fltk::{app, button::Button, frame::Frame, prelude::*, window::Window};

#[test]
fn raw_threads() {
    let app = app::App::default();
    let mut wind = Window::default()
        .with_size(400, 300)
        .center_screen()
        .with_label("threads");
    let frame = Frame::new(0, 0, 400, 200, "");
    let mut but = Button::new(160, 210, 80, 40, "Click me!");

    wind.show();

    but.set_callback(move |_| {
        let mut frame = frame.clone();
        std::thread::spawn(move || {
            for i in 0..1000 {
                app::sleep(0.010);
                frame.set_label(format!("Hello {}", i).as_str());
            }
        });
    });

    app.run().unwrap();
}
