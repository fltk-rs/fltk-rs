use fltk::{prelude::*, *};

fn main() {
    let (r, g, b) = utils::hex2rgb(0xfafdf3);

    let app = app::App::default();

    // global theming
    app::background(r, g, b); // background color. For input/output and text widgets, use app::background2
    app::foreground(20, 20, 20); // labels
    app::set_font(enums::Font::Courier);
    app::set_font_size(16);
    app::set_frame_type(enums::FrameType::RFlatBox);
    app::set_visible_focus(false);

    // regular widget code
    let mut win = window::Window::default().with_size(400, 300);
    let frame = frame::Frame::new(0, 0, 400, 200, "Defaults");
    let flex = group::Flex::default()
        .with_size(400, 50)
        .below_of(&frame, 50);
    let mut but1 = button::Button::default().with_label("Button1");
    but1.set_color(enums::Color::Yellow);
    but1.set_down_frame(enums::FrameType::RFlatBox);
    frame::Frame::default();
    let mut but2 = button::Button::default().with_label("Button2");
    but2.set_color(enums::Color::Yellow);
    but2.set_down_frame(enums::FrameType::RFlatBox);
    flex.end();

    win.end();
    win.show();

    app.run().unwrap();
}
