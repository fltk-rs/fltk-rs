use fltk::*;

fn main() {
    let app = app::App::default();
    let mut wind = window::Window::default().with_size(400, 300);
    let mut frame = frame::Frame::default()
        .size_of(&wind)
        .with_label("Right click me!");
    wind.end();
    wind.show();
    let mut menu = menu::MenuItem::new(&["1st menu item\t", "2nd menu item\t", "3rd menu item\t"]);
    frame.handle(move |ev| match ev {
        Event::Push => {
            if app::event_mouse_button() == Mouse::Right {
                // or app::event_button() == 3
                let coords = app::event_coords();
                match menu.popup(coords.0, coords.1) {
                    None => println!("No value was chosen!"),
                    Some(val) => println!("{}", val.label().unwrap()),
                }
            }
            true
        }
        _ => false,
    });

    app.run().unwrap();
}
