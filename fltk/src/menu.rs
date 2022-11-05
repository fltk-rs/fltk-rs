use crate::enums::{Color, Font, LabelType};
use crate::prelude::*;
use crate::utils::FlString;
use fltk_sys::menu::*;
use std::{
    ffi::{CStr, CString},
    mem,
    os::raw,
};

/// Creates a menu bar
#[derive(Debug)]
pub struct MenuBar {
    inner: *mut Fl_Menu_Bar,
    tracker: *mut fltk_sys::fl::Fl_Widget_Tracker,
    is_derived: bool,
}

crate::macros::widget::impl_widget_ext!(MenuBar, Fl_Menu_Bar);
crate::macros::widget::impl_widget_base!(MenuBar, Fl_Menu_Bar);
crate::macros::menu::impl_menu_ext!(MenuBar, Fl_Menu_Bar);

/// Creates a menu button
#[derive(Debug)]
pub struct MenuButton {
    inner: *mut Fl_Menu_Button,
    tracker: *mut fltk_sys::fl::Fl_Widget_Tracker,
    is_derived: bool,
}

crate::macros::widget::impl_widget_ext!(MenuButton, Fl_Menu_Button);
crate::macros::widget::impl_widget_base!(MenuButton, Fl_Menu_Button);
crate::macros::menu::impl_menu_ext!(MenuButton, Fl_Menu_Button);

/// Defines the menu button types, which can be changed dynamically using the `set_type()`.
#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum MenuButtonType {
    /// pops up with the mouse 1st button.
    Popup1 = 1,
    /// pops up with the mouse 2nd button.
    Popup2,
    /// pops up with the mouse 1st or 2nd buttons.
    Popup12,
    /// pops up with the mouse 3rd button.
    Popup3,
    /// pops up with the mouse 1st or 3rd buttons.
    Popup13,
    /// pops up with the mouse 2nd or 3rd buttons.
    Popup23,
    /// pops up with any mouse button.
    Popup123,
}

crate::macros::widget::impl_widget_type!(MenuButtonType);

impl MenuButton {
    /// Act exactly as though the user clicked the button or typed the shortcut key
    pub fn popup(&self) -> Option<MenuItem> {
        assert!(!self.was_deleted());
        unsafe {
            let ptr = Fl_Menu_Button_popup(self.inner);
            if ptr.is_null() {
                None
            } else {
                let item = MenuItem {
                    inner: ptr as *mut Fl_Menu_Item,
                    size: Fl_Menu_Item_children(ptr),
                };
                Some(item)
            }
        }
    }
}

/// Creates a menu choice
///
/// The [`set_frame`](crate::prelude::WidgetExt::set_frame) method styles the
/// dropdown menu. `Choice` does not expose it's uderlying widget (a
/// [`DownBox`](crate::enums::FrameType::DownBox)). It can only be changed
/// via the app scheme or by globally changin the draw function of
/// [`DownBox`](crate::enums::FrameType::DownBox):
///
/// ```rust,no_run
///use fltk::{enums::*, prelude::*, *};
///
///fn my_down_box(x: i32, y: i32, w: i32, h: i32, col: Color) {
///    draw::draw_rect_fill(x, y, w, h, Color::Red);
///    draw::draw_rect_fill(x + 1, y + 1, w - 2, h - 2, Color::BackGround2); // change values to change thickness
///}
///
///fn main() {
///     let a = app::App::default();
///     app::set_frame_type_cb(FrameType::DownBox, my_down_box, 0, 0, 0, 0);
///     let mut win = window::Window::new(100, 100, 400, 300, None);
///     win.set_color(Color::from_rgb(211, 211, 211));
///     let mut inp = input::Input::new(50, 10, 100, 30, None); // would work for any widget which has a DownBox frame type
///     let mut choice = menu::Choice::new(50, 100, 100, 30, None);
///     choice.add_choice("Choice 1| Choice 2| choice 3");
///     win.end();
///     win.show();
///     a.run().unwrap();
///}
///```
///
/// For more extensive options see the `custom_choice` example.
#[derive(Debug)]
pub struct Choice {
    inner: *mut Fl_Choice,
    tracker: *mut fltk_sys::fl::Fl_Widget_Tracker,
    is_derived: bool,
}

