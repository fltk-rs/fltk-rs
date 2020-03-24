use fltk::{button::*, window::*};

fn main() {
    let mut main_window = MainWindow::default();
    main_window.draw_elements();
    fl::run().unwrap();
}

struct MainWindow {
    wind: Window,
    but1: Button,
    but2: Button,
}

impl MainWindow {
    pub fn default() -> MainWindow {
        MainWindow {
            wind: Window::new(0, 0, 400, 300, "Hello from rust"),
            but1: Button::new(80, 80, 80, 60, "Click me!"),
            but2: Button::new(240, 80, 80, 60, "Click me!"),
        }
    }
    pub fn draw_elements(&mut self) {
        // Different ways of handling events

        // but1.clone().set_callback(Box::new(|| match fl::event() {
        //     fl::Event::Released => {
        //         println!("{:?}", fl::event());
        //         but1.set_label("Works");
        //         but2.set_label("No!");
        //     }
        //     _ => println!("{:?}", fl::event()),
        // }));

        self.but1.clone()
            .set_custom_handler(Box::new(|ev: Event| match ev {
                fl::Event::Released => {
                    println!("{:?}", ev);
                    return 1;
                }
                fl::Event::Push => {
                    println!("{:?}", ev);
                    return 1;
                }
                _ => {
                    return 0;
                }
            }));

        fl::set_callback(
            &self.but2.clone(),
            Box::new(|| match fl::event() {
                fl::Event::Released => {
                    println!("{:?}", fl::event());
                    self.but1.set_label("No!");
                    self.but2.set_label("Works");
                }
                _ => println!("{:?}", fl::event()),
            }),
        );
        self.wind.show();
    }
}