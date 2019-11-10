use fltk::{button::*, window::*};

fn main() {
    draw_elements();
}

fn draw_elements() {
    let mut wind = Window::new(0, 0, 400, 300, "Hello from rust");

    let mut but1 = Button::new(80, 80, 80, 60, "Click me!");
    let mut but2 = Button::new(240, 80, 80, 60, "Click me!");
    let mut but3 = Button::new(160, 160, 80, 60, "Click me!");

    fl::set_callback(&but1.clone(), &mut || match fl::event() {
        fl::Event::Released => {
            println!("{:?}", fl::event());
            but1.set_label("Works");
            but2.set_label("No!");
        }
        _ => println!("{:?}", fl::event()),
    });

    fl::set_callback(&but2.clone(), &mut || match fl::event() {
        fl::Event::Released => {
            println!("{:?}", fl::event());
            but1.set_label("No!");
            but2.set_label("Works");
        }
        _ => println!("{:?}", fl::event()),
    });

    but3.set_callback(&mut || {
        println!("event: {:?}", fl::event());
        println!("event_button: {:?}", fl::event_button());
        println!("event_clicks: {:?}", fl::event_clicks());
        println!("event_is_click: {:?}", fl::event_is_click());
    });

    wind.end();
    wind.show();
    fl::run();
}
