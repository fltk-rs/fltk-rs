use fltk::{enums::*, prelude::*, *};
use std::cell::RefCell;
use std::rc::Rc;

type WidgetCb = Rc<RefCell<Option<Box<dyn FnMut(&mut frame::Frame)>>>>;

struct MyButton {
    btn: frame::Frame,
    hover_cb: WidgetCb,
    leave_cb: WidgetCb,
    push_cb: WidgetCb,
}

impl MyButton {
    pub fn new(x: i32, y: i32, w: i32, h: i32, label: &str) -> Self {
        let mut btn = frame::Frame::new(x, y, w, h, None).with_label(label);
        btn.set_frame(FrameType::RFlatBox);
        btn.set_color(Color::Cyan);
        let hover_cb: WidgetCb = Rc::from(RefCell::from(None));
        let leave_cb: WidgetCb = Rc::from(RefCell::from(None));
        let push_cb: WidgetCb = Rc::from(RefCell::from(None));
        btn.handle({
            let hover_cb = hover_cb.clone();
            let leave_cb = leave_cb.clone();
            let push_cb = push_cb.clone();
            move |b, ev| match ev {
                Event::Enter => {
                    if let Some(cb) = hover_cb.borrow_mut().as_mut() {
                        cb(b);
                    }
                    true
                }
                Event::Leave => {
                    if let Some(cb) = leave_cb.borrow_mut().as_mut() {
                        cb(b);
                    }
                    true
                }
                Event::Push => {
                    if let Some(cb) = push_cb.borrow_mut().as_mut() {
                        cb(b);
                    }
                    true
                }
                _ => false,
            }
        });
        Self {
            btn,
            hover_cb,
            leave_cb,
            push_cb,
        }
    }
    pub fn on_hover(&mut self, cb: impl FnMut(&mut frame::Frame) + 'static) {
        *self.hover_cb.borrow_mut() = Some(Box::new(cb));
    }
    pub fn on_leave(&mut self, cb: impl FnMut(&mut frame::Frame) + 'static) {
        *self.leave_cb.borrow_mut() = Some(Box::new(cb));
    }
    pub fn on_click(&mut self, cb: impl FnMut(&mut frame::Frame) + 'static) {
        *self.push_cb.borrow_mut() = Some(Box::new(cb));
    }
}

widget_extends!(MyButton, frame::Frame, btn);

fn main() {
    let app = app::App::default();
    let mut wind = window::Window::default().with_size(400, 300);
    wind.set_color(Color::White);
    let mut but = MyButton::new(160, 210, 80, 40, "Click me!");
    wind.end();
    wind.show();

    but.on_hover(|b| {
        b.set_color(Color::Cyan.lighter().lighter());
        b.redraw();
    });

    but.on_leave(move |b| {
        b.set_color(Color::Cyan);
        b.redraw();
    });

    but.on_click(|_| {
        println!("Clicked");
    });

    app.run().unwrap();
}
