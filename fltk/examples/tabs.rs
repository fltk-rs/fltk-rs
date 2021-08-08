use fltk::{
    app,
    button::*,
    group::{Group, Pack, Tabs},
    input::Input,
    menu::{Choice, MenuButton},
    output::Output,
    prelude::{GroupExt, MenuExt, WidgetBase, WidgetExt, WindowExt},
    window::Window,
};

fn draw_gallery() {
    let tab = Tabs::new(10, 10, 500 - 20, 450 - 20, "");

    let grp1 = Group::new(10, 35, 500 - 20, 450 - 45, "Tab1\t\t");

    let mut pack = Pack::new(15, 45, 150, 450 - 45, "");
    pack.set_spacing(10);
    let _but1 = Button::default().with_size(0, 30).with_label("Button");
    let _but2 = RoundButton::default().with_size(0, 30).with_label("Round");
    let _but3 = CheckButton::default().with_size(0, 30).with_label("Check");
    let _but4 = LightButton::default().with_size(0, 30).with_label("Light");
    let mut but5 = MenuButton::default().with_size(0, 30).with_label("Menu");
    but5.add_choice("Hello|World|From|Rust");
    let _but6 = ReturnButton::default()
        .with_size(0, 30)
        .with_label("Return");
    let mut chce = Choice::new(50, 240, 90, 30, "");
    chce.add_choice("Hello");
    let _inp = Input::default().with_size(0, 30).with_label("");
    let _out = Output::default().with_size(0, 30).with_label("");

    pack.end();
    grp1.end();

    let grp2 = Group::new(10, 35, 500 - 30, 450 - 25, "Tab2\t\t");
    grp2.end();
    tab.end();
}

fn main() {
    let app = app::App::default().with_scheme(app::Scheme::Gtk);
    app::background(221, 221, 221);

    let mut wind = Window::default()
        .with_size(500, 450)
        .with_label("Tabs")
        .center_screen();

    draw_gallery();

    wind.make_resizable(true);
    wind.end();
    wind.show();

    app.run().unwrap();
}
