use fltk::{button::*, frame::*, window::*};

fn main() {
    let app = fl::App::default();
    let main_window = MainWindow::default();
    main_window.draw_elements();
    app.run().unwrap();
}

#[derive(Debug, Copy, Clone)]
struct MainWindow {
    wind: Window,
    but1: Button,
    but2: Button,
    frame: Frame,
}

impl MainWindow {
    pub fn default() -> MainWindow {
        MainWindow {
            wind: Window::new(0, 0, 400, 300, "Hello from rust"),
            but1: Button::new(80, 30, 80, 30, "Click me!"),
            but2: Button::new(240, 30, 80, 30, "Click me!"),
            frame: Frame::new(20, 80, 360, 160, ""),
        }
    }
    pub fn draw_elements(mut self) {
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
                    return true;
                }
                fl::Event::Push => {
                    let mut out = String::from("");
                    println!("{:?}", ev);
                    // Spawning a thread to allow for a responsive UI
                    std::thread::spawn(move || {
                        std::thread::sleep(std::time::Duration::from_millis(1000));
                        out = String::from("Hello");
                        self.frame.set_label(&out);
                    });
                    return true;
                }
                _ => {
                    return false;
                }
            }));

        fl::set_callback(
            &self.but2.clone(),
            Box::new(|| match fl::event() {
                fl::Event::Released => {
                    match fltk::dialog::input("hello", "") {
                        Some(inp) => println!("{}", inp),
                        None => return,
                    }
                    println!("{:?}", fl::event());
                    self.but1.set_label("No!");
                    self.but2.set_label("Works");
                }
                _ => println!("{:?}", fl::event()),
            }),
        );
        self.wind.make_resizable(true);
        self.wind.show();
    }
}
