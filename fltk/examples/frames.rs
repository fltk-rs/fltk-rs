use fltk::{prelude::*, *};

struct MyFrame {
    #[allow(dead_code)]
    f: frame::Frame,
}

impl MyFrame {
    pub fn new(idx: usize) -> MyFrame {
        let mut f = frame::Frame::default();
        // Normally you would use the FrameType enum, for example:
        // some_widget.set_frame(FrameType::DownBox);
        f.set_frame(enums::FrameType::by_index(idx));
        f.set_color(enums::Color::from_u32(0x7FFFD4));
        let f_name = format!("{:?}", f.frame());
        f.set_label(&f_name);
        f.set_label_size(12);
        Self { f }
    }
}

fn main() {
    let app = app::App::default();
    let mut win = window::Window::default()
        .with_size(1000, 800)
        .with_label("Frames")
        .center_screen();

    let mut col = group::Flex::default_fill().column();
    col.set_margin(20);

    let mut row = group::Flex::default();
    col.fixed(&row, 75);
    for i in 0..8 {
        let _ = MyFrame::new(i);
    }
    row.end();
    row.set_pad(10);

    let mut row = group::Flex::default();
    col.fixed(&row, 75);
    for i in 8..17 {
        let _ = MyFrame::new(i);
    }
    row.end();
    row.set_pad(10);

    let mut row = group::Flex::default();
    col.fixed(&row, 75);
    for i in 17..26 {
        let _ = MyFrame::new(i);
    }
    row.end();
    row.set_pad(10);

    let mut row = group::Flex::default();
    col.fixed(&row, 75);
    for i in 26..35 {
        let _ = MyFrame::new(i);
    }
    row.end();
    row.set_pad(10);

    let mut row = group::Flex::default();
    col.fixed(&row, 75);
    for i in 35..44 {
        let _ = MyFrame::new(i);
    }
    row.end();
    row.set_pad(10);

    let mut row = group::Flex::default();
    col.fixed(&row, 75);
    for i in 44..53 {
        let _ = MyFrame::new(i);
    }
    row.end();
    row.set_pad(10);

    col.end();
    col.set_pad(30);

    win.end();
    win.show();
    win.set_color(enums::Color::White);

    app.run().unwrap();
}
