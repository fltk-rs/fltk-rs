use fltk::*;

fn main() {
    let app = app::App::default().with_scheme(app::AppScheme::Gleam);
    let mut win = window::Window::new(200, 200, 150, 200, "");
    let pack = group::Pack::new(0, 0, 150, 250, "");
    let _frame1 = frame::Frame::new(0, 0, 0, 50, "Celcius");
    let mut inp1 = input::FloatInput::new(0, 0, 50, 50, "");
    let _frame2 = frame::Frame::new(0, 0, 0, 50, "Farenheit");
    let mut inp2 = input::FloatInput::new(0, 0, 50, 50, "");
    pack.end();
    win.end();
    win.show();

    inp1.set_value(&format!("{}", 0.0));
    inp2.set_value(&format!("{}", 32.0));

    inp1.set_trigger(CallbackTrigger::Changed);
    inp2.set_trigger(CallbackTrigger::Changed);

    let (s, r) = app::channel::<bool>();

    inp1.emit(s, true);
    inp2.emit(s, false);

    while app.wait().unwrap() {
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

        match r.recv() {
            Some(msg) => {
                if msg {
                    inp2.set_value(&format!("{:.4}", c_to_f(inp1_val)));
                } else {
                    inp1.set_value(&format!("{:.4}", f_to_c(inp2_val)));
                }
            }
            None => (),
        }
        std::thread::sleep(std::time::Duration::from_millis(16));
    }
}

fn c_to_f(val: f64) -> f64 {
    (val * 9.0 / 5.0) + 32.0
}

fn f_to_c(val: f64) -> f64 {
    (val - 32.0) * 5.0 / 9.0
}
