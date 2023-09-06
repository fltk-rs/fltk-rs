use fltk::{app, button::Button, frame::Frame, group::Flex, prelude::*, window::Window};

#[derive(Debug, Clone, Copy)]
pub enum Message {
    Increment,
    Decrement,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = app::App::default();
    let mut wind = Window::default().with_size(160, 200).with_label("Counter");
    let mut flex = Flex::default_fill().column();
    flex.set_margins(30, 40, 30, 40);
    flex.set_pad(10);
    let mut but_inc = Button::default().with_label("+");
    let mut frame = Frame::default().with_label("0");
    let mut but_dec = Button::default().with_label("-");
    flex.end();
    wind.end();
    wind.show();

    let (s, r) = app::channel::<Message>();

    but_inc.emit(s, Message::Increment);
    but_dec.emit(s, Message::Decrement);

    while app.wait() {
        let label: i32 = frame.label().parse()?;

        if let Some(msg) = r.recv() {
            match msg {
                Message::Increment => frame.set_label(&(label + 1).to_string()),
                Message::Decrement => frame.set_label(&(label - 1).to_string()),
            }
        }
    }
    Ok(())
}
