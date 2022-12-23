#![allow(dead_code)]

use fltk::{
    app,
    button::Button,
    enums::Event,
    frame::Frame,
    prelude::*,
    tree::{Tree, TreeItem, TreeReason, TreeSelect},
    window::Window,
};
use std::cell::RefCell;
use std::env;
use std::rc::Rc;

#[derive(PartialEq)]
enum State {
    MovingUp,
    MovingDown,
    Undefined,
}

fn verify_open_till_root(opt: &Option<fltk::tree::TreeItem>) -> bool {
    let mut par = opt.clone();
    loop {
        match par.as_ref().unwrap().parent() {
            Some(p) => {
                if p.is_close() {
                    return false;
                } else {
                    par = Some(p.clone());
                }
            }
            None => return true,
        }
    }
}

struct TreeMouseFocus {
    t_widget: fltk::tree::Tree,
    previous_focus: Rc<RefCell<Option<TreeItem>>>,
}

impl TreeMouseFocus {
    fn new(x: i32, y: i32, width: i32, height: i32, title: &'static str) -> Self {
        let mut t_widget = Tree::new(x, y, width, height, title);
        let previous_focus = Rc::new(RefCell::new(None::<TreeItem>));
        let pfr = Rc::clone(&previous_focus);
        t_widget.set_callback_reason(TreeReason::Selected);
        t_widget.set_callback(|_t| println!("clicked an item"));
        t_widget.handle(move |t, e| match e {
            Event::Move => {
                let (_, mouse_y) = app::event_coords();
                let mut state = State::Undefined;
                let mut pf = pfr.borrow_mut();
                loop {
                    match &*pf {
                        Some(item) => {
                            let item_y = item.y();
                            match state {
                                State::MovingUp => {
                                    if verify_open_till_root(&pf) == true {
                                        if mouse_y < item_y {
                                            *pf = pf.as_ref().unwrap().prev();
                                            continue;
                                        };
                                        break;
                                    } else {
                                        *pf = pf.as_ref().unwrap().prev();
                                        continue;
                                    }
                                }
                                State::MovingDown => {
                                    if verify_open_till_root(&pf) == true {
                                        if mouse_y > item_y + item.h() {
                                            *pf = pf.as_ref().unwrap().next();
                                            continue;
                                        };
                                        break;
                                    } else {
                                        *pf = pf.as_ref().unwrap().next();
                                        continue;
                                    }
                                }
                                State::Undefined => {
                                    if mouse_y < item_y {
                                        *pf = pf.as_ref().unwrap().prev();
                                        state = State::MovingUp;
                                        continue;
                                    };
                                    if mouse_y > item_y + item.h() {
                                        *pf = pf.as_ref().unwrap().next();
                                        state = State::MovingDown;
                                        continue;
                                    };
                                    return true; // If in same range, don't update 'previous_focus'
                                }
                            }
                        }
                        // End up here if y is outside tree boundaries, or no tree item is present
                        None => match &state {
                            State::MovingUp | State::MovingDown => return true,
                            State::Undefined => {
                                *pf = t.first();
                                state = State::MovingDown;
                                if pf.is_none() {
                                    return true;
                                }
                                continue;
                            }
                        },
                    };
                }
                if verify_open_till_root(&pf) == true {
                    t.take_focus().ok();
                    t.set_item_focus(&pf.as_ref().unwrap());
                    println!("Set focus to item: {:?}", pf.as_ref().unwrap().label());
                }
                true
            }
            _ => false,
        });
        Self {
            t_widget,
            previous_focus,
        }
    }

    fn add(&mut self, path: &str) -> Option<TreeItem> {
        self.t_widget.add(path)
    }

    /// Caution, variable 'previous focus' must be set to None, as it
    /// otherwise could try to refer to an already freed memory location,
    /// when this TreeItem is removed.
    fn remove(&mut self, item: &TreeItem) -> Result<(), FltkError> {
        *self.previous_focus.borrow_mut() = None;
        self.t_widget.remove(item)
    }

    fn get_items(&self) -> Option<Vec<TreeItem>> {
        self.t_widget.get_items()
    }
}

fn main() {
    let path = env::current_dir().unwrap();
    let path: String = path
        .to_str()
        .unwrap()
        .chars()
        .enumerate()
        .map(|(_, c)| match c {
            '\\' => '/', // change window paths to posix paths
            _ => c,
        })
        .collect();

    let app = app::App::default().with_scheme(app::Scheme::Gtk);
    let mut wind = Window::default().with_size(400, 300);
    let mut but = Button::new(260, 255, 80, 40, "Get Items");
    let _frame = Frame::new(20, 255, 160, 40, "Focus follow mouse");
    let mut tree = TreeMouseFocus::new(5, 10, 190, 240, "");
    tree.add(&path);

    let mut items = tree.get_items().unwrap();
    items.as_mut_slice()[0].set_label("/");

    let mut tree2 = Tree::new(205, 10, 190, 240, "");
    tree2.set_select_mode(TreeSelect::Multi);
    tree2.add("First");
    tree2.add("First/1st");
    tree2.add("First/2nd/3rd");
    tree2.add("Second");
    tree2.add("Third");

    tree2.set_trigger(fltk::enums::CallbackTrigger::ReleaseAlways);

    wind.make_resizable(true);
    wind.show();

    but.set_callback({
        let tree2 = tree2.clone();
        move |_| match tree2.get_selected_items() {
            None => println!("No items selected"),
            Some(vals) => print!(
                "In total {} items selected:\n{}",
                vals.len(),
                vals.iter()
                    .map(|i| tree2.item_pathname(&i).unwrap() + "\n")
                    .collect::<String>()
            ),
        }
    });

    app.run().unwrap();
}
