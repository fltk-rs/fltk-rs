use fltk::{prelude::*, *};

fn main() {
    let app = app::App::default().with_scheme(app::Scheme::Gtk);
    let mut win = window::Window::new(100, 100, 800, 600, "Charts");
    let mut chart = misc::Chart::default().size_of_parent();
    chart.set_type(misc::ChartType::Pie);
    chart.set_bounds(0.0, 100.0);
    chart.set_text_size(18);
    chart.add(88.4, "Rust", enums::Color::from_u32(0xcc9c59));
    chart.add(8.4, "C++", enums::Color::Red);
    chart.add(3.2, "C", enums::Color::Black);
    chart.set_color(enums::Color::White);
    let mut choice = menu::Choice::new(300, 5, 200, 40, "Chart type");
    choice.add_choice("Bar | HorzBar | Line | Fill | Spike | Pie | SpecialPie");
    choice.set_value(5);
    choice.set_color(enums::Color::White);
    win.end();
    win.show();

    choice.set_callback(move |c| {
        chart.set_type(misc::ChartType::from_i32(c.value()));
        chart.redraw();
    });

    app.run().unwrap();
}
