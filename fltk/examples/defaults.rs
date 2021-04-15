use fltk::{prelude::*, *};

fn main() {
    let (r, g, b) = utils::hex2rgb(0xfafdf3);

    let app = app::App::default();

    // global theming
    app::background(r, g, b);
    app::set_font(enums::Font::Courier);
    app::set_font_size(16);
    app::set_frame_type(enums::FrameType::RFlatBox);
    app::set_visible_focus(false);

    // regular widget code
    let mut win = window::Window::default().with_size(400, 300);
    let frame = frame::Frame::new(0, 0, 400, 200, "Defaults");
    let mut pack = group::Pack::default()
        .with_size(400, 50)
        .below_of(&frame, 50);
    pack.set_type(group::PackType::Horizontal);
    pack.set_spacing(80);
    let mut but1 = button::Button::default().with_label("Button1");
    but1.set_color(enums::Color::Yellow);
    but1.set_down_frame(enums::FrameType::RFlatBox);
    let mut but2 = button::Button::default().with_label("Button2");
    but2.set_color(enums::Color::Yellow);
    but2.set_down_frame(enums::FrameType::RFlatBox);
    pack.end();
    pack.auto_layout();

    win.end();
    win.show();

    app.run().unwrap();
}
