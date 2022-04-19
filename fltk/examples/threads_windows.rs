use fltk::{
    app, button::Button, enums::Align, frame::Frame, group::Pack, misc::HelpView, prelude::*,
    window::Window,
};

use std::{thread, time::Duration};

#[derive(Debug, Clone, Copy)]
pub enum Message {
    Increment,
    Decrement,
    Activate,
    Deactivate,
    Message(&'static str),
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = app::App::default();
    let wind = Window::default().with_size(160, 200).with_label("Counter");
    let pack = Pack::default().with_size(120, 140).center_of(&wind);
    pack.set_spacing(10);
    let but_inc = Button::default().with_size(0, 40).with_label("+");
    let frame = Frame::default().with_size(0, 40).with_label("0");
    let but_dec = Button::default().with_size(0, 40).with_label("-");
    pack.end();
    wind.end();
    wind.show();

    let msg_wind = Window::default().with_size(120, 100).with_label("Message");
    let msgview = HelpView::default().with_size(120, 100);
    msgview.set_align(Align::Center);
    msg_wind.end();

    let (s, r) = app::channel::<Message>();

    but_inc.set_callback({
        move |_| {
            s.send(Message::Deactivate);
            thread::spawn(move || {
                thread::sleep(Duration::from_secs(1));
                s.send(Message::Increment);
                s.send(Message::Message("Incremented"));
                s.send(Message::Activate);
            });
        }
    });
    but_dec.set_callback({
        move |_| {
            s.send(Message::Deactivate);
            thread::spawn(move || {
                thread::sleep(Duration::from_secs(1));
                s.send(Message::Decrement);
                s.send(Message::Message("Decremented"));
                s.send(Message::Activate);
            });
        }
    });

    while app.wait() {
        if let Some(msg) = r.recv() {
            let label: i32 = frame.label().unwrap().parse()?;
            match msg {
                Message::Increment => frame.set_label(&(label + 1).to_string()),
                Message::Decrement => frame.set_label(&(label - 1).to_string()),
                Message::Activate => {
                    but_inc.activate();
                    but_dec.activate();
                }
                Message::Deactivate => {
                    but_inc.deactivate();
                    but_dec.deactivate();
                }
                Message::Message(s) => {
                    msgview.set_value(s);
                    msg_wind.show();
                }
            }
        }
    }
    Ok(())
}
