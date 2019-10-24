 #![allow(non_camel_case_types)]
 #![allow(dead_code)]

include!(concat!("fl.rs"));

include!(concat!("window.rs"));

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
