use fltk::*;

fn main() {
    let app = app::App::default();
    let mut win = window::Window::default().with_size(400, 300);

    let mut grid = group::VGrid::new(0, 0, 400, 300, "");
    grid.set_params(3, 3, 5);

    grid.add(&button::Button::default());
    grid.add(&button::Button::default());
    grid.add(&button::Button::default());
    grid.add(&button::Button::default());
    grid.add(&button::Button::default());
    grid.add(&button::Button::default());
    grid.add(&button::Button::default());
    grid.add(&button::Button::default());

    win.end();
    win.show();

    app.run().unwrap();
}
