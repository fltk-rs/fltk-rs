use fltk::{app::*, button::*, frame::*, window::*};
use std::{thread, time};

fn main() {
    let app = App::default();
    let mut wind = Window::default()
        .with_size(400, 300)
        .center_screen()
        .with_label("Hello");
    let frame = Frame::new(0, 0, 400, 200, "");
    let mut but1 = Button::new(160, 210, 80, 40, "Click me!");
    but1.set_callback(Box::new(|| {
        let mut frame = frame.clone();
        thread::spawn(move|| {
            thread::sleep(time::Duration::from_secs(1));
            frame.set_label("Hello");
        });
    }));
    wind.show();
    app.run().unwrap();
}