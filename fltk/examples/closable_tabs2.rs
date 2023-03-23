// Shows how to create custom closable tabs

#![allow(dead_code)]

mod closable_tab {
    use fltk::{
        app, button,
        enums::{Align, Color, FrameType},
        group,
        prelude::*,
    };

    #[derive(Copy, Clone)]
    pub enum Message {
        Foreground(i32),
        Delete(i32),
        InsertNew(i32),
    }

    fn create_tab_button(label: &str) -> group::Group {
        let mut grp = group::Group::new(0, 0, 150, 40, None);
        grp.set_align(Align::Left | Align::Inside);
        let mut but_handle = button::Button::new(grp.x() + 5, grp.y() + 5, 110, 30, "");
        but_handle.set_align(Align::Left | Align::Inside);
        but_handle.set_label(label);
        let mut but_close = button::Button::new(grp.x() + 120, grp.y() + 10, 20, 20, "@1+");
        but_handle.set_frame(FrameType::FlatBox);
        but_handle.clear_visible_focus();
        but_close.set_frame(FrameType::FlatBox);
        grp.end();
        grp.set_frame(FrameType::UpFrame);
        grp
    }

    // Public
    pub struct ClosableTab<'a> {
        current: Option<i32>, // The tab which is visible on the foreground
        snd: &'a app::Sender<Message>,
        contents: group::Group,
        tab_labels: group::Pack,
    }

    impl<'a> ClosableTab<'a> {
        pub fn new(x: i32, y: i32, w: i32, h: i32, snd: &'a app::Sender<Message>) -> Self {
            let current = None;
            let parent_grp = group::Group::new(x, y, w, h, None);
            let mut tab_labels = group::Pack::new(x + 5, y, w - 10, 40, None);
            tab_labels.set_spacing(3);
            tab_labels.set_type(group::PackType::Horizontal);
            tab_labels.end();
            let mut contents = group::Group::new(x, y + 40, w, h - 40, None);
            contents.set_frame(FrameType::NoBox);
            contents.end();
            parent_grp.end();
            Self {
                current,
                snd,
                contents,
                tab_labels,
            }
        }

        pub fn add(&mut self, child: &mut group::Group, label: &str) {
            child.resize(
                self.contents.x(),
                self.contents.y(),
                self.contents.w(),
                self.contents.h(),
            );
            self.contents.add(child);
            let but = create_tab_button(label);
            self.tab_labels.add(&but);
            but.child(1).unwrap().set_callback({
                let curr_child = child.clone();
                let contents = self.contents.clone();
                let sndb = *self.snd;
                move |_| {
                    let idx = contents.find(&curr_child);
                    sndb.send(Message::Delete(idx));
                    app::redraw();
                }
            });
            but.child(0).unwrap().set_callback({
                let curr_child = child.clone();
                let contents = self.contents.clone();
                let sndb = *self.snd;
                move |_| {
                    let idx = contents.find(&curr_child);
                    sndb.send(Message::Foreground(idx));
                    app::redraw();
                }
            });
        }

        pub fn remove(&mut self, idx: i32) {
            self.contents.remove_by_index(idx);
            self.tab_labels.remove_by_index(idx);
            if self.current == Some(idx) {
                if idx > 1 {
                    self.set_foreground(idx - 1);
                } else if self.contents.children() > 0 {
                    self.set_foreground(0);
                }
            }
        }

        /** No return variable, fails silently */
        pub fn set_foreground(&mut self, fg_idx: i32) {
            for idx in 0..self.contents.children() {
                if idx != fg_idx {
                    self.contents.child(idx).unwrap().hide();
                    self.tab_labels
                        .child(idx)
                        .unwrap()
                        .set_label_color(fltk::enums::Color::Inactive);
                    self.tab_labels
                        .child(idx)
                        .unwrap()
                        .set_color(fltk::enums::Color::Inactive);
                    self.tab_labels
                        .child(idx)
                        .unwrap()
                        .set_frame(FrameType::DownFrame);
                } else {
                    self.contents.child(idx).unwrap().show();
                    self.tab_labels
                        .child(idx)
                        .unwrap()
                        .set_label_color(Color::Selection);
                    self.tab_labels
                        .child(idx)
                        .unwrap()
                        .set_color(fltk::enums::Color::Selection);
                    self.tab_labels
                        .child(idx)
                        .unwrap()
                        .set_frame(FrameType::NoBox);
                }
                self.tab_labels.child(idx).unwrap().set_damage(true);
                self.tab_labels.child(idx).unwrap().redraw();
                self.current = Some(fg_idx);
            }
        }

        /** Report which tab index is visible on foreground */
        pub fn get_foreground(&self) -> Option<i32> {
            self.current
        }
    }
}

fn main() {
    use fltk::{prelude::*, *};
    // Create groups to be used as content for tabs
    pub fn create_tab(from: i32, to: i32) -> group::Group {
        let grp = group::Group::new(0, 0, 800, 600, None);
        for idx in from..to {
            button::Button::new(
                idx * 10 + (idx - from) * 42,
                idx * 10 + (idx - from) * 42,
                80,
                40,
                None,
            )
            .with_label(&format!("button {idx}"));
        }
        grp.end();
        grp
    }
    let app = app::App::default();
    let mut win = window::Window::default().with_size(800, 600);
    let (s, r) = app::channel::<closable_tab::Message>();
    let mut tabs = closable_tab::ClosableTab::new(0, 0, 800, 600, &s);
    win.end();
    win.show();
    tabs.add(&mut create_tab(1, 3), "tab 1");
    tabs.add(&mut create_tab(4, 7), "tab 2");
    tabs.add(&mut create_tab(8, 11), "tab 3");
    tabs.add(&mut create_tab(12, 15), "tab 4");
    tabs.add(&mut create_tab(16, 22), "tab 5");
    tabs.set_foreground(2);
    while app.wait() {
        use closable_tab::Message::*;
        if let Some(msg) = r.recv() {
            match msg {
                Foreground(idx) => {
                    tabs.set_foreground(idx);
                }
                Delete(idx) => {
                    tabs.remove(idx);
                }
                InsertNew(_) => {}
            }
        }
    }
}
