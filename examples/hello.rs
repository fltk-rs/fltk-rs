use fltk::{
    app::*,
    dialog::*,
    frame::*,
    enums::Font::*,
    enums::LabelType::*,
    enums::FrameType::*,
    window::Window,
};

fn main() {
    let app = App::default();
    let  mut window:Window = Window::new(0,0,340,180,"Problem");
    let mut frame:Frame = Frame::new(20,40,300,100, "Hello, World!");
    frame.set_frame(UpBox);
    frame.set_label_font(HelveticaBoldItalic);
    frame.set_label_size(36);
    frame.set_label_type(ShadowLabel);
    window.end();
    window.show();
    app.run().unwrap();
}