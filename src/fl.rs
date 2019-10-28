use std::mem;

#[repr(i32)]
#[derive(Debug, Copy, Clone)]
pub enum Event {
    NoEvent = 0,
    Push,
    Released,
    Enter,
    Leave,
    Drag,
    Focus,
    Unfocus,
    KeyDown,
    KeyUp,
    Close,
    Move,
    Shortcut,
    Deactivate,
    Activate,
    Hide,
    Show,
    Paste,
    SelectionClear,
    MouseWheel,
}

pub fn run() {
    unsafe {
        fltk_sys::fl::Fl_run();
    }
}

pub fn event() -> Event {
    unsafe {
        let x = fltk_sys::fl::Fl_event();
        let x: Event = mem::transmute(x);
        x
    }
}
