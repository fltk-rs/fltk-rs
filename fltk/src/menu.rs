use crate::enums::{
    Align, CallbackTrigger, Color, Damage, Event, Font, FrameType, LabelType, Shortcut,
};
use crate::image::Image;
use crate::prelude::*;
use crate::utils::FlString;
use fltk_sys::menu::*;
use std::{
    ffi::{CStr, CString},
    mem,
    os::raw,
};

/// Creates a menu bar
#[derive(WidgetBase, WidgetExt, MenuExt, Debug)]
pub struct MenuBar {
    inner: *mut Fl_Menu_Bar,
    tracker: *mut fltk_sys::fl::Fl_Widget_Tracker,
}

/// Creates a menu button
#[derive(WidgetBase, WidgetExt, MenuExt, Debug)]
pub struct MenuButton {
    inner: *mut Fl_Menu_Button,
    tracker: *mut fltk_sys::fl::Fl_Widget_Tracker,
}

/// Creates a menu choice
#[derive(WidgetBase, WidgetExt, MenuExt, Debug)]
pub struct Choice {
    inner: *mut Fl_Choice,
    tracker: *mut fltk_sys::fl::Fl_Widget_Tracker,
}

/// Creates a macOS system menu bar on macOS and a normal menu bar on other systems
#[derive(WidgetBase, WidgetExt, MenuExt, Debug)]
pub struct SysMenuBar {
    inner: *mut Fl_Sys_Menu_Bar,
    tracker: *mut fltk_sys::fl::Fl_Widget_Tracker,
}

/// Creates a menu item
#[derive(Debug, Clone)]
pub struct MenuItem {
    inner: *mut Fl_Menu_Item,
}

/// Defines the menu flag for any added menu items using the add() method
#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq)]
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
    /// Initializes a new window, useful for popup menus
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
            MenuItem { inner: item_ptr }
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
            let ptr = Fl_Menu_Item_label(self.inner) as *mut raw::c_char;
            if !ptr.is_null() {
                let _ = CString::from_raw(ptr);
            }
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

    /// Clears the menu item
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
    pub fn active(&mut self) -> bool {
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

    /// Get the next menu item
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
            Some(MenuItem { inner: ptr })
        }
    }

    /// Get children of `MenuItem`
    pub fn children(&self) -> i32 {
        let mut i = 0;
        while let Some(_item) = self.next(i) {
            i += 1;
        }
        i
    }

    /// Get the menu item at `idx`
    pub fn at(&self, idx: i32) -> Option<MenuItem> {
        self.next(idx)
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
        let h: &mut i32 = &mut 0;
        let ret = unsafe { Fl_Menu_Item_measure(self.inner, h as _, std::ptr::null()) };
        (ret, *h)
    }

    /// Set the image of the menu item
    /// # Safety
    /// Trying to add a label after adding an image might lead to undefined behavior
    pub unsafe fn set_image<I: ImageExt>(&mut self, image: I) {
        assert!(!self.was_deleted());
        Fl_Menu_Item_image(self.inner, image.as_image_ptr() as _)
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

impl IntoIterator for MenuItem {
    type Item = MenuItem;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        let mut v: Vec<MenuItem> = vec![];
        let mut i = 0;
        while let Some(item) = self.next(i) {
            v.push(item);
            i += 1;
        }
        v.into_iter()
    }
}
