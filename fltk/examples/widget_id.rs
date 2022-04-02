use fltk::{prelude::*, *};
use std::sync::Mutex;
use std::collections::HashMap;

lazy_static::lazy_static! {
    static ref WIDGET_MAP: Mutex<HashMap<&'static str, widget::Widget>> = Mutex::new(HashMap::default());
}

pub trait WidgetId<W> where W: WidgetExt {
    fn set_id(&mut self, id: &'static str);
    fn with_id(self, id: &'static str) -> Self where Self: Sized;
}

impl<W> WidgetId<W> for W where W: WidgetExt {
    fn set_id(&mut self, id: &'static str) {
        WIDGET_MAP.lock().unwrap().insert(id, unsafe { self.as_widget() });
    }
    fn with_id(mut self, id: &'static str) -> Self {
        self.set_id(id);
        self
    }
}

pub fn from_id(id: &'static str) -> Option<widget::Widget> {
    if let Some(w) = WIDGET_MAP.lock().unwrap().get(&id) {
        Some(w.clone())
    } else {
        None
    }
}

// So we can do `widget.on_trigger()` and get self back. Useful for chaining methods.
trait OnTrigger<W> where W: WidgetExt {
    fn on_trigger<F: 'static + FnMut(&mut Self)>(self, cb: F) -> Self where Self: Sized;
}

impl<W> OnTrigger<W> for W where W: WidgetExt {
    fn on_trigger<F: 'static + FnMut(&mut Self)>(mut self, mut cb: F) -> Self {
        self.set_callback(move |s| cb(s));
        self
    }
}

// For calls inside a closure
fn increment_by(step: i32) {
    if let Some(mut frame) = from_id("my_frame") {
        let label: i32 = frame.label().unwrap().parse().unwrap();
        frame.set_label(&(label + step).to_string());
    }
}

// To pass a function object directly!
fn increment(_w: &mut impl WidgetExt) {
    if let Some(mut frame) = from_id("my_frame") {
        let label: i32 = frame.label().unwrap().parse().unwrap();
        frame.set_label(&(label + 1).to_string());
    }
}

fn main() {
    let app = app::App::default().with_scheme(app::Scheme::Gtk);

    let mut wind = window::Window::default()
        .with_size(160, 200)
        .with_label("Counter");
    let col = group::Flex::default_fill().column();
    button::Button::default()
        .with_label("+")
        .on_trigger(increment); // passed by function object
    frame::Frame::default().with_label("0").with_id("my_frame"); // pass id here
    button::Button::default()
        .with_label("-")
        .on_trigger(|_| increment_by(-1)); // called in closure
    col.end();
    wind.make_resizable(true);
    wind.end();
    wind.show();

    app.run().unwrap();
}
