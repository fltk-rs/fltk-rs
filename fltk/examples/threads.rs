// fltk-rs surrounds all mutating calls to widgets with a lock on the C++ wrapper side.
// Normally you wouldn't have to call app::lock() and app::unlock().
// This depends however on the support of recursive mutexes in your system.
// If you notice haning in multithreaded applications,
// you might have to initialize threads (like xlib threads) by calling app::lock() once in your main thread.
// In that case, you can wrap widgets in an Arc<Mutex>
// or surround widget-mutating functions/methods with an app::lock and app::unlock.
// But that should rarely be required.

use fltk::{app::*, button::*, frame::*, window::*};
use std::{thread, time};

fn main() {
    let app = App::default();
    let mut wind = Window::default()
        .with_size(400, 300)
        .center_screen()
        .with_label("Hello");
    let frame = Frame::new(0, 0, 400, 200, "");
    let mut but = Button::new(160, 210, 80, 40, "Click me!");

    wind.show();

    but.set_callback(move || {
        let mut frame = frame.clone();
        thread::spawn(move || {
            for i in 0..1000 {
                thread::sleep(time::Duration::from_millis(10));
                frame.set_label(format!("Hello {}", i).as_str());
            }
        });
    });

    app.run().unwrap();
}

// // Using messages
// fn main() {
//     let app = App::default();
//     let mut wind = Window::default()
//         .with_size(400, 300)
//         .center_screen()
//         .with_label("Hello");
//     let mut frame = Frame::new(0, 0, 400, 200, "");
//     let mut but = Button::new(160, 210, 80, 40, "Click me!");

//     wind.show();

//     let (s, r) = channel::<i32>();

//     but.set_callback(move || {
//         // let mut frame = frame.clone();
//         thread::spawn(move|| {
//             for i in 0..1000 {
//                 thread::sleep(time::Duration::from_millis(10));
//                 // frame.set_label(format!("Hello {}", i).as_str());
//                 s.send(i);
//             }
//         });
//     });

//     while app.wait() {
//         let msg = r.recv();
//         match msg {
//             Some(val) => frame.set_label(format!("Hello {}", val).as_str()),
//             None => (),
//         }
//     }
// }

// // Using an Arc<Mutex>>
// fn main() {
//     let app = App::default();
//     let mut wind = Window::default()
//         .with_size(400, 300)
//         .center_screen()
//         .with_label("Hello");
//     let mut but = Button::new(160, 210, 80, 40, "Click me!");
//     use std::sync::{Arc, Mutex};
//     let frame = Arc::from(Mutex::from(Frame::new(0, 0, 400, 200, "")));
//     wind.show();
//     but.set_callback(move || {
//         let frame = frame.clone();
//         thread::spawn(move|| {
//             for i in 0..1000 {
//                 thread::sleep(time::Duration::from_millis(10));
//                 let mut frame = frame.lock().unwrap();
//                 frame.set_label(format!("Hello {}", i).as_str());
//             }
//         });
//     });
//     app.run().unwrap();
// }
