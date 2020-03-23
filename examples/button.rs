use fltk::{button::*, window::*};

fn main() {
    draw_elements();
}

fn draw_elements() {
    let mut wind = Window::new(0, 0, 400, 300, "Hello from rust");

    let mut but1 = Button::new(80, 80, 80, 60, "Click me!");
    let mut but2 = Button::new(240, 80, 80, 60, "Click me!");

    // but1.clone().set_callback(Box::new(|| match fl::event() {
    //     fl::Event::Released => {
    //         println!("{:?}", fl::event());
    //         but1.set_label("Works");
    //         but2.set_label("No!");
    //     }
    //     _ => println!("{:?}", fl::event()),
    // }));

    but1.clone().set_custom_handler(Box::new( |ev: Event| {
        match ev {
            fl::Event::Released => {
                println!("{:?}", ev);
                return 1;
            },
            fl::Event::Push => {
                println!("{:?}", ev);
                return 1;
            },
            _ => {
                return 0;
            },
        }
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
    fl::run().unwrap();
}
