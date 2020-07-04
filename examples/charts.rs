use fltk::*;

fn main() {
    let app = app::App::default().with_scheme(app::AppScheme::Gtk);
    let mut win = window::Window::new(100, 100, 800, 600, "Charts");
    let mut chart = misc::Chart::new(0, 0, 800, 600, "");
    chart.set_type(ChartType::Pie);
    chart.set_bounds(0.0, 100.0);
    chart.set_text_size(18);
    chart.add(88.4, "Rust", Color::from_u32(0xcc9c59));
    chart.add(8.4, "C++", Color::Red);
    chart.add(3.2, "C", Color::Black);
    chart.set_color(Color::White);
    win.end();
    win.show();
    app.run().unwrap();
}
