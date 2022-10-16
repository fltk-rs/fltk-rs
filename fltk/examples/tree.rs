use fltk::{
    app,
    button::Button,
    enums::Event,
    frame::Frame,
    prelude::*,
    tree::{Tree, TreeItem, TreeSelect},
    window::Window,
};
use std::env;

#[derive(PartialEq)]
enum State {
    MovingUp,
    MovingDown,
    Undefined,
}

struct TreeMouseFocus {
    t_widget: fltk::tree::Tree,
    previous_focus: Option<TreeItem>,
}

impl TreeMouseFocus {
    fn new(x: i32, y: i32, width: i32, height: i32, title: &'static str) -> Self {
        let mut t_widget = Tree::new(x, y, width, height, title);
        let previous_focus: Option<TreeItem> = None;
        let mut pf = previous_focus.clone();
        t_widget.handle(move |t, e| match e {
            Event::Move => {
                let (_, mouse_y) = app::event_coords();
                let mut state = State::Undefined;
                loop {
                    match &pf {
                        Some(item) => {
                            let item_y = item.y();
                            if mouse_y < item_y {
                                pf = pf.as_ref().unwrap().prev();
                                state = State::MovingUp;
                                continue;
                            };
                            if mouse_y > item_y + item.h() {
                                pf = pf.as_ref().unwrap().next();
                                state = State::MovingDown;
                                continue;
                            };
                            if state == State::Undefined {
                                return true; // If in same range, don't update 'previous_focus'
                            };
                            break;
                        }
                        // En up here if y is outside tree boundaries, or no tree item is present
                        None => match &state {
                            State::MovingUp | State::MovingDown => return true,
                            State::Undefined => {
                                pf = t.first();
                                if pf.is_none() {
                                    return true;
                                }
                                continue;
                            }
                        },
                    };
                }
                let mut open = true;
                let mut pa = pf.clone();
                loop {
                    match &pa.as_ref().unwrap().parent() {
                        Some(p) => {
                            if p.is_close() {
                                open = false;
                                break;
                            } else {
                                pa = Some(p.clone());
                            }
                        }
                        None => {
                            break;
                        }
                    }
                }
                if open == true {
                    t.take_focus();
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
        self.previous_focus = None;
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

    wind.make_resizable(true);
    wind.show();

    but.set_callback(move |_| match tree2.get_selected_items() {
        None => println!("No items selected"),
        Some(vals) => print!(
            "In total {} items selected:\n{}",
            vals.len(),
            vals.iter()
                .map(|i| i.label().unwrap() + "\n")
                .collect::<String>()
        ),
    });

    app.run().unwrap();
}
