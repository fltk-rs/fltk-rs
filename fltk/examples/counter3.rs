use fltk::{
    enums::{Align, Color, Font, FrameType},
    prelude::*,
    *,
};

const BLUE: Color = Color::from_hex(0x42A5F5);
const SEL_BLUE: Color = Color::from_hex(0x3f51b5);
const GRAY: Color = Color::from_hex(0x757575);
const WIDTH: i32 = 600;
const HEIGHT: i32 = 400;

fn main() {
    let app = app::App::default();
    let mut win = window::Window::default()
        .with_size(WIDTH, HEIGHT)
        .with_label("Flutter-like!");
    let mut col = group::Flex::default_fill().column();
    let mut bar = frame::Frame::default()
        .with_label("  FLTK App!")
        .with_align(Align::Left | Align::Inside);
    col.fixed(&bar, 60);
    let mut text = frame::Frame::default()
        .with_label("You have pushed the button this many times:")
        .with_align(Align::Bottom | Align::Inside);
    let mut count = frame::Frame::default()
        .with_label("0")
        .with_align(Align::Top | Align::Inside);
    let mut row = group::Flex::default();
    col.fixed(&row, 60);
    frame::Frame::default();
    let mut but = button::Button::default().with_label("@+6plus");
    row.fixed(&but, 60);
    let spacing = frame::Frame::default();
    row.fixed(&spacing, 20);
    row.end();
    let spacing = frame::Frame::default();
    col.fixed(&spacing, 20);
    col.end();
    win.end();
    win.make_resizable(true);
    win.show();

    // Theming
    app::background(255, 255, 255);
    app::set_visible_focus(false);

    bar.set_frame(FrameType::FlatBox);
    bar.set_label_size(22);
    bar.set_label_color(Color::White);
    bar.set_color(BLUE);
    bar.draw(|b| {
        draw::set_draw_rgb_color(211, 211, 211);
        draw::draw_rectf(0, b.height(), b.width(), 3);
    });

    text.set_label_size(18);
    text.set_label_font(Font::Times);

    count.set_label_size(36);
    count.set_label_color(GRAY);

    but.set_color(BLUE);
    but.set_selection_color(SEL_BLUE);
    but.set_label_color(Color::White);
    but.set_frame(FrameType::OFlatFrame);
    // End theming

    but.set_callback(move |_| {
        let label = (count.label().parse::<i32>().unwrap() + 1).to_string();
        count.set_label(&label);
    });

    app.run().unwrap();
}
