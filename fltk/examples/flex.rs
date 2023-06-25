use fltk::{prelude::*, *};

fn main() {
    let a = app::App::default().with_scheme(app::Scheme::Gtk);
    let mut win = window::Window::default().with_size(640, 480);
    let mut col = group::Flex::default_fill().column();
    main_panel(&mut col);
    col.end();
    win.resizable(&col);
    win.set_color(enums::Color::from_rgb(250, 250, 250));
    win.end();
    win.show();
    win.size_range(600, 400, 0, 0);
    a.run().unwrap();
}

fn buttons_panel(parent: &mut group::Flex) {
    frame::Frame::default();
    let w = frame::Frame::default().with_label("Welcome to Flex Login");

    let mut urow = group::Flex::default().row();
    {
        frame::Frame::default()
            .with_label("Username:")
            .with_align(enums::Align::Inside | enums::Align::Right);
        let username = input::Input::default();

        urow.fixed(&username, 180);
        urow.end();
    }

    let mut prow = group::Flex::default().row();
    {
        frame::Frame::default()
            .with_label("Password:")
            .with_align(enums::Align::Inside | enums::Align::Right);
        let password = input::Input::default();

        prow.fixed(&password, 180);
        prow.end();
    }

    let pad = frame::Frame::default();

    let mut brow = group::Flex::default().row();
    {
        frame::Frame::default();
        let reg = create_button("Register");
        let login = create_button("Login");

        brow.fixed(&reg, 80);
        brow.fixed(&login, 80);
        brow.end();
    }

    let b = frame::Frame::default();

    frame::Frame::default();

    parent.fixed(&w, 60);
    parent.fixed(&urow, 30);
    parent.fixed(&prow, 30);
    parent.fixed(&pad, 1);
    parent.fixed(&brow, 30);
    parent.fixed(&b, 30);
}

fn middle_panel(parent: &mut group::Flex) {
    frame::Frame::default();

    let mut frame = frame::Frame::default().with_label("Image");
    frame.set_frame(enums::FrameType::BorderBox);
    frame.set_color(enums::Color::from_rgb(0, 200, 0));
    let spacer = frame::Frame::default();

    let mut bp = group::Flex::default().column();
    buttons_panel(&mut bp);
    bp.end();

    frame::Frame::default();

    parent.fixed(&frame, 200);
    parent.fixed(&spacer, 10);
    parent.fixed(&bp, 300);
}

fn main_panel(parent: &mut group::Flex) {
    frame::Frame::default();

    let mut mp = group::Flex::default().row();
    middle_panel(&mut mp);
    mp.end();

    frame::Frame::default();

    parent.fixed(&mp, 200);
}

fn create_button(caption: &str) -> button::Button {
    let mut btn = button::Button::default().with_label(caption);
    btn.set_color(enums::Color::from_rgb(225, 225, 225));
    btn
}
