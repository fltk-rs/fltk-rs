 #![allow(non_camel_case_types)]
 #![allow(dead_code)]

pub mod fl;
pub mod widget;
pub mod group;
pub mod text;
pub mod window;
pub mod button;
pub mod frame;
pub mod input;
pub mod output;
pub mod menu;
pub mod dialog;
pub mod valuator;
pub mod browser;
pub mod image;
pub mod misc;


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
