use fltk::{button::*, window::*};

fn main() {
    draw_elements();
}

fn draw_elements() {
    let mut wind = Window::new(0, 0, 400, 300, "Hello from rust");

    let mut but1 = Button::new(80, 80, 80, 60, "Click me!");
    let mut but2 = Button::new(240, 80, 80, 60, "Click me!");

    but1.clone().set_callback(Box::new(|| match fl::event() {
        fl::Event::Released => {
            println!("{:?}", fl::event());
            but1.set_label("Works");
            but2.set_label("No!");
        }
        _ => println!("{:?}", fl::event()),
    }));

    fl::set_callback(
        &but2.clone(),
        Box::new(|| match fl::event() {
            fl::Event::Released => {
                println!("{:?}", fl::event());
                but1.set_label("No!");
                but2.set_label("Works");
            }
            _ => println!("{:?}", fl::event()),
        }),
    );

    wind.show();
    fl::run();
}
