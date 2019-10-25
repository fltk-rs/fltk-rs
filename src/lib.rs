pub mod fl;
pub mod window;
pub mod button;

use std::ffi;

pub type VoidPtr = *mut ffi::c_void;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
