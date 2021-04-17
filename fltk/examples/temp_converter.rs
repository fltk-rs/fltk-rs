use fltk::{prelude::*, *};

fn main() {
    let app = app::App::default().with_scheme(app::Scheme::Gleam);
    let mut win = window::Window::default().with_size(150, 200);
    let mut pack = group::Pack::default().with_size(130, 180).center_of(&win);
    pack.set_spacing(5);
    frame::Frame::default()
        .with_size(0, 40)
        .with_label("Celcius")
        .with_align(enums::Align::Inside | enums::Align::Bottom);
    let mut inp1 = input::FloatInput::default().with_size(0, 40);
    frame::Frame::default()
        .with_size(0, 40)
        .with_label("Farenheit")
        .with_align(enums::Align::Inside | enums::Align::Bottom);
    let mut inp2 = input::FloatInput::default().with_size(0, 40);
    pack.end();
    win.end();
    win.show();

    inp1.set_value(&format!("{}", 0.0));
    inp2.set_value(&format!("{}", 32.0));

    inp1.set_trigger(enums::CallbackTrigger::Changed);
    inp2.set_trigger(enums::CallbackTrigger::Changed);

    let (s, r) = app::channel::<bool>();

    inp1.emit(s, true);
    inp2.emit(s, false);

    while app.wait() {
        let inp1_val: f64 = if inp1.value().is_empty() {
            0.0
        } else {
            inp1.value().parse().unwrap_or(0.0)
        };
        let inp2_val = if inp2.value().is_empty() {
            0.0
        } else {
            inp2.value().parse().unwrap_or(0.0)
        };

        if let Some(msg) = r.recv() {
            if msg {
                inp2.set_value(&format!("{:.4}", c_to_f(inp1_val)));
            } else {
                inp1.set_value(&format!("{:.4}", f_to_c(inp2_val)));
            }
        }
    }
}

fn c_to_f(val: f64) -> f64 {
    (val * 9.0 / 5.0) + 32.0
}

fn f_to_c(val: f64) -> f64 {
    (val - 32.0) * 5.0 / 9.0
}
