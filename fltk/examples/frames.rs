use fltk::{prelude::*, *};

struct MyFrame {
    f: frame::Frame,
}

impl MyFrame {
    pub fn new(idx: usize) -> MyFrame {
        let mut f = MyFrame {
            f: frame::Frame::default().with_size(150, 75),
        };
        // Normally you would use the FrameType enum, for example:
        // some_widget.set_frame(FrameType::DownBox);
        f.f.set_frame(enums::FrameType::by_index(idx));
        f.f.set_color(enums::Color::from_u32(0x7FFFD4));
        let f_name = format!("{:?}", f.f.frame());
        f.f.set_label(&f_name);
        f.f.set_label_size(12);
        f
    }
}

fn main() {
    let app = app::App::default();
    let mut win = window::Window::default()
        .with_size(1000, 800)
        .with_label("Frames")
        .center_screen();

    let mut hpack = group::Pack::new(20, 0, 1000, 800, "");

    let mut pack = group::Pack::default().with_size(150, 800);
    for i in 0..9 {
        let _ = MyFrame::new(i);
    }
    pack.end();
    pack.set_spacing(10);

    let mut pack = group::Pack::default().with_size(150, 800);
    for i in 9..18 {
        let _ = MyFrame::new(i);
    }
    pack.end();
    pack.set_spacing(10);

    let mut pack = group::Pack::default().with_size(150, 800);
    for i in 18..27 {
        let _ = MyFrame::new(i);
    }
    pack.end();
    pack.set_spacing(10);

    let mut pack = group::Pack::default().with_size(150, 800);
    for i in 27..36 {
        let _ = MyFrame::new(i);
    }
    pack.end();
    pack.set_spacing(10);

    let mut pack = group::Pack::default().with_size(150, 800);
    for i in 36..45 {
        let _ = MyFrame::new(i);
    }
    pack.end();
    pack.set_spacing(10);

    let mut pack = group::Pack::default().with_size(150, 800);
    for i in 45..54 {
        let _ = MyFrame::new(i);
    }
    pack.end();
    pack.set_spacing(10);

    hpack.end();
    hpack.set_spacing(10);
    hpack.set_type(group::PackType::Horizontal);

    win.end();
    win.show();
    win.set_color(enums::Color::White);

    app.run().unwrap();
}
