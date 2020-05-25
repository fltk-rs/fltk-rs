// Works, but requires adding tokio

// // Example 1 using Arc, Mutex
//
// use fltk::{app::*, button::*, frame::*, window::*};
// use std::sync::Arc;
// use std::time::Duration;
// use tokio::sync::Mutex;
//
// async fn hello() -> String {
//     tokio::time::delay_for(Duration::from_secs(2)).await;
//     String::from("Hello, world!")
// }
//
// #[tokio::main]
// async fn main() {
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
//         tokio::spawn(async move {
//             let msg = hello().await;
//             let mut frame = frame.lock().await;
//             frame.set_label(&msg);
//         });
//     }));
//
//     app.run().unwrap();
// }

// // Example2 using channels
//
// use fltk::{app::*, button::*, frame::*, window::*};
// use tokio::sync::mpsc::channel;
// use std::time::Duration;

// #[derive(Debug, Clone, Copy)]
// pub enum Message {
//     Inc(i32),
//     Dec(i32),
// }

// async fn inc_val() -> i32 {
//     tokio::time::delay_for(Duration::from_secs(1)).await;
//     1
// }

// #[tokio::main]
// async fn main() {
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
//         tokio::spawn(async move {
//             let v = inc_val().await;
//             s.try_send(Message::Inc(v)).unwrap();
//         });
//     }));
//
//     while app.wait().unwrap() {
//         match r.try_recv() {
//             Ok(Message::Inc(v)) => frame.set_label(&format!("increments by {}", v)),
//             _ => (),
//         }
//     }
// }

fn main() {}
