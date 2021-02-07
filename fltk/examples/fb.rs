use fltk::*;

fn main() {
    let app = app::App::default();
    let mut win = window::Window::default().with_size(500, 400);
    let mut frame = frame::Frame::default().size_of(&win);
    win.end();
    win.make_resizable(true);
    win.show();
    let mut fb: Vec<u8> = Vec::with_capacity(500 * 400 * 4);
    for _i in 0..fb.capacity() {
        fb.push(0xff); // red
        fb.push(0x11); // green
        fb.push(0x20); // blue
        fb.push(0xfa); // alpha
    }

    draw::draw_rgba(&mut frame, &fb).unwrap();

    app.run().unwrap();
}
