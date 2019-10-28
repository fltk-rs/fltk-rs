use fltk::{button::Button, fl, widget, widget::WidgetTrait, window::Window};

fn main() {
    let mut wind = Window::new().set(100, 100, 400, 300, "Hello from rust");
    wind.begin();
    let but = Button::new().set(150, 100, 80, 60, "Click me!");
    widget::register_callback(&but, &mut || match fl::event() {
        fl::Event::Released => {
            println!("{:?}", fl::event());
            wind.set_label("works");
        }
        _ => println!("{:?}", fl::event()),
    });
    wind.end();
    wind.show();
    fl::run();
}
