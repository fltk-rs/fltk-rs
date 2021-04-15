use fltk::{
    app, button, group,
    prelude::{GroupExt, WidgetExt},
    window,
};

pub fn show_wizard(_but: &mut button::Button) {
    let mut win = window::Window::default()
        .with_size(400, 300)
        .with_label("Wizard");
    let mut wizard = group::Wizard::default().with_size(400, 300);
    let grp1 = group::Group::default().size_of(&wizard);
    let mut but1 = button::Button::default()
        .with_size(80, 40)
        .center_of(&wizard)
        .with_label("next");
    grp1.end();
    let grp2 = group::Group::default().size_of(&wizard);
    let mut pack = group::Pack::default().with_size(160, 40).center_of(&grp2);
    pack.set_type(group::PackType::Horizontal);
    let mut but2 = button::Button::default()
        .with_size(80, 0)
        .center_of(&wizard)
        .with_label("previous");
    let mut but3 = button::Button::default()
        .with_size(80, 0)
        .center_of(&wizard)
        .with_label("next");
    pack.end();
    grp2.end();
    let grp3 = group::Group::default().size_of(&wizard);
    let mut but4 = button::Button::default()
        .with_size(80, 40)
        .center_of(&wizard)
        .with_label("previous");
    grp3.end();
    wizard.end();
    win.end();
    win.show();
    but1.set_callback({
        let mut wiz_c = wizard.clone();
        move |_| wiz_c.next()
    });
    but2.set_callback({
        let mut wiz_c = wizard.clone();
        move |_| wiz_c.prev()
    });
    but3.set_callback({
        let mut wiz_c = wizard.clone();
        move |_| wiz_c.next()
    });
    but4.set_callback(move |_| wizard.prev());
}

fn main() {
    let app = app::App::default().with_scheme(app::Scheme::Gtk);
    let mut win = window::Window::default().with_size(400, 300);
    let _but = {
        let mut b = button::Button::default()
            .with_size(160, 40)
            .with_label("Show wizard")
            .center_of(&win);
        b.set_callback(show_wizard);
        b
    };
    win.end();
    win.show();
    app.run().unwrap();
}
