pub trait Widget {
    fn new(x: i32, y: i32, width: i32, height: i32, title: &str) -> Self;
    fn set_label(&mut self, title: &str);
    fn redraw(&mut self);
    fn callback<W>(&mut self, cb: fn(&mut W))
    where
        W: Widget;
}
