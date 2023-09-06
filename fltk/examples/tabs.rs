use fltk::{
    app,
    button::*,
    group::{Flex, Tabs},
    input::Input,
    menu::{Choice, MenuButton},
    output::Output,
    prelude::{GroupExt, InputExt, MenuExt, WidgetBase, WidgetExt, WindowExt},
    window::Window,
};

fn draw_gallery() {
    let mut tab = Tabs::default_fill();

    let mut grp1 = Flex::default_fill().with_label("Tab1\t\t").row();
    let mut col = Flex::default().column();
    grp1.fixed(&col, 160);
    col.set_pad(10);
    col.set_margin(10);
    let _but1 = Button::default().with_label("Button");
    let _but2 = RoundButton::default().with_label("Round");
    let _but3 = CheckButton::default().with_label("Check");
    let _but4 = LightButton::default().with_label("Light");
    let mut but5 = MenuButton::default().with_label("Menu");
    but5.add_choice("Hello|World|From|Rust");
    let _but6 = ReturnButton::default().with_label("Return");
    let mut chce = Choice::default();
    chce.add_choice("Hello");
    let _inp = Input::default();
    let mut out = Output::default();
    out.set_value("output");
    col.end();
    grp1.end();

    let grp2 = Flex::default_fill().with_label("Tab2\t\t").row();
    grp2.end();
    tab.end();
    tab.auto_layout();
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
