use fltk::*;
use std::cell::RefCell;
use std::rc::Rc;
use std::thread;
use std::time::Duration;

fn main() {
    let app = app::App::default();
    let mut wind = window::Window::default()
        .with_size(800, 600)
        .center_screen()
        .with_label("Pong!");
    let mut ball = valuator::FillDial::new(0, 0, 40, 40, "");
    ball.set_color(Color::White);
    wind.set_color(Color::Black);
    wind.end();
    wind.show();

    let paddle_pos = Rc::from(RefCell::from(320));
    let paddle_c = paddle_pos.clone();
    wind.draw(Box::new(move || {
        draw::set_draw_color(Color::White);
        draw::draw_rectf(*paddle_c.borrow(), 540, 160, 20);
    }));

    let ball_x = &mut 0;
    let ball_y = &mut 0;
    let paddle_c = paddle_pos.clone();
    let mut wind_c = wind.clone();

    wind.handle(Box::new(move |ev| {
        match ev {
            enums::Event::KeyDown => {
                match app::event_key() {
                    enums::Key::Right => *paddle_pos.borrow_mut() += 30,
                    enums::Key::Left => *paddle_pos.borrow_mut() -= 30,
                    _ => (),
                }
                wind_c.redraw();
                true
            }
            _ => false,
        }
    }));

    let x_positive = &mut true;
    let y_positive = &mut true;
    while app.wait().unwrap() {
        if *x_positive {
            *ball_x += 5;
        } else {
            *ball_x -= 5;
        }
        if *y_positive {
            *ball_y += 5;
        } else {
            *ball_y -= 5;
        }
        if *ball_y == 540 - 40
            && (*ball_x > *paddle_c.borrow() && *ball_x < *paddle_c.borrow() + 160)
        {
            *y_positive = false;
        }
        if *ball_y == 0 {
            *y_positive = true;
        }
        if *ball_x == 800 - 40 {
            *x_positive = false;
        }
        if *ball_x == 0 {
            *x_positive = true;
        }
        if *ball_y > 600 {
            app.quit();
        }
        ball.resize(*ball_x, *ball_y, 40, 40);
        wind.redraw();
        thread::sleep(Duration::from_millis(16));
    }
}
