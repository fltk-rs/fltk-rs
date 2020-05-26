use fltk::{app, button::*, frame::*, window::*};

#[derive(Debug, Clone, Copy)]
pub enum Message {
    Increment,
    Decrement,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = app::App::default();
    let mut wind = Window::new(200, 200, 160, 200, "Counter");
    let mut frame = Frame::new(30, 80, 100, 40, "0");
    let mut but_inc = Button::new(30, 40, 100, 40, "+");
    let mut but_dec = Button::new(30, 120, 100, 40, "-");
    wind.end();
    wind.show();

    let (s, r) = app::channel::<Message>();

    but_inc.emit(s, Message::Increment);
    but_dec.emit(s, Message::Decrement);

    while app.wait()? {
        let label: i32 = frame.label().parse()?;
        
        match r.recv() {
            Some(Message::Increment) => frame.set_label(&(label + 1).to_string()),
            Some(Message::Decrement) => frame.set_label(&(label - 1).to_string()),
            None => (),
        }
    }
    Ok(())
}
