use fltk::{prelude::*, *};

fn main() {
    let app = app::App::default().with_scheme(app::Scheme::Gleam);
    let mut win = window::Window::default().with_size(150, 200);
    let mut col = group::Flex::default()
        .column()
        .with_size(130, 180)
        .center_of(&win);
    col.set_pad(5);
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
    col.end();
    win.end();
    win.show();

    inp1.set_value(&format!("{}", 0.0));
    inp2.set_value(&format!("{}", 32.0));

    inp1.set_trigger(enums::CallbackTrigger::Changed);
    inp2.set_trigger(enums::CallbackTrigger::Changed);

    inp1.set_callback({
        let mut inp2 = inp2.clone();
        move |i| {
            let inp1_val: f64 = if i.value().is_empty() {
                0.0
            } else {
                i.value().parse().unwrap_or(0.0)
            };
            inp2.set_value(&format!("{:.4}", c_to_f(inp1_val)));
        }
    });
    inp2.set_callback(move |i| {
        let inp2_val: f64 = if i.value().is_empty() {
            0.0
        } else {
            i.value().parse().unwrap_or(0.0)
        };
        inp1.set_value(&format!("{:.4}", f_to_c(inp2_val)));
    });

    app.run().unwrap();
}

fn c_to_f(val: f64) -> f64 {
    (val * 9.0 / 5.0) + 32.0
}

fn f_to_c(val: f64) -> f64 {
    (val - 32.0) * 5.0 / 9.0
}
