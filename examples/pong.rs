use fltk::*;
use std::cell::RefCell;
use std::rc::Rc;
use std::thread;
use std::time::Duration;

#[repr(i32)]
#[derive(Copy, Clone)]
enum Direction {
    Positive = 1,
    Negative = -1,
}

struct Ball {
    wid: valuator::FillDial,
    pos: (i32, i32),       // x and y positions
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
    ball.wid.set_color(Color::White);
    wind.set_color(Color::Black);
    wind.end();
    wind.show();

    let paddle_pos = Rc::from(RefCell::from(320));
    let paddle_c = paddle_pos.clone();
    wind.draw(Box::new(move || {
        draw::set_draw_color(Color::White);
        draw::draw_rectf(*paddle_c.borrow(), 540, 160, 20);
    }));

    let paddle_c = paddle_pos.clone();
    wind.handle(Box::new(move |ev| {
        match ev {
            enums::Event::Move => {
                *paddle_pos.borrow_mut() = app::event_coords().0 - 80; // Mouse's x position
                true
            }
            _ => false,
        }
    }));

    while app.wait().unwrap() {
        ball.pos.0 += 10 * ball.dir.0 as i32;
        ball.pos.1 += 10 * ball.dir.1 as i32;
        if ball.pos.1 == 540 - 40
            && (ball.pos.0 > *paddle_c.borrow() - 40 && ball.pos.0 < *paddle_c.borrow() + 160)
        {
            ball.dir.1 = Direction::Negative;
        }
        if ball.pos.1 == 0 {
            ball.dir.1 = Direction::Positive;
        }
        if ball.pos.0 == 800 - 40 {
            ball.dir.0 = Direction::Negative;
        }
        if ball.pos.0 == 0 {
            ball.dir.0 = Direction::Positive;
        }
        if ball.pos.1 > 600 {
            ball.pos = (0, 0);
            ball.dir = (Direction::Positive, Direction::Positive);
        }
        ball.wid.resize(ball.pos.0, ball.pos.1, 40, 40);
        wind.redraw();
        thread::sleep(Duration::from_millis(16));
    }
}
