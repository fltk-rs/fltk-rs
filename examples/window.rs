use fltk::{button::Button, fl, widget::WidgetTrait, window::Window};

fn main() {
    let mut wind = Window::new().set(100, 100, 400, 300, "Hello from rust");
    wind.begin();
    let mut but = Button::new().set(150, 100, 80, 60, "Click me!");
    but.add_callback(|| match fl::event() {
        fl::Event::Clicked => println!("Works!"),
        _ => unimplemented!(),
    });
    // but.add_callback_with_captures(&mut || match fl::event() {
    //     fl::Event::Clicked => but.set_label("Doesn't work!"),
    //     _ => unimplemented!(), 
    // });
    wind.end();
    wind.show();
    fl::run();
}
