use fltk::{app, frame::Frame, prelude::*, window::Window};

#[derive(Debug, Copy, Clone)]
pub enum Message {
    Increment(i32),
    Decrement(i32),
}

fn inc_frame(frame: &mut Frame, val: &mut i32, step: i32) {
    *val += step;
    frame.set_label(&val.to_string());
}

fn main() {
    let app = app::App::default();
    let mut wind = Window::default().with_size(400, 300);
    let mut frame = Frame::default().size_of(&wind).with_label("0");

    let mut val = 0;

    wind.show();

    let (s, r) = app::channel::<Message>();

    std::thread::spawn(move || loop {
        app::sleep(1.);
        s.send(Message::Increment(2));
    });

    while app.wait() {
        if let Some(msg) = r.recv() {
            if let Message::Increment(step) = msg {
                inc_frame(&mut frame, &mut val, step)
            }
        }
    }
}
