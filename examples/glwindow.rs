use fltk::{app::*, gl::*, window::*};
use std::cell::RefCell;
use std::rc::Rc;

const W: i32 = 600;
const H: i32 = 400;

pub fn main() {
    let app = App::default().set_scheme(AppScheme::Gleam);
    let mut wind = GlWindow::new(100, 100, W, H, "Rotate me!");

    wind.end();
    wind.show();

    let rotangle = Rc::from(RefCell::from(0.0));
    let rotangle_rc = rotangle.clone();

    wind.draw(Box::new(move || draw_triangle(&rotangle_rc.borrow())));

    let (s, r) = channel::<(i32, i32)>();

    wind.handle(Box::new(move |ev| {
        match ev {
            Event::Drag => {
                s.send(event_coords());
                true
            },
            _ => false,
        }
    }));

    while app.wait().unwrap() {
        match r.recv() {
            Some(coords) => { 
                let rand: f32 = ((coords.0 - W/2) * (coords.1 - H/2) / 360) as f32;
                *rotangle.borrow_mut() += rand;
                wind.redraw();
            },
            None => (),
        }
    }
}

fn draw_triangle(rotangle: &f32) {
    unsafe {
        glClear(GL_COLOR_BUFFER_BIT | GL_DEPTH_BUFFER_BIT);
        glMatrixMode(GL_PROJECTION);
        glLoadIdentity();
        glViewport(0, 0, W, H);
        gluPerspective(45.0, (W as f32 / H as f32).into(), 1.0, 10.0);
        glTranslatef(0.0, 0.0, -5.0);
        glMatrixMode(GL_MODELVIEW);
        glLoadIdentity();
        glRotatef(*rotangle, 1.0, 1.0, 0.0);
        glColor3f(1.0, 0.0, 0.0);
        glBegin(GL_POLYGON);
        glVertex3f(0.0, 1.0, 0.0);
        glVertex3f(1.0, -1.0, 1.0);
        glVertex3f(-1.0, -1.0, 1.0);
        glEnd();
        glColor3f(0.0, 1.0, 0.0);
        glBegin(GL_POLYGON);
        glVertex3f(0.0, 1.0, 0.0);
        glVertex3f(0.0, -1.0, -1.0);
        glVertex3f(1.0, -1.0, 1.0);
        glEnd();
        glColor3f(0.0, 0.0, 1.0);
        glBegin(GL_POLYGON);
        glVertex3f(0.0, 1.0, 0.0);
        glVertex3f(-1.0, -1.0, 1.0);
        glVertex3f(0.0, -1.0, -1.0);
        glEnd();
        glColor3f(1.0, 0.0, 0.0);
        glBegin(GL_POLYGON);
        glVertex3f(1.0, -1.0, 1.0);
        glVertex3f(0.0, -1.0, -1.0);
        glVertex3f(-1.0, -1.0, 1.0);
        glEnd();
        glLoadIdentity();
        glRasterPos2f(-3.0, -2.0);
    }
}


