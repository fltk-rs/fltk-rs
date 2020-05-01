use fltk::{app::*, frame::*, window::*};

#[derive(Debug, Copy, Clone)]
pub enum Message {
    Increment,
    Decrement,
}

fn inc_frame(frame: &mut Frame, val: &mut i32) {
    *val = *val + 1;
    frame.set_label(&val.to_string());
}

fn main() {
    let app = App::default();
    let mut wind = Window::new(100, 100, 400, 300, "Hello from rust");
    let mut frame = Frame::new(0, 0, 400, 300, "");
    let mut val = 0;
    wind.show();
    std::thread::spawn(move || loop {
        std::thread::sleep(std::time::Duration::from_millis(1000));
        awake_msg(Message::Increment);
    });
    while app.wait() {
        let msg = thread_msg::<Message>();
        match msg {
            Some(Message::Increment) => inc_frame(&mut frame, &mut val),
            _ => (),
        }
    }
}
