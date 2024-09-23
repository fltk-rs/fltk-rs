use fltk::{enums::*, prelude::*, *};

fn main() {
    let app = app::App::default();
    // global theming
    app::background(55, 55, 55); // background color.
                                 // For input/output and text widgets, use app::background2
    app::background2(0, 0, 0);
    app::foreground(255, 255, 255); // labels
    app::set_font_size(16);
    app::set_frame_type2(FrameType::UpBox, FrameType::RFlatBox);
    app::set_frame_type2(FrameType::DownBox, FrameType::RFlatBox);
    app::set_frame_border_radius_max(15); // set the roundness of the RFlatBox
    app::set_font(Font::Times);
    app::set_visible_focus(false);

    // regular widget code
    let mut win = window::Window::default().with_size(400, 300);
    let mut flex = group::Flex::default_fill().column();
    flex.set_margins(100, 60, 100, 60);
    flex.set_pad(10);
    let mut btn1 = button::Button::default().with_label("Increment");
    btn1.set_color(Color::Red.darker());
    btn1.set_selection_color(Color::Red.darker().darker());
    let mut out = frame::Frame::default().with_label("0");
    out.set_color(Color::Green.darker());
    let mut btn2 = button::Button::default().with_label("Decrement");
    btn2.set_color(Color::Red.darker());
    btn2.set_selection_color(Color::Red.darker().darker());
    flex.end();
    win.end();
    win.show();

    app.run().unwrap();
}
