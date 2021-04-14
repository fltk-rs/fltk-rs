use fltk::*;

const BLUE: u32 = 0x42A5F5;
const SEL_BLUE: u32 = 0x2196F3;
const GRAY: u32 = 0x757575;
const WIDTH: i32 = 600;
const HEIGHT: i32 = 400;

fn main() {
    let app = app::App::default();
    let mut win = window::Window::default()
        .with_size(WIDTH, HEIGHT)
        .with_label("Flutter-like!");
    let mut bar =
        frame::Frame::new(0, 0, WIDTH, 60, "  FLTK App!").with_align(Align::Left | Align::Inside);
    let mut text = frame::Frame::default()
        .with_size(100, 40)
        .center_of(&win)
        .with_label("You have pushed the button this many times:");
    let mut count = frame::Frame::default()
        .size_of(&text)
        .below_of(&text, 0)
        .with_label("0");
    let mut but = button::Button::new(WIDTH - 100, HEIGHT - 100, 60, 60, "@+6plus");
    win.end();
    win.make_resizable(true);
    win.show();

    // Theming
    app::background(255, 255, 255);
    app::set_visible_focus(false);

    bar.set_frame(FrameType::FlatBox);
    bar.set_label_size(22);
    bar.set_label_color(Color::White);
    bar.set_color(Color::from_u32(BLUE));
    bar.draw(|b| {
        draw::set_draw_rgb_color(211, 211, 211);
        draw::draw_rectf(0, b.height(), b.width(), 3);
    });

    text.set_label_size(18);
    text.set_label_font(Font::Times);

    count.set_label_size(36);
    count.set_label_color(Color::from_u32(GRAY));

    but.set_color(Color::from_u32(BLUE));
    but.set_selection_color(Color::from_u32(SEL_BLUE));
    but.set_label_color(Color::White);
    but.set_frame(FrameType::OFlatFrame);
    // End theming

    but.set_callback(move |_| {
        let label = (count.label().parse::<i32>().unwrap() + 1).to_string();
        count.set_label(&label);
    });

    app.run().unwrap();
}
