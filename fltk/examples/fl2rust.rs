// Test fl2rust generated code

#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_imports)]

use fltk::browser::*;
use fltk::button::*;
use fltk::dialog::*;
use fltk::frame::*;
use fltk::group::*;
use fltk::image::*;
use fltk::input::*;
use fltk::menu::*;
use fltk::misc::*;
use fltk::output::*;
use fltk::prelude::*;
use fltk::table::*;
use fltk::text::*;
use fltk::tree::*;
use fltk::valuator::*;
use fltk::widget::*;
use fltk::window::*;


#[derive(Debug, Clone, Default)]
pub struct UserInterface {
    pub fds: Slider,
    pub fds2: Slider,
}


impl UserInterface {
    pub fn make_window2() -> Self {
        let mut fl2rust_gen_widget_0 = Window::new(186, 209, 229, 214, "");
        fl2rust_gen_widget_0.end();
        fl2rust_gen_widget_0.show();
        let mut fl2rust_gen_widget_1 = Button::new(90, 90, 60, 15, "button");
        fl2rust_gen_widget_0.add(&fl2rust_gen_widget_1);
        let mut fds = Slider::new(25, 35, 175, 35, "fds");
        fds.set_type(SliderType::HorizontalNice);
        fl2rust_gen_widget_0.add(&fds);
        Self { fds, ..Default::default() }
    }
    pub fn make_window() -> Self {
        let mut fl2rust_gen_widget_2 = Window::new(186, 209, 229, 214, "");
        fl2rust_gen_widget_2.end();
        fl2rust_gen_widget_2.show();
        let mut fl2rust_gen_widget_3 = Button::new(90, 90, 60, 15, "button");
        fl2rust_gen_widget_2.add(&fl2rust_gen_widget_3);
        let mut fds2 = Slider::new(25, 35, 175, 35, "fds");
        fds2.set_type(SliderType::HorizontalNice);
        fl2rust_gen_widget_2.add(&fds2);
        Self {fds2, ..Default::default() }
    }
}

fn main() {
    use fltk::app;
    let app = app::App::default();
    let ui = UserInterface::make_window2();
    app.run().unwrap();
}