use fltk::{enums::*, prelude::*, *};

fn tab_close_cb(g: &mut impl GroupExt) {
    if app::callback_reason() == CallbackReason::Closed {
        let mut parent = g.parent().unwrap();
        parent.remove(g);
        app::redraw();
    }
}

fn main() {
    let app = app::App::default();
    let mut win = window::Window::default().with_size(800, 600);
    let row = group::Flex::default_fill().row();
    let mut tabs = group::Tabs::default();
    tabs.set_tab_align(enums::Align::Right);
    tabs.handle_overflow(group::TabsOverflow::Compress);
    // first tab
    {
        let mut col1 = group::Flex::default().with_label("\t\ttab1").column();
        col1.set_trigger(CallbackTrigger::Closed);
        col1.set_callback(tab_close_cb);
        // widgets
        col1.end();
    }
    // end first tab

    // second tab
    {
        let mut col2 = group::Flex::default().with_label("\t\ttab2").column();
        col2.set_trigger(CallbackTrigger::Closed);
        col2.set_callback(tab_close_cb);
        // widgets
        col2.end();
    }
    // end second tab
    tabs.end();
    tabs.auto_layout();
    row.end();
    win.end();
    win.show();
    app.run().unwrap();
}
