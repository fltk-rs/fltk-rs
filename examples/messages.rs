use fltk::{app::*, frame::*, window::*};

#[derive(Debug, Copy, Clone)]
pub enum Message {
    Increment(i32),
    Decrement(i32),
}

fn inc_frame(frame: &mut Frame, val: &mut i32, step: i32) {
    *val = *val + step;
    frame.set_label(&val.to_string());
}

fn main() {
    let app = App::default();
    let mut wind = Window::new(100, 100, 400, 300, "Hello from rust");
    let mut frame = Frame::new(0, 0, 400, 300, "0");
    wind.end();
    wind.show();
    let (s, r) = std::sync::mpsc::channel::<Message>();
    std::thread::spawn(move || loop {
        std::thread::sleep(std::time::Duration::from_millis(1000));
        s.send(Message::Increment(2)).unwrap();
    });
    let mut val = 0;
    while app.wait() {
        match r.recv().unwrap() {
            Message::Increment(step) => inc_frame(&mut frame, &mut val, step),
            _ => (),
        }
    }
}
