pub trait Widget {
    fn new(x: i32, y: i32, width: i32, height: i32, title: &str) -> Self;
}
