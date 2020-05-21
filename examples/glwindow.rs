// // This is an example showing basic usage of a GlWindow

// #![allow(non_camel_case_types)]
// #![allow(dead_code)]
// #![allow(non_upper_case_globals)]
//
// use fltk::{app::*, window::*};
// mod gl; // For aw gl calls just by running bindgen on the gl.h header
//
// pub fn main() {
//     let app = App::default().set_scheme(AppScheme::Gleam);
//     let mut wind = GlWindow::new(100, 100, 800, 600, "GlWindow Example");
//
//     wind.make_resizable(true);
//     wind.end();
//     wind.show();
//
//     let w = wind.width() as f32;
//     let h = wind.height() as f32;
//
//     wind.draw(Box::new(move || {
//         unsafe {
//             gl::glClear(gl::GL_COLOR_BUFFER_BIT);
//             gl::glColor3f(1.0, 1.0, 1.0);
//             gl::glBegin(gl::GL_LINE_STRIP); 
//             gl::glVertex2f(w, h); 
//             gl::glVertex2f(-w,-h);
//             gl::glEnd();
//             gl::glBegin(gl::GL_LINE_STRIP); 
//             gl::glVertex2f(w,-h); 
//             gl::glVertex2f(-w, h); 
//             gl::glEnd()
//         }
//     }));
//
//     app.run().unwrap();
// }

fn main() {}
