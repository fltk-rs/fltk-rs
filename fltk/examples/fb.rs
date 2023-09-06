use fltk::{app, draw, frame, prelude::*, window::Window};

const WIDTH: i32 = 600;
const HEIGHT: i32 = 400;
const CIRCLE_RADIUS: i16 = 64;

struct World {
    circle_x: i16,
    circle_y: i16,
    velocity_x: i16,
    velocity_y: i16,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = app::App::default();
    let mut win = Window::default()
        .with_size(WIDTH, HEIGHT)
        .with_label("Framebuffer");
    let mut frame = frame::Frame::default_fill();
    win.end();
    win.make_resizable(true);
    win.show();

    let mut framebuf: Vec<u8> = vec![0; (WIDTH * HEIGHT * 4) as usize];
    let mut world = World::new();

    unsafe {
        draw::draw_rgba_nocopy(&mut frame, &framebuf);
    }

    app::add_idle3(move |_| {
        world.update();
        world.draw(&mut framebuf);
        // draw::draw_rgba(&mut frame, &framebuf).unwrap(); // A safe variant of draw_rgba_nocopy
        win.redraw();
        // sleeps are necessary when calling redraw in the event loop
        app::sleep(0.016);
    });

    app.run()?;
    Ok(())
}

impl World {
    fn new() -> Self {
        Self {
            circle_x: 300,
            circle_y: 200,
            velocity_x: 5,
            velocity_y: 5,
        }
    }

    fn update(&mut self) {
        if self.circle_x - CIRCLE_RADIUS <= 0 || self.circle_x + CIRCLE_RADIUS > WIDTH as i16 {
            self.velocity_x *= -1;
        }
        if self.circle_y - CIRCLE_RADIUS <= 0 || self.circle_y + CIRCLE_RADIUS > HEIGHT as i16 {
            self.velocity_y *= -1;
        }

        self.circle_x += self.velocity_x;
        self.circle_y += self.velocity_y;
    }

    fn draw(&self, frame: &mut [u8]) {
        for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
            let x = (i % WIDTH as usize) as i16;
            let y = (i / WIDTH as usize) as i16;
            let d = {
                let xd = x as i32 - self.circle_x as i32;
                let yd = y as i32 - self.circle_y as i32;
                ((xd.pow(2) + yd.pow(2)) as f64).sqrt().powi(2)
            };
            let inside_the_circle = d < (CIRCLE_RADIUS as f64).powi(2);

            let rgba = if inside_the_circle {
                [0xac, 0x00, 0xe6, 0xff]
            } else {
                [0x26, 0x00, 0x33, 0xff]
            };

            pixel.copy_from_slice(&rgba);
        }
    }
}
