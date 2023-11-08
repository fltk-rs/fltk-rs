use fltk::{prelude::*, *};

fn main() {
    let app = app::App::default();

    let mut window = window::Window::default().with_size(200, 200);
    let mut grp = group::Group::default_fill();
    button::Button::default_fill();
    grp.end();
    window.end();
    window.show();

    let mut btn = button::Button::from_dyn_widget(&grp.child(0).unwrap()).unwrap();
    btn.handle(move |w, ev| {
        if ev == enums::Event::Push {
            grp.clear();
            grp.begin();
            let b = button::Button::new(0, 0, 100, 100, "None");
            grp.end();
            true
        } else {
            false
        }
    });

    app.run().unwrap();
}
