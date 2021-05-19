mod closable_tab {
    use fltk::{
        app, button,
        enums::{Align, Event, FrameType},
        group,
        prelude::*,
    };

    // Internal
    struct TabButton {
        grp: group::Group,
        but: button::Button,
    }

    impl TabButton {
        pub fn new(label: &'static str) -> Self {
            let mut grp = group::Group::new(0, 0, 150, 40, label);
            let mut but = button::Button::new(grp.x() + 120, grp.y() + 10, 20, 20, "@1+");
            grp.end();
            grp.set_frame(FrameType::UpFrame);
            but.set_frame(FrameType::FlatBox);
            Self { grp, but }
        }
    }

    // Public
    pub struct ClosableTab {
        grp: group::Group,
        pk: group::Pack,
    }

    impl ClosableTab {
        pub fn new(x: i32, y: i32, w: i32, h: i32) -> Self {
            let parent_grp = group::Group::new(x, y, w, h, None);
            let mut pk = group::Pack::new(x + 5, y, w - 10, 40, None);
            pk.set_spacing(3);
            pk.set_type(group::PackType::Horizontal);
            pk.end();
            let mut grp = group::Group::new(x, y + 40, w, h - 40, None);
            grp.set_frame(FrameType::UpFrame);
            grp.end();
            parent_grp.end();
            Self { grp, pk }
        }
        pub fn add(&mut self, grp: &mut group::Group) {
            grp.resize(self.grp.x(), self.grp.y(), self.grp.w(), self.grp.h());
            if self.grp.children() == 0 {
                grp.show();
            } else {
                grp.hide();
            }
            self.grp.add(grp);
            let label = grp.label();
            grp.set_label("");
            let mut but = TabButton::new("");
            but.grp.set_align(Align::Left | Align::Inside);
            but.grp.set_label(&label);
            but.but.clear_visible_focus();
            but.grp.handle({
                let self_grp = self.grp.clone();
                let mut curr_grp = grp.clone();
                move |_, ev| match ev {
                    Event::Push => {
                        for child in 0..self_grp.children() {
                            self_grp.child(child).unwrap().hide();
                        }
                        curr_grp.show();
                        true
                    }
                    _ => false,
                }
            });
            self.pk.add(&but.grp);
            but.but.set_callback({
                let curr_grp = grp.clone();
                let mut self_grp = self.grp.clone();
                let mut self_pack = self.pk.clone();
                move |_| {
                    let idx = self_grp.find(&curr_grp);
                    self_grp.remove_by_index(idx);
                    self_pack.remove_by_index(idx);
                    if let Some(mut grp) = self_grp.child(self_grp.children() - 1) {
                        grp.show();
                    }
                    app::redraw();
                }
            });
        }
    }
}

fn main() {
    use fltk::{prelude::*, *};
    // Create groups to be used as tabs
    pub fn create_tab(count: i32) -> group::Group {
        let grp = group::Group::new(0, 0, 800, 600, None).with_label(&format!("tab {}", count));
        button::Button::new(count * 200, count * 200, 80, 40, None)
            .with_label(&format!("button {}", count));
        grp.end();
        grp
    }
    let app = app::App::default();
    let mut win = window::Window::default().with_size(800, 600);
    let mut tabs = closable_tab::ClosableTab::new(0, 0, 800, 600);
    win.end();
    win.show();
    tabs.add(&mut create_tab(1));
    tabs.add(&mut create_tab(2));
    app.run().unwrap();
}
