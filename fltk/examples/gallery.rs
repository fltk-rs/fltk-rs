use fltk::{
    app, button::*, group::*, input::*, menu::*, misc::*, output::*, valuator::*, window::*,
};

fn draw_gallery() {
    let tab = Tabs::new(10, 10, 500 - 20, 500 - 20, "");

    let grp1 = Group::new(10, 35, 500 - 20, 500 - 45, "Tab1\t");

    let mut pack = Pack::new(15, 45, 150, 500 - 45, "");

    pack.set_spacing(10);

    let _but1 = Button::default().with_size(0, 30).with_label("Button");

    let _but2 = RoundButton::default().with_size(0, 30).with_label("Round");

    let _but3 = CheckButton::default().with_size(0, 30).with_label("Check");

    let _but4 = LightButton::default().with_size(0, 30).with_label("Light");

    let mut but5 = MenuButton::default().with_size(0, 30).with_label("Menu");

    but5.add_choice("Hello");

    let _but6 = ReturnButton::default()
        .with_size(0, 30)
        .with_label("Return");

    let mut chce = Choice::new(50, 240, 90, 30, "");

    chce.add_choice("Hello");

    let _inp = Input::default().with_size(0, 30).with_label("");

    let _out = Output::default().with_size(0, 30).with_label("");

    let _cnt = Counter::default().with_size(0, 30);

    let mut scrl = Scrollbar::default().with_size(0, 30);

    scrl.set_type(ScrollbarType::HorizontalNice);

    pack.end();

    let mut pack = Pack::new(200, 45, 150, 400, "");

    pack.set_type(PackType::Horizontal);

    pack.set_spacing(30);

    let mut slider = Slider::default().with_size(25, 0).with_label("Slider");

    slider.set_type(SliderType::VerticalNice);

    let _scrl2 = Scrollbar::default()
        .with_size(25, 0)
        .with_label("Scrollbar");

    let _rller = Roller::default().with_size(25, 0).with_label("Roller");

    pack.end();

    let _clock = Clock::new(380, 200, 90, 90, "Clock");

    let mut dial = FillDial::new(380, 50, 90, 90, "Dial");

    dial.set_selection_color(Color::Red);

    grp1.end();

    let grp2 = Group::new(10, 35, 500 - 30, 500 - 25, "Tab2\t");

    grp2.end();

    tab.end();
}

fn main() {
    let app = app::App::default().with_scheme(app::Scheme::Gtk);
    app::background(221, 221, 221);

    let mut wind = Window::default()
        .with_size(500, 500)
        .with_label("Gallery")
        .center_screen();

    draw_gallery();

    wind.make_resizable(true);
    wind.end();
    wind.show();

    app.run().unwrap();
}
