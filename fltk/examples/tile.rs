use fltk::{enums::*, prelude::*, *};

fn main() {
    let app = app::App::default();
    let mut window = window::Window::default().with_size(300, 300);
    window.set_frame(FrameType::NoBox);
    window.make_resizable(true);

    let dx = 20;
    let dy = dx; // border width of resizable() - see below
    let tile = group::Tile::default_fill();

    // create the symmetrical resize box with dx and dy pixels distance, resp.
    // from the borders of the Fl_Tile widget before all other children
    let r = frame::Frame::new(
        tile.x() + dx,
        tile.y() + dy,
        tile.w() - 2 * dx,
        tile.h() - 2 * dy,
        None,
    );
    tile.resizable(&r);

    let mut box0 = frame::Frame::new(0, 0, 150, 150, "0");
    box0.set_frame(FrameType::DownBox);
    box0.set_color(Color::by_index(9));
    box0.set_label_size(36);
    box0.set_align(Align::Clip);

    let mut w1 = window::Window::new(150, 0, 150, 150, "1");
    w1.set_frame(FrameType::NoBox);
    let mut box1 = frame::Frame::new(0, 0, 150, 150, "1\nThis is a child window");
    box1.set_frame(FrameType::DownBox);
    box1.set_color(Color::by_index(19));
    box1.set_label_size(18);
    box1.set_align(Align::Clip | Align::Inside | Align::Wrap);
    w1.resizable(&box1);
    w1.end();

    let mut box2a = frame::Frame::new(0, 150, 70, 150, "2a");
    box2a.set_frame(FrameType::DownBox);
    box2a.set_color(Color::by_index(12));
    box2a.set_label_size(36);
    box2a.set_align(Align::Clip);

    let mut box2b = frame::Frame::new(70, 150, 80, 150, "2b");
    box2b.set_frame(FrameType::DownBox);
    box2b.set_color(Color::by_index(13));
    box2b.set_label_size(36);
    box2b.set_align(Align::Clip);

    let mut box3a = frame::Frame::new(150, 150, 150, 70, "3a");
    box3a.set_frame(FrameType::DownBox);
    box3a.set_color(Color::by_index(12));
    box3a.set_label_size(36);
    box3a.set_align(Align::Clip);

    let mut box3b = frame::Frame::new(150, 150 + 70, 150, 80, "3b");
    box3b.set_frame(FrameType::DownBox);
    box3b.set_color(Color::by_index(13));
    box3b.set_label_size(36);
    box3b.set_align(Align::Clip);

    tile.end();
    window.end();

    w1.show();
    window.show();

    app.run().unwrap();
}
