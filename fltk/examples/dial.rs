use fltk::{
    app,
    button::Button,
    enums::{FrameType, Event, CallbackTrigger},
    valuator::Dial,
    group::Flex,
    prelude::*,
    window::Window,
};

const HEARTBEAT: i32 = 31;
const PAD: i32 = 10;

fn main() {
    let app = app::App::default().with_scheme(app::Scheme::Plastic);
    let mut wind = Window::default().with_size(640, 360).with_label("Dial");
    {
        let mut page = Flex::default().with_size(100, 200 - PAD).center_of_parent().column();
        crate::dial();
        Button::default().with_label("@circle").set_callback(move |_| {
            app::handle_main(Event::from_i32(HEARTBEAT)).unwrap();
        });
        page.end();
        page.set_frame(FrameType::FlatBox);
        page.set_margin(PAD);
        page.set_pad(PAD);
    }
    wind.end();
    wind.show();
    app.run().unwrap();
}

fn dial() {
    const DIAL: u8 = 120;
    let mut element = Dial::default();
    element.set_maximum((DIAL / 4 * 3) as f64);
    element.set_value(element.minimum());
    element.handle(move |dial, event| if event == Event::from_i32(HEARTBEAT) {
        dial.set_value(match dial.value() == (DIAL - 1) as f64 {
            true => dial.minimum(),
            false => dial.value() + 1f64,
        });
        true
    } else {
        false
    });
}
