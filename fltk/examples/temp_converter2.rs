use fltk::{enums::*, prelude::*, *};

fn main() {
    MyApp::new().run()
}

#[derive(Copy, Clone)]
pub enum Message {
    CelciusChanged,
    FahrenheitChanged,
}

fn make_label(label: &str) -> frame::Frame {
    let mut f = frame::Frame::default()
        .with_label(label)
        .with_align(Align::Inside | Align::Bottom);
    f.set_label_type(LabelType::Engraved);
    f
}

fn c_to_f(val: f64) -> f64 {
    (val * 9.0 / 5.0) + 32.0
}

fn f_to_c(val: f64) -> f64 {
    (val - 32.0) * 5.0 / 9.0
}

struct MyApp {
    a: app::App,
    inp1: input::FloatInput,
    inp2: input::FloatInput,
    r: app::Receiver<Message>,
}

impl MyApp {
    pub fn new() -> Self {
        let a = app::App::default();
        MyApp::init_styles();
        let (s, r) = app::channel();

        let (inp1, inp2) = {
            let mut win = window::Window::default().with_size(150, 200);
            let mut flex = group::Flex::default()
                .with_size(130, 180)
                .center_of(&win)
                .column();
            make_label("Celcius");
            let mut inp1 = input::FloatInput::default().with_size(0, 40);
            make_label("Fahrenheit");
            let mut inp2 = input::FloatInput::default().with_size(0, 40);
            flex.fixed(&inp1, 30);
            flex.fixed(&inp2, 30);
            flex.end();
            win.end();
            win.make_resizable(true);
            win.show();

            inp1.set_value(&format!("{}", 0.0));
            inp2.set_value(&format!("{}", 32.0));

            inp1.set_trigger(CallbackTrigger::Changed);
            inp2.set_trigger(CallbackTrigger::Changed);

            inp1.emit(s, Message::CelciusChanged);
            inp2.emit(s, Message::FahrenheitChanged);

            (inp1, inp2)
        };
        Self { a, inp1, inp2, r }
    }

    fn init_styles() {
        app::set_scheme(app::Scheme::Gleam);
        app::set_background_color(170, 189, 206);
        app::set_background2_color(255, 255, 255);
        app::set_foreground_color(0, 0, 0);
        app::set_selection_color(255, 160, 63);
        app::set_inactive_color(130, 149, 166);
        app::set_font_size(16);
    }

    pub fn run(&mut self) {
        while self.a.wait() {
            if let Some(msg) = self.r.recv() {
                match msg {
                    Message::CelciusChanged => {
                        self.inp2.set_value(&format!(
                            "{:.4}",
                            c_to_f(self.inp1.value().parse().unwrap_or(0.0))
                        ));
                    }
                    Message::FahrenheitChanged => {
                        self.inp1.set_value(&format!(
                            "{:.4}",
                            f_to_c(self.inp2.value().parse().unwrap_or(0.0))
                        ));
                    }
                }
            }
        }
    }
}
