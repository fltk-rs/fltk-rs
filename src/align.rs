#[repr(i32)]
#[derive(Debug, Copy, Clone)]
pub enum Align {
    AlignCenter = 0,
    AlignTop = 1,
    AlignBottom = 2,
    AlignLeft = 4,
    AlignRight = 8,
}
