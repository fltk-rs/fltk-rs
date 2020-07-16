use fltk::{app::*, window::*};

fn main() {
    let app = App::default();
    let mut wind = Window::new(100, 100, 400, 300, "Hello from rust");

    wind.end();
    wind.show();

    unsafe {
        let mut h = wind.raw_handle();
        let mut w = Window::from_raw_handle(&h).unwrap();

        // Or to get the raw handle from an external source use RawWindowHandle::from_external_handle
        // Note:
        // For xid on X11, the u32 value needs to be boxed if using from_external_handle
        // i.e:
        // let handle = some_xid_getting_function();
        // let handle = Box::new(handle);
        // let t = RawWindowHandle::from_external_handle(Box::into_raw(handle) as *mut std::os::raw::c_void);
        // let mut w = Window::from_raw_handle(&t).unwrap();

        w.set_color(Color::White);
    }

    app.run().unwrap();
}
