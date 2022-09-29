use fltk::{
    app,
    button::Button,
    enums::{Color, FrameType},
    frame::Frame,
    prelude::*,
    window::Window,
};

fn main() {
    let app = app::App::default().with_scheme(app::Scheme::Gtk);
    app::background(0x62, 0x00, 0xee);
    app::set_visible_focus(false);

    let mut wind = Window::default().with_size(160, 200).with_label("Counter");
    let mut frame = Frame::default()
        .with_size(100, 40)
        .center_of(&wind)
        .with_label("0");
    frame.set_label_size(20);
    let mut but_inc = Button::default()
        .size_of(&frame)
        .above_of(&frame, 0)
        .with_label("+");
    let mut but_dec = Button::default()
        .size_of(&frame)
        .below_of(&frame, 0)
        .with_label("-");
    // wind.make_resizable(true);
    wind.end();
    wind.show();

    but_inc.set_callback({
        let mut frame = frame.clone();
        move |_| {
            let label = (frame.label().parse::<i32>().unwrap() + 1).to_string();
            frame.set_label(&label);
        }
    });

    but_dec.set_callback(move |_| {
        let label = (frame.label().parse::<i32>().unwrap() - 1).to_string();
        frame.set_label(&label);
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
