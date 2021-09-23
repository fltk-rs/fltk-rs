use fltk::{prelude::*, *};

fn main() {
    let a = app::App::default().with_scheme(app::Scheme::Gtk);
    let mut win = window::Window::default().with_size(640, 480);
    let mut col = group::Flex::default()
        .with_size(630, 470)
        .center_of_parent()
        .column();
    main_panel(&mut col);
    col.end();
    win.resizable(&col);
    win.set_color(enums::Color::from_rgb(250, 250, 250));
    win.end();
    win.show();
    a.run().unwrap();
}

fn buttons_panel(parent: &mut group::Flex) {
    frame::Frame::default();
    let mut w = frame::Frame::default().with_label("Welcome to Flex Login");

    let mut urow = group::Flex::default().row();
    {
        frame::Frame::default()
            .with_label("Username:")
            .with_align(enums::Align::Inside | enums::Align::Right);
        let mut username = input::Input::default();

        urow.set_size(&mut username, 180);
        urow.end();
    }

    let mut prow = group::Flex::default().row();
    {
        frame::Frame::default()
            .with_label("Password:")
            .with_align(enums::Align::Inside | enums::Align::Right);
        let mut password = input::Input::default();

        prow.set_size(&mut password, 180);
        prow.end();
    }

    let mut pad = frame::Frame::default();

    let mut brow = group::Flex::default().row();
    {
        frame::Frame::default();
        let mut reg = create_button("Register");
        let mut login = create_button("Login");

        brow.set_size(&mut reg, 80);
        brow.set_size(&mut login, 80);
        brow.end();
    }

    let mut b = frame::Frame::default();

    frame::Frame::default();

    parent.set_size(&mut w, 60);
    parent.set_size(&mut urow, 30);
    parent.set_size(&mut prow, 30);
    parent.set_size(&mut pad, 1);
    parent.set_size(&mut brow, 30);
    parent.set_size(&mut b, 30);
}

fn middle_panel(parent: &mut group::Flex) {
    frame::Frame::default();

    let mut frame = frame::Frame::default().with_label("Image");
    frame.set_frame(enums::FrameType::BorderBox);
    frame.set_color(enums::Color::from_rgb(0, 200, 0));
    let mut spacer = frame::Frame::default();

    let mut bp = group::Flex::default().column();
    buttons_panel(&mut bp);
    bp.end();

    frame::Frame::default();

    parent.set_size(&mut frame, 200);
    parent.set_size(&mut spacer, 10);
    parent.set_size(&mut bp, 300);
}

fn main_panel(parent: &mut group::Flex) {
    frame::Frame::default();

    let mut mp = group::Flex::default().row();
    middle_panel(&mut mp);
    mp.end();

    frame::Frame::default();

    parent.set_size(&mut mp, 200);
}

fn create_button(caption: &str) -> button::Button {
    let mut btn = button::Button::default().with_label(caption);
    btn.set_color(enums::Color::from_rgb(225, 225, 225));
    btn
}
