pub mod button;
pub mod dialog;
pub mod enums;
pub mod fl;
pub mod frame;
pub mod group;
pub mod input;
pub mod menu;
pub mod output;
pub mod prelude;
pub mod valuator;
pub mod widget;
pub mod window;
pub mod image;

#[macro_use]
extern crate fltk_derive;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let x = std::path::PathBuf::new("fdsg");
        let x = x.to_str().unwrap().as_ptr();
    }
}
