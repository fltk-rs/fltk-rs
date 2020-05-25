// Works, but requires adding async_std

// // Example1 using callbacks and Arc, Mutex
//
// use async_std::sync::{Arc, Mutex};
// use async_std::task;
// use fltk::{app::*, button::*, frame::*, window::*};
// use std::time::Duration;
//
// async fn hello() -> String {
//     task::sleep(Duration::from_secs(2)).await;
//     String::from("Hello, world!")
// }
//
// fn main() {
//     let app = App::default();
//     let mut wind = Window::new(100, 100, 400, 300, "Hello from rust");
//     let frame = Frame::new(0, 0, 400, 200, "");
//     let mut but = Button::new(160, 210, 80, 40, "Click me!");
//     wind.end();
//     wind.show();
//
//     let frame = Arc::from(Mutex::from(frame));
//
//     but.set_callback(Box::new(move || {
//         let frame = frame.clone();
//         task::spawn(async move {
//             let msg = hello().await;
//             let mut frame = frame.lock().await;
//             frame.set_label(&msg);
//         });
//     }));
//
//     app.run().unwrap();
// }

// // Example2 using future channels
//
// use async_std::task;
// use fltk::{app::*, button::*, frame::*, window::*};
// use futures::channel::mpsc::channel;
// use std::time::Duration;
//
// #[derive(Debug, Clone, Copy)]
// pub enum Message {
//     Inc(i32),
//     Dec(i32),
// }
//
// async fn inc_val() -> i32 {
//     task::sleep(Duration::from_secs(1)).await;
//     1
// }
//
// fn main() {
//     let app = App::default();
//     let mut wind = Window::new(100, 100, 400, 300, "Hello from rust");
//     let mut frame = Frame::new(0, 0, 400, 200, "");
//     let mut but = Button::new(160, 210, 80, 40, "Click me!");
//     wind.end();
//     wind.show();
//
//     let (s, mut r) = channel::<Message>(10);
//
//     but.set_callback(Box::new(move || {
//         let mut s = s.clone();
//         task::spawn(async move {
//             let v = inc_val().await;
//             s.try_send(Message::Inc(v)).unwrap();
//         });
//     }));
//
//     while app.wait().unwrap() {
//         match r.try_next() {
//             Ok(Some(Message::Inc(v))) => frame.set_label(&format!("increments by {}", v)),
//             _ => (),
//         }
//     }
// }


fn main() {}
