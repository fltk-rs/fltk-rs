use fltk::{app, button::Button, frame::Frame, group::Flex, prelude::*, window::Window};
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
    let mut flex = Flex::default_fill().column();
    flex.set_margins(30, 40, 30, 40);
    flex.set_pad(10);
    let mut but_inc = Button::default().with_label("+");
    let mut frame = Frame::default().with_label(&counter.value().to_string());
    let mut but_dec = Button::default().with_label("-");
    flex.end();
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
        if ev == MyEvent::CHANGED.into() {
            f.set_label(&counter.clone().value().to_string());
            true
        } else {
            false
        }
    });

    Ok(app.run()?)
}