crate::macros::widget::impl_widget_ext!(Choice, Fl_Choice);
crate::macros::widget::impl_widget_base!(Choice, Fl_Choice);
crate::macros::menu::impl_menu_ext!(Choice, Fl_Choice);

/// Creates a macOS system menu bar on macOS and a normal menu bar on other systems
#[derive(Debug)]
pub struct SysMenuBar {
    inner: *mut Fl_Sys_Menu_Bar,
    tracker: *mut fltk_sys::fl::Fl_Widget_Tracker,
    is_derived: bool,
}

crate::macros::widget::impl_widget_ext!(SysMenuBar, Fl_Sys_Menu_Bar);
crate::macros::widget::impl_widget_base!(SysMenuBar, Fl_Sys_Menu_Bar);
crate::macros::menu::impl_menu_ext!(SysMenuBar, Fl_Sys_Menu_Bar);

/// Creates a menu item
#[derive(Debug, Clone)]
pub struct MenuItem {
    inner: *mut Fl_Menu_Item,
    size: i32,
}

/// Defines the menu flag for any added menu items using the add() method
#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum MenuFlag {
    /// Normal item
    Normal = 0,
    /// Inactive item
    Inactive = 1,
    /// Item is a checkbox toggle (shows checkbox for on/off state)
    Toggle = 2,
    /// The on/off state for checkbox/radio buttons (if set, state is 'on')
    Value = 4,
    /// Item is a radio button
    Radio = 8,
    /// Invisible item
    Invisible = 0x10,
    /// Indicates user_data() is a pointer to another menu array (unused with Rust)
    SubmenuPointer = 0x20,
    /// Menu item is a submenu
    Submenu = 0x40,
    /// Menu divider
    MenuDivider = 0x80,
    /// Horizontal menu (actually reserved for future use)
    MenuHorizontal = 0x100,
}

