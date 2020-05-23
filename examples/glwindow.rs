use fltk::{app::*, gl::*, window::*};

pub fn main() {
    let app = App::default().set_scheme(AppScheme::Gleam);
    let mut wind = GlWindow::new(100, 100, 800, 600, "GlWindow Example");

    wind.end();
    wind.show();

    let w = wind.width() as f32;
    let h = wind.height() as f32;
    let rotangle = 0.0;

    wind.draw(Box::new(move || {
        unsafe {
            glClear(GL_COLOR_BUFFER_BIT | GL_DEPTH_BUFFER_BIT);
            glMatrixMode(GL_PROJECTION);
            glLoadIdentity();
            glViewport(0, 0, w as i32, h as i32);
            gluPerspective(45.0, (w / h).into(), 1.0, 10.0);
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
            glColor3f(0.5, 0.5, 0.5);
            glBegin(GL_POLYGON);
            glVertex3f(1.0, -1.0, 1.0);
            glVertex3f(0.0, -1.0, -1.0);
            glVertex3f(-1.0, -1.0, 1.0);
            glEnd();
            glEnable(GL_DEPTH_TEST);
            glLoadIdentity();
            glRasterPos2f(-3.0, -2.0);
        }
    }));

    app.run().unwrap();
}
