use fltk::{prelude::*, *};
use std::cell::RefCell;
use std::rc::Rc;

#[repr(i32)]
#[derive(Copy, Clone)]
enum Direction {
    Positive = 1,
    Negative = -1,
}

struct Ball {
    wid: frame::Frame,
    pos: (i32, i32),             // x and y positions
    dir: (Direction, Direction), // x and y directions
}

impl Ball {
    pub fn new(w: i32, h: i32) -> Self {
        let mut wid = frame::Frame::new(0, 0, w, h, None);
        wid.set_frame(enums::FrameType::OFlatBox);
        wid.set_color(enums::Color::White);
        Self {
            wid,
            pos: (0, 0),
            dir: (Direction::Positive, Direction::Positive),
        }
    }
}

fn main() {
    let app = app::App::default();
    let mut wind = window::Window::default()
        .with_size(800, 600)
        .center_screen()
        .with_label("Pong!");
    let mut ball = Ball::new(40, 40);
    ball.wid.set_color(enums::Color::White);
    wind.set_color(enums::Color::Black);
    wind.end();
    wind.show();

    let paddle_pos = Rc::from(RefCell::from(320)); // paddle's starting x position

    // This is called whenever the window is drawn and redrawn (in the event loop)
    wind.draw({
        let paddle_pos = paddle_pos.clone();
        move |_| {
            draw::set_draw_color(enums::Color::White);
            draw::draw_rectf(*paddle_pos.borrow(), 540, 160, 20);
        }
    });

    wind.handle({
        let paddle_pos = paddle_pos.clone();
        move |_, ev| {
            match ev {
                enums::Event::Move => {
                    // Mouse's x position relative to the paddle's center
                    *paddle_pos.borrow_mut() = app::event_coords().0 - 80;
                    true
                }
                _ => false,
            }
        }
    });

    app::add_idle3(move |_| {
        ball.pos.0 += 10 * ball.dir.0 as i32; // The increment in x position
        ball.pos.1 += 10 * ball.dir.1 as i32; // The increment in y position
        if ball.pos.1 == 540 - 40
            && (ball.pos.0 > *paddle_pos.borrow() - 40 && ball.pos.0 < *paddle_pos.borrow() + 160)
        {
            ball.dir.1 = Direction::Negative; // Reversal of motion when hitting the paddle
        }
        if ball.pos.1 == 0 {
            ball.dir.1 = Direction::Positive; // Reversal of motion when hitting the top border
        }
        if ball.pos.0 == 800 - 40 {
            ball.dir.0 = Direction::Negative; // Reversal of motion when hitting the right border
        }
        if ball.pos.0 == 0 {
            ball.dir.0 = Direction::Positive; // Reversal of motion when hitting the left border
        }
        if ball.pos.1 > 600 {
            // Resetting the ball position after it bypasses the paddle
            ball.pos = (0, 0);
            ball.dir = (Direction::Positive, Direction::Positive);
        }
        ball.wid.resize(ball.pos.0, ball.pos.1, 40, 40); // Moves the ball
        wind.redraw();
        // sleeps are necessary when calling redraw in the event loop
        app::sleep(0.016);
    });
    app.run().unwrap();
}
