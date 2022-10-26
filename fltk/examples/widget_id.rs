#![feature(once_cell)]

use std::sync::LazyLock;
use fltk::{prelude::*, *};
use std::any::Any;
use std::collections::HashMap;
use std::sync::Mutex;

static WIDGET_MAP: LazyLock<Mutex<HashMap<&'static str, Box<dyn Any + Send + Sync + 'static>>>> =
    LazyLock::new(|| Mutex::new(HashMap::default()));

pub trait WidgetId<W>
where
    W: WidgetExt,
{
    fn set_id(&mut self, id: &'static str);
    fn with_id(self, id: &'static str) -> Self
    where
        Self: Sized;
}

impl<W> WidgetId<W> for W
where
    W: WidgetExt + Send + Sync + Clone + 'static,
{
    fn set_id(&mut self, id: &'static str) {
        WIDGET_MAP
            .lock()
            .unwrap()
            .insert(id, Box::new(self.clone()));
    }
    fn with_id(mut self, id: &'static str) -> Self {
        self.set_id(id);
        self
    }
}

pub fn from_id<T: 'static + Clone>(id: &'static str) -> Option<T> {
    if let Some(w) = WIDGET_MAP.lock().unwrap().get(&id) {
        w.downcast_ref::<T>().map(|w| (*w).clone())
    } else {
        None
    }
}

// So we can do `widget.on_trigger()` and get self back. Useful for chaining methods.
trait OnTrigger<W>
where
    W: WidgetExt,
{
    fn on_trigger<F: 'static + FnMut(&mut Self)>(self, cb: F) -> Self
    where
        Self: Sized;
}

impl<W> OnTrigger<W> for W
where
    W: WidgetExt,
{
    fn on_trigger<F: 'static + FnMut(&mut Self)>(mut self, mut cb: F) -> Self {
        self.set_callback(move |s| cb(s));
        self
    }
}

struct Counter {
    count: i32,
}

// For calls inside a closure
fn increment_by(step: i32) {
    let mut frame: frame::Frame = from_id("my_frame").unwrap();
    let state = app::GlobalState::<Counter>::get();
    let count = state.with(move |c| {
        c.count += step;
        c.count
    });
    frame.set_label(&count.to_string());
}

// To pass a function object directly!
fn increment(_w: &mut impl WidgetExt) {
    let mut frame: frame::Frame = from_id("my_frame").unwrap();
    let state = app::GlobalState::<Counter>::get();
    let count = state.with(|c| {
        c.count += 1;
        c.count
    });
    frame.set_label(&count.to_string());
}

fn main() {
    let counter = Counter { count: 0 };
    let _state = app::GlobalState::new(counter);
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

