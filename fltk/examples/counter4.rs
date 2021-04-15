use fltk::{app, button::Button, frame::Frame, group::Pack, prelude::*, window::Window};
use std::cell::RefCell;
use std::rc::Rc;

pub struct MyEvent;

impl MyEvent {
    const CHANGED: i32 = 40;
}

#[derive(Clone)]
pub struct Counter {
    count: Rc<RefCell<i32>>,
}

impl Counter {
    #[must_use]
    pub fn new(val: i32) -> Self {
        Counter {
            count: Rc::from(RefCell::from(val)),
        }
    }

    pub fn increment(&mut self) {
        *self.count.borrow_mut() += 1;
        app::handle_main(MyEvent::CHANGED).unwrap();
    }

    pub fn decrement(&mut self) {
        *self.count.borrow_mut() -= 1;
        app::handle_main(MyEvent::CHANGED).unwrap();
    }

    #[must_use]
    pub fn value(&self) -> i32 {
        *self.count.borrow()
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = app::App::default();
    let counter = Counter::new(0);
    let mut wind = Window::default().with_size(160, 200).with_label("Counter");
    let mut pack = Pack::default().with_size(120, 140).center_of(&wind);
    pack.set_spacing(10);
    let mut but_inc = Button::default().with_size(0, 40).with_label("+");
    let mut frame = Frame::default()
        .with_size(0, 40)
        .with_label(&counter.value().to_string());
    let mut but_dec = Button::default().with_size(0, 40).with_label("-");
    pack.end();
    wind.end();
    wind.show();

    but_inc.set_callback({
        let mut c = counter.clone();
        move |_| c.increment()
    });

    but_dec.set_callback({
        let mut c = counter.clone();
        move |_| c.decrement()
    });

    frame.handle(move |f, ev| {
        if ev as i32 == MyEvent::CHANGED {
            f.set_label(&counter.clone().value().to_string());
            true
        } else {
            false
        }
    });

    Ok(app.run()?)
}