impl MenuItem {
    /// Initializes a new menu item
    pub fn new(choices: &[&'static str]) -> MenuItem {
        unsafe {
            let sz = choices.len();
            let mut temp: Vec<*mut raw::c_char> = vec![];
            for &choice in choices {
                let c = CString::safe_new(choice);
                temp.push(c.into_raw());
            }
            let item_ptr = Fl_Menu_Item_new(temp.as_ptr() as *mut *mut raw::c_char, sz as i32);
            assert!(!item_ptr.is_null());
            MenuItem {
                inner: item_ptr,
                size: choices.len() as i32,
            }
        }
    }

    /// Creates a popup menu at the specified coordinates and returns its choice
    pub fn popup(&self, x: i32, y: i32) -> Option<MenuItem> {
        assert!(!self.was_deleted());
        unsafe {
            let item = Fl_Menu_Item_popup(self.inner, x, y);
            if item.is_null() {
                None
            } else {
                let item = MenuItem {
                    inner: item as *mut Fl_Menu_Item,
                    size: Fl_Menu_Item_children(item),
                };
                Some(item)
            }
        }
    }

    /// Returns the label of the menu item
    pub fn label(&self) -> Option<String> {
        assert!(!self.was_deleted());
        unsafe {
            let label_ptr = Fl_Menu_Item_label(self.inner);
            if label_ptr.is_null() {
                return None;
            }
            Some(
                CStr::from_ptr(label_ptr as *mut raw::c_char)
                    .to_string_lossy()
                    .to_string(),
            )
        }
    }

    /// Sets the label of the menu item
    pub fn set_label(&mut self, txt: &str) {
        assert!(!self.was_deleted());
        unsafe {
            let txt = CString::safe_new(txt);
            Fl_Menu_Item_set_label(self.inner, txt.into_raw());
        }
    }

    /// Returns the label type of the menu item
    pub fn label_type(&self) -> LabelType {
        assert!(!self.was_deleted());
        unsafe { mem::transmute(Fl_Menu_Item_label_type(self.inner)) }
    }

    /// Sets the label type of the menu item
    pub fn set_label_type(&mut self, typ: LabelType) {
        assert!(!self.was_deleted());
        unsafe {
            Fl_Menu_Item_set_label_type(self.inner, typ as i32);
        }
    }

    /// Returns the label color of the menu item
    pub fn label_color(&self) -> Color {
        assert!(!self.was_deleted());
        unsafe { mem::transmute(Fl_Menu_Item_label_color(self.inner)) }
    }

    /// Sets the label color of the menu item
    pub fn set_label_color(&mut self, color: Color) {
        assert!(!self.was_deleted());
        unsafe { Fl_Menu_Item_set_label_color(self.inner, color.bits() as u32) }
    }

    /// Returns the label font of the menu item
    pub fn label_font(&self) -> Font {
        assert!(!self.was_deleted());
        unsafe { mem::transmute(Fl_Menu_Item_label_font(self.inner)) }
    }

    /// Sets the label font of the menu item
    pub fn set_label_font(&mut self, font: Font) {
        assert!(!self.was_deleted());
        unsafe { Fl_Menu_Item_set_label_font(self.inner, font.bits() as i32) }
    }

    /// Returns the label size of the menu item
    pub fn label_size(&self) -> i32 {
        assert!(!self.was_deleted());
        unsafe { Fl_Menu_Item_label_size(self.inner) as i32 }
    }

    /// Sets the label size of the menu item
    pub fn set_label_size(&mut self, sz: i32) {
        assert!(!self.was_deleted());
        let sz = if sz < 1 { 1 } else { sz };
        unsafe { Fl_Menu_Item_set_label_size(self.inner, sz) }
    }

    /// Returns the value of the menu item
    pub fn value(&self) -> bool {
        assert!(!self.was_deleted());
        unsafe { Fl_Menu_Item_value(self.inner) != 0 }
    }

    /// Sets the menu item
    pub fn set(&mut self) {
        assert!(!self.was_deleted());
        unsafe { Fl_Menu_Item_set(self.inner) }
    }

    /// Turns the check or radio item "off" for the menu item
    pub fn clear(&mut self) {
        assert!(!self.was_deleted());
        unsafe { Fl_Menu_Item_clear(self.inner) }
    }

    /// Returns whether the menu item is visible or not
    pub fn visible(&self) -> bool {
        assert!(!self.was_deleted());
        unsafe { Fl_Menu_Item_visible(self.inner) != 0 }
    }

    /// Returns whether the menu item is active
    pub fn active(&self) -> bool {
        assert!(!self.was_deleted());
        unsafe { Fl_Menu_Item_active(self.inner) != 0 }
    }

    /// Activates the menu item
    pub fn activate(&mut self) {
        assert!(!self.was_deleted());
        unsafe { Fl_Menu_Item_activate(self.inner) }
    }

    /// Deactivates the menu item
    pub fn deactivate(&mut self) {
        assert!(!self.was_deleted());
        unsafe { Fl_Menu_Item_deactivate(self.inner) }
    }

    /// Returns whether a menu item is a submenu
    pub fn is_submenu(&self) -> bool {
        assert!(!self.was_deleted());
        unsafe { Fl_Menu_Item_submenu(self.inner) != 0 }
    }

    /// Returns whether a menu item is a checkbox
    pub fn is_checkbox(&self) -> bool {
        assert!(!self.was_deleted());
        unsafe { Fl_Menu_Item_checkbox(self.inner) != 0 }
    }

    /// Returns whether a menu item is a radio item
    pub fn is_radio(&self) -> bool {
        assert!(!self.was_deleted());
        unsafe { Fl_Menu_Item_radio(self.inner) != 0 }
    }

    /// Shows the menu item
    pub fn show(&mut self) {
        assert!(!self.was_deleted());
        unsafe { Fl_Menu_Item_show(self.inner) }
    }

    /// Hides the menu item
    pub fn hide(&mut self) {
        assert!(!self.was_deleted());
        unsafe { Fl_Menu_Item_hide(self.inner) }
    }

    /// Get the next menu item skipping submenus
    pub fn next(&self, idx: i32) -> Option<MenuItem> {
        assert!(!self.was_deleted());
        unsafe {
            let ptr = Fl_Menu_Item_next(self.inner, idx as i32);
            if ptr.is_null() {
                return None;
            }
            let label_ptr = Fl_Menu_Item_label(ptr);
            if label_ptr.is_null() {
                return None;
            }
            Some(MenuItem {
                inner: ptr,
                size: Fl_Menu_Item_children(ptr),
            })
        }
    }

    /// Get children of `MenuItem`
    pub fn children(&self) -> i32 {
        unsafe { Fl_Menu_Item_children(self.inner) }
    }

    /// Get the submenu count
    pub fn submenus(&self) -> i32 {
        let mut i = 0;
        while let Some(_item) = self.next(i) {
            i += 1;
        }
        i
    }

    /// Get the size of the MenuItem
    pub fn size(&self) -> i32 {
        self.size
    }

    /// Get the menu item at `idx`
    pub fn at(&self, idx: i32) -> Option<MenuItem> {
        assert!(idx < self.size);
        unsafe {
            let ptr = Fl_Menu_Item_at(self.inner, idx);
            if ptr.is_null() {
                None
            } else {
                Some(MenuItem {
                    inner: ptr as _,
                    size: Fl_Menu_Item_children(ptr),
                })
            }
        }
    }

    /// Get the user data
    /// # Safety
    /// Can return multiple mutable instances of the user data, which has a different lifetime than the object
    pub unsafe fn user_data(&self) -> Option<Box<dyn FnMut()>> {
        let ptr = Fl_Menu_Item_user_data(self.inner);
        if ptr.is_null() {
            None
        } else {
            let x = ptr as *mut Box<dyn FnMut()>;
            let x = Box::from_raw(x);
            Fl_Menu_Item_set_callback(self.inner, None, std::ptr::null_mut());
            Some(*x)
        }
    }

    /// Set a callback for the menu item
    pub fn set_callback<F: FnMut(&mut Self) + 'static>(&mut self, cb: F) {
        assert!(!self.was_deleted());
        unsafe {
            unsafe extern "C" fn shim(wid: *mut fltk_sys::menu::Fl_Widget, data: *mut raw::c_void) {
                let mut wid = crate::widget::Widget::from_widget_ptr(wid as *mut _);
                let a: *mut Box<dyn FnMut(&mut crate::widget::Widget)> =
                    data as *mut Box<dyn FnMut(&mut crate::widget::Widget)>;
                let f: &mut (dyn FnMut(&mut crate::widget::Widget)) = &mut **a;
                let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| f(&mut wid)));
            }
            let _old_data = self.user_data();
            let a: *mut Box<dyn FnMut(&mut Self)> = Box::into_raw(Box::new(Box::new(cb)));
            let data: *mut raw::c_void = a as *mut std::ffi::c_void;
            let callback: fltk_sys::menu::Fl_Callback = Some(shim);
            Fl_Menu_Item_set_callback(self.inner, callback, data);
        }
    }

    /// Use a sender to send a message during callback
    pub fn emit<T: 'static + Clone + Send + Sync>(
        &mut self,
        sender: crate::app::Sender<T>,
        msg: T,
    ) {
        self.set_callback(move |_| sender.send(msg.clone()));
    }

    /// Check if a menu item was deleted
    pub fn was_deleted(&self) -> bool {
        self.inner.is_null()
    }

    /// Draw a box around the menu item.
    /// Requires the call to be made inside a MenuExt-implementing widget's own draw method
    pub fn draw<M: MenuExt>(&self, x: i32, y: i32, w: i32, h: i32, menu: &M, selected: bool) {
        assert!(!self.was_deleted());
        unsafe {
            Fl_Menu_Item_draw(
                self.inner,
                x,
                y,
                w,
                h,
                menu.as_widget_ptr() as _,
                selected as i32,
            )
        }
    }

    /// Measure the width and height of a menu item
    pub fn measure(&self) -> (i32, i32) {
        assert!(!self.was_deleted());
        let mut h = 0;
        let ret = unsafe { Fl_Menu_Item_measure(self.inner, &mut h as _, std::ptr::null()) };
        (ret, h)
    }

    /// Set the image of the menu item
    /// # Safety
    /// Trying to add a label after adding an image might lead to undefined behavior
    #[doc(hidden)]
    pub unsafe fn set_image<I: ImageExt>(&mut self, mut image: I) {
        assert!(!self.was_deleted());
        assert!(!image.was_deleted());
        image.increment_arc();
        Fl_Menu_Item_image(self.inner, image.as_image_ptr() as _)
    }

    /**
        Add an image to a menu item
        ```rust,no_run
        use fltk::{prelude::*, *};
        const PXM: &[&str] = &[
            "13 11 3 1",
            "   c None",
            "x  c #d8d833",
            "@  c #808011",
            "             ",
            "     @@@@    ",
            "    @xxxx@   ",
            "@@@@@xxxx@@  ",
            "@xxxxxxxxx@  ",
            "@xxxxxxxxx@  ",
            "@xxxxxxxxx@  ",
            "@xxxxxxxxx@  ",
            "@xxxxxxxxx@  ",
            "@xxxxxxxxx@  ",
            "@@@@@@@@@@@  "
        ];
        let image = image::Pixmap::new(PXM).unwrap();
        let mut menu = menu::MenuBar::default();
        menu.add(
            "&File/Open...\t",
            enums::Shortcut::Ctrl | 'o',
            menu::MenuFlag::Normal,
            |_| println!("Opened file!"),
        );
        if let Some(mut item) = menu.find_item("&File/Open...\t") {
            item.add_image(Some(image), true);
        }
        ```
    */
    pub fn add_image<I: ImageExt>(&mut self, image: Option<I>, on_left: bool) {
        assert!(!self.was_deleted());
        unsafe {
            if let Some(image) = image {
                assert!(!image.was_deleted());
                Fl_Menu_Item_add_image(self.inner, image.as_image_ptr() as _, on_left as i32)
            } else {
                Fl_Menu_Item_add_image(self.inner, std::ptr::null_mut(), on_left as i32)
            }
        }
    }

    /// Add a menu item
    pub fn add<F: FnMut(&mut Choice) + 'static>(
        &mut self,
        name: &str,
        shortcut: crate::enums::Shortcut,
        flag: MenuFlag,
        cb: F,
    ) -> i32 {
        assert!(!self.was_deleted());
        let temp = CString::safe_new(name);
        unsafe {
            unsafe extern "C" fn shim(wid: *mut Fl_Widget, data: *mut std::os::raw::c_void) {
                let mut wid = crate::widget::Widget::from_widget_ptr(wid as *mut _);
                let a: *mut Box<dyn FnMut(&mut crate::widget::Widget)> =
                    data as *mut Box<dyn FnMut(&mut crate::widget::Widget)>;
                let f: &mut (dyn FnMut(&mut crate::widget::Widget)) = &mut **a;
                let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| f(&mut wid)));
            }
            let a: *mut Box<dyn FnMut(&mut Choice)> = Box::into_raw(Box::new(Box::new(cb)));
            let data: *mut std::os::raw::c_void = a as *mut std::os::raw::c_void;
            let callback: Fl_Callback = Some(shim);
            Fl_Menu_Item_add(
                self.inner,
                temp.as_ptr(),
                shortcut.bits() as i32,
                callback,
                data,
                flag as i32,
            )
        }
    }

    /// Insert a menu item
    pub fn insert<F: FnMut(&mut Choice) + 'static>(
        &mut self,
        idx: i32,
        name: &str,
        shortcut: crate::enums::Shortcut,
        flag: MenuFlag,
        cb: F,
    ) -> i32 {
        assert!(!self.was_deleted());
        let temp = CString::safe_new(name);
        unsafe {
            unsafe extern "C" fn shim(wid: *mut Fl_Widget, data: *mut std::os::raw::c_void) {
                let mut wid = crate::widget::Widget::from_widget_ptr(wid as *mut _);
                let a: *mut Box<dyn FnMut(&mut crate::widget::Widget)> =
                    data as *mut Box<dyn FnMut(&mut crate::widget::Widget)>;
                let f: &mut (dyn FnMut(&mut crate::widget::Widget)) = &mut **a;
                let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| f(&mut wid)));
            }
            let a: *mut Box<dyn FnMut(&mut Choice)> = Box::into_raw(Box::new(Box::new(cb)));
            let data: *mut std::os::raw::c_void = a as *mut std::os::raw::c_void;
            let callback: Fl_Callback = Some(shim);
            Fl_Menu_Item_insert(
                self.inner,
                idx as i32,
                temp.as_ptr(),
                shortcut.bits() as i32,
                callback,
                data,
                flag as i32,
            )
        }
    }

    /// Add a menu item along with an emit (sender and message).
    pub fn add_emit<T: 'static + Clone + Send + Sync>(
        &mut self,
        label: &str,
        shortcut: crate::enums::Shortcut,
        flag: MenuFlag,
        sender: crate::app::Sender<T>,
        msg: T,
    ) -> i32 {
        self.add(label, shortcut, flag, move |_| sender.send(msg.clone()))
    }

    /// Insert a menu item along with an emit (sender and message).
    pub fn insert_emit<T: 'static + Clone + Send + Sync>(
        &mut self,
        idx: i32,
        label: &str,
        shortcut: crate::enums::Shortcut,
        flag: MenuFlag,
        sender: crate::app::Sender<T>,
        msg: T,
    ) -> i32 {
        self.insert(idx, label, shortcut, flag, move |_| {
            sender.send(msg.clone())
        })
    }
}

