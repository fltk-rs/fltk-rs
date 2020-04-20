use fltk::{app, button::*, frame::*, window::*};

fn main() {
    let app = app::App::default();
    let main_window = MainWindow::default();
    main_window.draw_elements();
    app.run().unwrap();
}

#[derive(Debug, Clone)]
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
            frame: Frame::default()
                .with_pos(20, 80)
                .with_size(360, 160)
                .with_label(""),
        }
    }
    pub fn draw_elements(mut self) {
        self.wind.make_resizable(true);
        self.wind.end();
        self.wind.show();
        self.but1
            .clone()
            .handle(Box::new(|ev: app::Event| match ev {
                app::Event::Released => {
                    println!("{:?}", ev);
                    return true;
                }
                app::Event::Push => {
                    let mut out = String::from("");
                    println!("{:?}", ev);
                    let mut frame = self.frame.clone();
                    // Spawning a thread to allow for a responsive UI
                    std::thread::spawn(move || {
                        std::thread::sleep(std::time::Duration::from_millis(1000));
                        out = String::from("Hello");
                        frame.set_label(&out);
                    });
                    return true;
                }
                _ => {
                    return false;
                }
            }));
        app::set_callback(
            &mut self.but2.clone(),
            Box::new(|| match app::event() {
                app::Event::Released => {
                    match fltk::dialog::input("hello", "") {
                        Some(inp) => println!("{}", inp),
                        None => return,
                    }
                    println!("{:?}", app::event());
                    self.but1.set_label("No!");
                    self.but2.set_label("Works");
                }
                _ => println!("{:?}", app::event()),
            }),
        );
    }
}
