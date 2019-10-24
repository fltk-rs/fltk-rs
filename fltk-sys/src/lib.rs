 #![allow(non_camel_case_types)]
 #![allow(dead_code)]

include!(concat!("bindings.rs"));

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
