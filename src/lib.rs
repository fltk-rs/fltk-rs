pub mod app;
pub mod browser;
pub mod button;
pub mod dialog;
pub mod draw;
pub mod enums;
pub mod frame;
pub mod group;
pub mod image;
pub mod input;
pub mod menu;
pub mod misc;
pub mod output;
pub mod prelude;
pub mod text;
pub mod valuator;
pub mod widget;
pub mod window;

#[macro_use]
extern crate fltk_derive;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
