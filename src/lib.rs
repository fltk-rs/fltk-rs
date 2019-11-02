pub mod button;
pub mod fl;
pub mod frame;
pub mod group;
pub mod input;
pub mod output;
pub mod prelude;
pub mod widget;
pub mod window;
pub mod enums;
pub mod menu;
pub mod dialog;

#[macro_use]
extern crate fltk_derive;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
