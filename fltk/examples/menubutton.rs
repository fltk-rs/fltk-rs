use fltk::{enums::*, prelude::*, *};

fn main() {
    let app = app::App::default();
    let mut win = window::Window::default().with_size(400, 300);
    let mut frame = frame::Frame::default()
        .with_size(200, 100)
        .with_label("Right click me!")
        .center_of_parent();
    frame.set_frame(FrameType::BorderFrame);
    frame.set_color(Color::Red);
    let mut menu = menu::MenuButton::default()
        .size_of(&frame)
        .center_of(&frame)
        .with_type(menu::MenuButtonType::Popup3);
    menu.add_choice("1st menu item\t|2nd menu item\t|3rd menu item\t");
    menu.set_callback(|m| println!("{:?}", m.choice()));
    win.end();
    win.show();

    app.run().unwrap();
}
