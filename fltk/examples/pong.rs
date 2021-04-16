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
    wid: valuator::FillDial,
    pos: (i32, i32),             // x and y positions
    dir: (Direction, Direction), // x and y directions
}

fn main() {
    let app = app::App::default();
    let mut wind = window::Window::default()
        .with_size(800, 600)
        .center_screen()
        .with_label("Pong!");
    let mut ball = Ball {
        wid: valuator::FillDial::new(0, 0, 40, 40, ""),
        pos: (0, 0),
        dir: (Direction::Positive, Direction::Positive),
    };
    ball.wid.set_color(enums::Color::White);
    wind.set_color(enums::Color::Black);
    wind.end();
    wind.show();

    let paddle_pos = Rc::from(RefCell::from(320)); // paddle's starting x position
    let paddle_c = paddle_pos.clone();

    // This is called whenever the window is drawn and redrawn (in the event loop)
    wind.draw(move |_| {
        draw::set_draw_color(enums::Color::White);
        draw::draw_rectf(*paddle_c.borrow(), 540, 160, 20);
    });

    let paddle_c = paddle_pos.clone();
    wind.handle(move |_, ev| {
        match ev {
            enums::Event::Move => {
                // Mouse's x position relative to the paddle's center
                *paddle_pos.borrow_mut() = app::event_coords().0 - 80;
                true
            }
            _ => false,
        }
    });

    while app.wait() {
        ball.pos.0 += 10 * ball.dir.0 as i32; // The increment in x position
        ball.pos.1 += 10 * ball.dir.1 as i32; // The increment in y position
        if ball.pos.1 == 540 - 40
            && (ball.pos.0 > *paddle_c.borrow() - 40 && ball.pos.0 < *paddle_c.borrow() + 160)
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
    }
}