/// Delete a menu item
/// # Safety
/// The wrapper can't assure use after free when manually deleting a menu item
pub unsafe fn delete_menu_item(item: MenuItem) {
    Fl_Menu_Item_delete(item.inner)
}

unsafe impl Send for MenuItem {}

unsafe impl Sync for MenuItem {}

impl PartialEq for MenuItem {
    fn eq(&self, other: &Self) -> bool {
        self.inner == other.inner
    }
}

impl Eq for MenuItem {}

impl IntoIterator for MenuItem {
    type Item = MenuItem;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        let mut v: Vec<MenuItem> = vec![];
        let mut i = 0;
        while let Some(item) = self.at(i) {
            v.push(item);
            i += 1;
        }
        v.into_iter()
    }
}

/// Set a callback for the "About" item of the system application menu on macOS.
pub fn mac_set_about<F: FnMut() + 'static>(cb: F) {
    unsafe {
        unsafe extern "C" fn shim(_wid: *mut fltk_sys::menu::Fl_Widget, data: *mut raw::c_void) {
            let a: *mut Box<dyn FnMut()> = data as *mut Box<dyn FnMut()>;
            let f: &mut (dyn FnMut()) = &mut **a;
            let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(f));
        }
        let a: *mut Box<dyn FnMut()> = Box::into_raw(Box::new(Box::new(cb)));
        let data: *mut raw::c_void = a as *mut std::ffi::c_void;
        let callback: fltk_sys::menu::Fl_Callback = Some(shim);
        Fl_mac_set_about(callback, data, 0);
    }
}
