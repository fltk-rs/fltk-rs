use fltk::{app::*, gl::*, window::*};
use std::{time::Duration, thread};

const W: i32 = 600;
const H: i32 = 400;

pub fn main() {
    let app = App::default().set_scheme(AppScheme::Gleam);
    let mut wind = GlWindow::new(100, 100, W, H, "GlWindow Example");
    wind.set_mode(Mode::Opengl3);

    wind.end();
    wind.show();

    wind.draw(Box::new(move || draw_triangle(0.0)));

    thread::spawn(move || {
        for i in 0..1000 {
            thread::sleep(Duration::from_millis(16));
            // Rotates the triangle at 60 fps
            wind.draw(Box::new(move || draw_triangle(i as f32)));
            wind.redraw();
        }
    });

    app.run().unwrap();
}

fn draw_triangle(rotangle: f32) {
    unsafe {
        glClear(GL_COLOR_BUFFER_BIT | GL_DEPTH_BUFFER_BIT);
        glMatrixMode(GL_PROJECTION);
        glLoadIdentity();
        glViewport(0, 0, W, H);
        gluPerspective(45.0, (W as f32/ H as f32).into(), 1.0, 10.0);
        glTranslatef(0.0, 0.0, -5.0);
        glMatrixMode(GL_MODELVIEW);
        glLoadIdentity();
        glRotatef(rotangle, 1.0, 0.0, 1.0);
        glRotatef(rotangle, 0.0, 1.0, 0.0);
        glRotatef(rotangle, 1.0, 1.0, 1.0);
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
