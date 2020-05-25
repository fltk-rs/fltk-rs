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

    but.set_callback(Box::new(move || {
        let mut frame = frame.clone();
        thread::spawn(move|| {
            for i in 0..1000 {
                thread::sleep(time::Duration::from_millis(10));
                frame.set_label(format!("Hello {}", i).as_str());
            }
        });
    }));

    app.run().unwrap();
}

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
//     but.set_callback(Box::new(move || {
//         let frame = frame.clone();
//         thread::spawn(move|| {
//             for i in 0..1000 {
//                 thread::sleep(time::Duration::from_millis(10));
//                 let mut frame = frame.lock().unwrap();
//                 frame.set_label(format!("Hello {}", i).as_str());
//             }
//         });
//     }));
//     app.run().unwrap();
// }