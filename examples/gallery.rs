use fltk::{
    app::{App, AppScheme},
    button::*,
    frame::*,
    group::*,
    input::*,
    menu::*,
    output::*,
    valuator::*,
    window::*,
};

fn draw_gallery() {
    let tab = Tabs::new(10, 10, 500 - 20, 400 - 20, "");
    
    let grp1 = Group::new(10, 35, 500 - 20, 400 - 45, "Tab1");
    
    let _but1 = Button::new(50, 60, 90, 25, "Button");
    
    let _but2 = RoundButton::new(50, 90, 90, 25, "Round");
    
    let _but3 = CheckButton::new(50, 120, 90, 25, "Check");
    
    let _but4 = LightButton::new(50, 150, 90, 25, "Light");
    
    let mut but5 = MenuButton::new(50, 180, 90, 25, "Menu");
    
    but5.add_choice("Hello");
    
    let _but6 = ReturnButton::new(50, 210, 90, 25, "Return");
    
    let mut chce = Choice::new(50, 240, 90, 30, "");
    
    chce.add_choice("Hello");
    
    let _slider = Slider::new(180, 60, 30, 210, "Slider");
    
    let _dial = Dial::new(360, 150, 90, 90, "Dial");
    
    let _scrl = Scrollbar::new(240, 60, 30, 210, "Scrollbar");
    
    let _rller = Roller::new(300, 60, 30, 210, "Roller");
    
    let _cnt = Counter::new(50, 330, 210, 30, "Counter");
    
    let _inp = Input::new(50, 270, 90, 25, "");
    
    let _out = Output::new(50, 300, 90, 25, "");
    
    let mut scrl2 = Scrollbar::new(180, 300, 150, 25, "");
    
    scrl2.set_type(ScrollBarType::HorizontalNiceScrollBar);
    
    let _frame = Frame::new(270, 335, 90, 30, "Examples!");
    
    grp1.end();
    
    let grp2 = Group::new(10, 35, 500 - 30, 400 - 25, "Tab2");
    
    grp2.end();
    
    tab.end();
}

fn main() {
    let app = App::default().set_scheme(AppScheme::Gtk);

    let (screen_width, screen_height) = fltk::app::screen_size();
    let mut wind = Window::new(
        (screen_width / 2.0 - 250.0) as i32,
        (screen_height / 2.0 - 200.0) as i32,
        500,
        400,
        "Gallery",
    );

    draw_gallery();

    wind.make_resizable(true);
    wind.end();
    wind.show();
    
    app.run().unwrap();
}
