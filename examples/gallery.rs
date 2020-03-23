use fltk::{button::*, group::*, input::*, output::*, valuator::*, window::*};

fn draw_gallery() {
    let tab = Tabs::new(10, 10, 500 - 20, 200 - 20, "");
    let grp1 = Group::new(10, 35, 500 - 20, 200 - 45, "Buttons");
    let _but1 = Button::new(50, 60, 90, 25, "Button");
    let _but2 = RoundButton::new(50, 90, 90, 25, "Round");
    let _but3 = CheckButton::new(50, 120, 90, 25, "Check");
    grp1.end();
    let grp2 = Group::new(10, 35, 500 - 10, 200 - 35, "Valuators");
    let _slider = Slider::new(50, 60, 30, 90, "Slider");
    let _dial = Dial::new(100, 60, 90, 90, "Dial");
    grp2.end();
    let grp3 = Group::new(10, 35, 500 - 30, 200 - 25, "Input");
    let _inp = Input::new(50, 60, 90, 30, "");
    let _out = Output::new(150, 60, 90, 30, "");
    grp3.end();
    tab.end();
}

fn main() {
    let mut wind = Window::new(100, 100, 500, 200, "Tabs Example");
    draw_gallery();
    wind.show();
    fl::run().unwrap();
}
