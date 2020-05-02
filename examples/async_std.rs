use async_std::sync::{Arc, Mutex};
use async_std::task;
use fltk::{app::*, button::*, frame::*, window::*};
use std::time::Duration;

async fn hello() -> String {
    task::sleep(Duration::from_secs(2)).await;
    String::from("Hello, world!")
}

fn main() {
    let app = App::default();
    let mut wind = Window::new(100, 100, 400, 300, "Hello from rust");
    let frame = Frame::new(0, 0, 400, 200, "");
    let mut but = Button::new(160, 210, 80, 40, "Click me!");
    wind.end();
    wind.show();
    let frame = Arc::from(Mutex::from(frame));
    but.set_callback(Box::new(move || {
        let frame = frame.clone();
        task::spawn(async move {
            let msg = hello().await;
            let mut frame = frame.lock().await;
            frame.set_label(&msg);
        });
    }));
    app.run().unwrap();
}
