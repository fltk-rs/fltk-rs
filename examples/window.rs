use fltk::button::Button;
use fltk::fl;
use fltk::widget::WidgetTrait;
use fltk::window::Window;

fn main() {
    let mut wind = Window::new().set(100, 100, 400, 300, "Hello from rust");
    wind.begin();
    let mut but = Button::new().set(150, 100, 80, 60, "Click me!");
    but.add_callback(|| match fl::event() {
        fl::Event::Release => println!("Works!"),
        _ => unimplemented!(),
    });
    wind.end();
    wind.show();
    fl::run();
}
