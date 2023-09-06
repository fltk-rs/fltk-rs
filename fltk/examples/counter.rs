use fltk::{
    app,
    button::Button,
    enums::{Color, FrameType},
    frame::Frame,
    group::Flex,
    prelude::*,
    window::Window,
};
use std::{cell::RefCell, rc::Rc};

fn main() {
    let app = app::App::default().with_scheme(app::Scheme::Gtk);
    app::background(0x62, 0x00, 0xee);
    app::set_visible_focus(false);

    let count = Rc::new(RefCell::new(0));

    let mut wind = Window::default().with_size(160, 200).with_label("Counter");
    let mut flex = Flex::default_fill().column();
    flex.set_margins(30, 40, 30, 40);
    flex.set_pad(10);
    let mut but_inc = Button::default().with_label("+");
    let mut frame = Frame::default().with_label(&count.borrow().to_string());
    let mut but_dec = Button::default().with_label("-");
    flex.end();
    // wind.make_resizable(true);
    wind.end();
    wind.show();

    but_inc.set_callback({
        let count = count.clone();
        let mut frame = frame.clone();
        move |_| {
            *count.borrow_mut() += 1;
            frame.set_label(&count.borrow().to_string());
        }
    });

    but_dec.set_callback(move |_| {
        *count.borrow_mut() -= 1;
        frame.set_label(&count.borrow().to_string());
    });

    // Theming
    wind.set_color(Color::White);
    but_inc.set_color(Color::from_u32(0x304FFE));
    but_inc.set_selection_color(Color::Green);
    but_inc.set_label_size(20);
    but_inc.set_frame(FrameType::FlatBox);
    but_inc.set_label_color(Color::White);
    but_dec.set_color(Color::from_u32(0x2962FF));
    but_dec.set_selection_color(Color::Red);
    but_dec.set_frame(FrameType::FlatBox);
    but_dec.set_label_size(20);
    but_dec.set_label_color(Color::White);
    // End theming

    app.run().unwrap();
}
