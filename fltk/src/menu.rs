use crate::enums::{Color, Font, LabelType, Shortcut};
use crate::prelude::*;
use crate::utils::FlString;
use fltk_sys::menu::*;
use std::{
    ffi::{CStr, CString},
    mem,
    os::raw,
};

bitflags::bitflags! {
    /// Defines the menu flag for any added menu items using the add() method
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct MenuFlag: i32 {
        /// Normal item
        const Normal = 0;
        /// Inactive item
        const Inactive = 1;
        /// Item is a checkbox toggle (shows checkbox for on/off state)
        const Toggle = 2;
        /// The on/off state for checkbox/radio buttons (if set, state is 'on')
        const Value = 4;
        /// Item is a radio button
        const Radio = 8;
        /// Invisible item
        const Invisible = 0x10;
        /// Indicates user_data() is a pointer to another menu array (unused with Rust)
        const SubmenuPointer = 0x20;
        /// Menu item is a submenu
        const Submenu = 0x40;
        /// Menu divider
        const MenuDivider = 0x80;
        /// Horizontal menu (actually reserved for future use)
        const MenuHorizontal = 0x100;
    }
}

/// Creates a menu bar
#[derive(Debug)]
pub struct MenuBar {
    inner: crate::widget::WidgetTracker,
    is_derived: bool,
}

crate::macros::widget::impl_widget_ext!(MenuBar, Fl_Menu_Bar);
crate::macros::widget::impl_widget_base!(MenuBar, Fl_Menu_Bar);
crate::macros::widget::impl_widget_default!(MenuBar);
crate::macros::menu::impl_menu_ext!(MenuBar, Fl_Menu_Bar);

/// Creates a menu button
#[derive(Debug)]
pub struct MenuButton {
    inner: crate::widget::WidgetTracker,
    is_derived: bool,
}

crate::macros::widget::impl_widget_ext!(MenuButton, Fl_Menu_Button);
crate::macros::widget::impl_widget_base!(MenuButton, Fl_Menu_Button);
crate::macros::widget::impl_widget_default!(MenuButton);
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
        if self.size() == 0 {
            return None;
        }
        unsafe {
            let ptr = Fl_Menu_Button_popup(self.inner.widget() as _);
            if ptr.is_null() {
                None
            } else {
                let item = MenuItem::from_ptr(ptr as *mut Fl_Menu_Item);
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
    inner: crate::widget::WidgetTracker,
    is_derived: bool,
}

crate::macros::widget::impl_widget_ext!(Choice, Fl_Choice);
crate::macros::widget::impl_widget_base!(Choice, Fl_Choice);
crate::macros::widget::impl_widget_default!(Choice);
crate::macros::menu::impl_menu_ext!(Choice, Fl_Choice);

/// Defines the window menu style for SysMenuBar
#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum WindowMenuStyle {
    /// No Window menu in the system menu bar
    NoWindowMenu = 0,
    /// No tabbed windows, but the system menu bar contains a Window menu
    TabbingModeNone,
    /// Windows are created by themselves but can be tabbed later
    TabbingModeAutomatic,
    /// Windows are tabbed when created
    TabbingModePreferred,
}

/// Creates a macOS system menu bar on macOS and a normal menu bar on other systems
#[derive(Debug)]
pub struct SysMenuBar {
    inner: crate::widget::WidgetTracker,
    is_derived: bool,
}

crate::macros::widget::impl_widget_ext!(SysMenuBar, Fl_Sys_Menu_Bar);
crate::macros::widget::impl_widget_base!(SysMenuBar, Fl_Sys_Menu_Bar);
crate::macros::widget::impl_widget_default!(SysMenuBar);
crate::macros::menu::impl_menu_ext!(SysMenuBar, Fl_Sys_Menu_Bar);

impl SysMenuBar {
    /// Sets the macos window menu style
    pub fn set_window_menu_style(style: WindowMenuStyle) {
        unsafe {
            Fl_Sys_Menu_Bar_set_window_menu_style(style as i32);
        }
    }

    /// Sets the about menu item callback
    pub fn set_about_callback<F: FnMut(&mut Self) + 'static>(&mut self, cb: F) {
        unsafe {
            unsafe extern "C" fn shim(wid: *mut Fl_Widget, data: *mut std::os::raw::c_void) {
                let mut wid = SysMenuBar::from_widget_ptr(wid as *mut _);
                let a = data as *mut Box<dyn FnMut(&mut SysMenuBar)>;
                let f: &mut (dyn FnMut(&mut SysMenuBar)) = &mut **a;
                let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| f(&mut wid)));
            }
            let a: *mut Box<dyn FnMut(&mut Self)> = Box::into_raw(Box::new(Box::new(cb)));
            let data: *mut std::os::raw::c_void = a as *mut std::os::raw::c_void;
            let callback: Fl_Callback = Some(shim);
            Fl_Sys_Menu_Bar_about(self.inner.widget() as _, callback, data);
        }
    }
}

#[cfg(feature = "single-threaded")]
type MenuItemWrapper = std::rc::Rc<*mut Fl_Menu_Item>;

#[cfg(not(feature = "single-threaded"))]
type MenuItemWrapper = std::sync::Arc<*mut Fl_Menu_Item>;

/// Creates a menu item
#[derive(Debug, Clone)]
pub struct MenuItem {
    inner: MenuItemWrapper,
}

impl Drop for MenuItem {
    fn drop(&mut self) {
        assert!(!self.inner.is_null());
        if MenuItemWrapper::strong_count(&self.inner) == 1 {
            unsafe {
                Fl_Menu_Item_delete(*self.inner);
            }
        }
    }
}

#[cfg(not(feature = "single-threaded"))]
unsafe impl Send for MenuItem {}

#[cfg(not(feature = "single-threaded"))]
unsafe impl Sync for MenuItem {}

impl PartialEq for MenuItem {
    fn eq(&self, other: &Self) -> bool {
        *self.inner == *other.inner
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

#[doc(hidden)]
#[derive(Clone, Debug)]
pub struct CMenuItem {
    /// menu item text, returned by label()
    pub text: Option<&'static str>,
    /// menu item shortcut
    pub shortcut: Shortcut,
    /// menu item callback          
    pub cb: Option<fn(*mut Choice)>,
    /// menu item flags like FL_MENU_TOGGLE, FL_MENU_RADIO
    pub flags: MenuFlag,
    /// how the menu item text looks like  
    pub labeltype: LabelType,
    /// which font for this menu item text       
    pub labelfont: Font,
    /// size of menu item text     
    pub labelsize: i32,
    /// menu item text color
    pub labelcolor: Color,
}

impl Default for CMenuItem {
    fn default() -> Self {
        Self {
            text: Some(""),
            shortcut: Shortcut::None,
            cb: None,
            flags: MenuFlag::Normal,
            labeltype: LabelType::Normal,
            labelfont: Font::Helvetica,
            labelsize: crate::app::font_size(),
            labelcolor: Color::Foreground,
        }
    }
}

impl CMenuItem {
    pub fn empty() -> Self {
        Self {
            text: None,
            shortcut: Shortcut::None,
            cb: None,
            flags: MenuFlag::Normal,
            labeltype: LabelType::Normal,
            labelfont: Font::Helvetica,
            labelsize: crate::app::font_size(),
            labelcolor: Color::Foreground,
        }
    }
}

impl MenuItem {
    /// Initializes a MenuItem from a pointer
    /// # Safety
    /// The pointer must be valid
    pub unsafe fn from_ptr(ptr: *mut Fl_Menu_Item) -> MenuItem {
        assert!(!ptr.is_null());
        let inner = MenuItemWrapper::from(ptr);
        let ptr = MenuItemWrapper::into_raw(inner);
        MenuItemWrapper::increment_strong_count(ptr);
        let inner = MenuItemWrapper::from_raw(ptr);
        MenuItem { inner }
    }

    /// Returns the inner pointer from a MenuItem
    /// # Safety
    /// Can return multiple mutable pointers to the same item
    pub unsafe fn as_ptr(&self) -> *mut Fl_Menu_Item {
        let ptr = MenuItemWrapper::into_raw(MenuItemWrapper::clone(&self.inner));
        MenuItemWrapper::increment_strong_count(ptr);
        let inner = MenuItemWrapper::from_raw(ptr);
        *inner
    }

    /// Initializes a new menu item.
    /// This will allocate a static MenuItem, that is expected to live for the entirety of the program.
    pub fn new(choices: &[&'static str]) -> MenuItem {
        unsafe {
            let sz = choices.len();
            let mut temp: Vec<*mut raw::c_char> = vec![];
            for &choice in choices {
                let c = CString::safe_new(choice);
                temp.push(c.into_raw() as _);
            }
            let item_ptr = Fl_Menu_Item_new(temp.as_ptr() as *mut *mut raw::c_char, sz as i32);
            assert!(!item_ptr.is_null());
            MenuItem {
                inner: MenuItemWrapper::new(item_ptr),
            }
        }
    }

    /// Initializes a MenuItem from a slice of CMenuItem.
    /// This will allocate a static MenuItem, that is expected to live for the entirety of the program
    #[doc(hidden)]
    pub fn new_from_cmenu(choices: &[CMenuItem]) -> MenuItem {
        unsafe {
            let sz = choices.len();
            let mut texts: Vec<*mut raw::c_char> = vec![];
            let mut shortcuts = vec![];
            let mut cbs = vec![];
            let mut flags = vec![];
            let mut lts = vec![];
            let mut fs = vec![];
            let mut sizes = vec![];
            let mut colors = vec![];
            for choice in choices {
                let c = if let Some(text) = choice.text {
                    CString::safe_new(text).into_raw() as _
                } else {
                    std::ptr::null_mut()
                };
                texts.push(c);
                shortcuts.push(choice.shortcut.bits());
                cbs.push(std::mem::transmute(choice.cb));
                flags.push(choice.flags.bits());
                lts.push(choice.labeltype as i32);
                fs.push(choice.labelfont.bits());
                sizes.push(choice.labelsize);
                colors.push(choice.labelcolor.bits());
            }
            let item_ptr = Fl_Menu_Item_new2(
                texts.as_mut_ptr() as *mut *mut raw::c_char,
                shortcuts.as_mut_ptr(),
                cbs.as_mut_ptr(),
                flags.as_mut_ptr(),
                lts.as_mut_ptr(),
                fs.as_mut_ptr(),
                sizes.as_mut_ptr(),
                colors.as_mut_ptr(),
                sz as i32,
            );
            assert!(!item_ptr.is_null());
            MenuItem {
                inner: MenuItemWrapper::new(item_ptr),
            }
        }
    }

    /// Creates a popup menu at the specified coordinates and returns its choice
    pub fn popup(&self, x: i32, y: i32) -> Option<MenuItem> {
        if self.size() == 0 {
            return None;
        }
        unsafe {
            let item = Fl_Menu_Item_popup(*self.inner, x, y);
            if item.is_null() {
                None
            } else {
                let item = MenuItem::from_ptr(item as *mut Fl_Menu_Item);
                Some(item)
            }
        }
    }

    /// Returns the label of the menu item
    pub fn label(&self) -> Option<String> {
        unsafe {
            let label_ptr = Fl_Menu_Item_label(*self.inner);
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
        unsafe {
            let txt = CString::safe_new(txt);
            Fl_Menu_Item_set_label(*self.inner, txt.into_raw() as _);
        }
    }

    /// Returns the label type of the menu item
    pub fn label_type(&self) -> LabelType {
        unsafe { mem::transmute(Fl_Menu_Item_label_type(*self.inner)) }
    }

    /// Sets the label type of the menu item
    pub fn set_label_type(&mut self, typ: LabelType) {
        unsafe {
            Fl_Menu_Item_set_label_type(*self.inner, typ as i32);
        }
    }

    /// Returns the label color of the menu item
    pub fn label_color(&self) -> Color {
        unsafe { mem::transmute(Fl_Menu_Item_label_color(*self.inner)) }
    }

    /// Sets the label color of the menu item
    pub fn set_label_color(&mut self, color: Color) {
        unsafe { Fl_Menu_Item_set_label_color(*self.inner, color.bits()) }
    }

    /// Returns the label font of the menu item
    pub fn label_font(&self) -> Font {
        unsafe { mem::transmute(Fl_Menu_Item_label_font(*self.inner)) }
    }

    /// Sets the label font of the menu item
    pub fn set_label_font(&mut self, font: Font) {
        unsafe { Fl_Menu_Item_set_label_font(*self.inner, font.bits()) }
    }

    /// Returns the label size of the menu item
    pub fn label_size(&self) -> i32 {
        unsafe { Fl_Menu_Item_label_size(*self.inner) }
    }

    /// Sets the label size of the menu item
    pub fn set_label_size(&mut self, sz: i32) {
        let sz = if sz < 1 { 1 } else { sz };
        unsafe { Fl_Menu_Item_set_label_size(*self.inner, sz) }
    }

    /// Returns the value of the menu item
    pub fn value(&self) -> bool {
        unsafe { Fl_Menu_Item_value(*self.inner) != 0 }
    }

    /// Sets the menu item
    pub fn set(&mut self) {
        unsafe { Fl_Menu_Item_set(*self.inner) }
    }

    /// Turns the check or radio item "off" for the menu item
    pub fn clear(&mut self) {
        unsafe { Fl_Menu_Item_clear(*self.inner) }
    }

    /// Returns whether the menu item is visible or not
    pub fn visible(&self) -> bool {
        unsafe { Fl_Menu_Item_visible(*self.inner) != 0 }
    }

    /// Returns whether the menu item is active
    pub fn active(&self) -> bool {
        unsafe { Fl_Menu_Item_active(*self.inner) != 0 }
    }

    /// Activates the menu item
    pub fn activate(&mut self) {
        unsafe { Fl_Menu_Item_activate(*self.inner) }
    }

    /// Deactivates the menu item
    pub fn deactivate(&mut self) {
        unsafe { Fl_Menu_Item_deactivate(*self.inner) }
    }

    /// Returns whether a menu item is a submenu
    pub fn is_submenu(&self) -> bool {
        unsafe { Fl_Menu_Item_submenu(*self.inner) != 0 }
    }

    /// Returns whether a menu item is a checkbox
    pub fn is_checkbox(&self) -> bool {
        unsafe { Fl_Menu_Item_checkbox(*self.inner) != 0 }
    }

    /// Returns whether a menu item is a radio item
    pub fn is_radio(&self) -> bool {
        unsafe { Fl_Menu_Item_radio(*self.inner) != 0 }
    }

    /// Shows the menu item
    pub fn show(&mut self) {
        unsafe { Fl_Menu_Item_show(*self.inner) }
    }

    /// Hides the menu item
    pub fn hide(&mut self) {
        unsafe { Fl_Menu_Item_hide(*self.inner) }
    }

    /// Get the next menu item skipping submenus
    pub fn next(&self, idx: i32) -> Option<MenuItem> {
        unsafe {
            let ptr = Fl_Menu_Item_next(*self.inner, idx);
            if ptr.is_null() {
                return None;
            }
            let label_ptr = Fl_Menu_Item_label(ptr);
            if label_ptr.is_null() {
                return None;
            }
            Some(MenuItem::from_ptr(ptr))
        }
    }

    /// Get children of `MenuItem`
    pub fn children(&self) -> i32 {
        unsafe { Fl_Menu_Item_children(*self.inner) }
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
        unsafe { Fl_Menu_Item_children(*self.inner) }
    }

    /// Get the menu item at `idx`
    pub fn at(&self, idx: i32) -> Option<MenuItem> {
        assert!(idx < self.size());
        unsafe {
            let ptr = Fl_Menu_Item_at(*self.inner, idx);
            if ptr.is_null() {
                None
            } else {
                Some(MenuItem::from_ptr(ptr as _))
            }
        }
    }

    /// Get the user data
    /// # Safety
    /// Can return multiple mutable instances of the user data, which has a different lifetime than the object
    pub unsafe fn user_data(&self) -> Option<Box<dyn FnMut()>> {
        let ptr = Fl_Menu_Item_user_data(*self.inner);
        if ptr.is_null() {
            None
        } else {
            let x = ptr as *mut Box<dyn FnMut()>;
            let x = Box::from_raw(x);
            Fl_Menu_Item_set_callback(*self.inner, None, std::ptr::null_mut());
            Some(*x)
        }
    }

    /// Set a callback for the menu item
    pub fn set_callback<F: FnMut(&mut Choice) + 'static>(&mut self, cb: F) {
        unsafe {
            unsafe extern "C" fn shim(wid: *mut fltk_sys::menu::Fl_Widget, data: *mut raw::c_void) {
                let mut wid = crate::widget::Widget::from_widget_ptr(wid as *mut _);
                let a: *mut Box<dyn FnMut(&mut crate::widget::Widget)> =
                    data as *mut Box<dyn FnMut(&mut crate::widget::Widget)>;
                let f: &mut (dyn FnMut(&mut crate::widget::Widget)) = &mut **a;
                let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| f(&mut wid)));
            }
            let _old_data = self.user_data();
            let a: *mut Box<dyn FnMut(&mut Choice)> = Box::into_raw(Box::new(Box::new(cb)));
            let data: *mut raw::c_void = a as *mut std::ffi::c_void;
            let callback: fltk_sys::menu::Fl_Callback = Some(shim);
            Fl_Menu_Item_set_callback(*self.inner, callback, data);
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
        unsafe {
            Fl_Menu_Item_draw(
                *self.inner,
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
        let mut h = 0;
        let ret = unsafe { Fl_Menu_Item_measure(*self.inner, &mut h as _, std::ptr::null()) };
        (ret, h)
    }

    /// Set the image of the menu item
    /// # Safety
    /// Trying to add a label after adding an image might lead to undefined behavior
    #[doc(hidden)]
    pub unsafe fn set_image<I: ImageExt>(&mut self, image: I) {
        assert!(!image.was_deleted());
        Fl_Menu_Item_image(*self.inner, image.as_image_ptr() as _)
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
        unsafe {
            if let Some(image) = image {
                assert!(!image.was_deleted());
                Fl_Menu_Item_add_image(*self.inner, image.as_image_ptr() as _, on_left as i32)
            } else {
                Fl_Menu_Item_add_image(*self.inner, std::ptr::null_mut(), on_left as i32)
            }
        }
    }

    /// Add a menu item
    pub fn add<F: FnMut(&mut Choice) + 'static>(
        &mut self,
        name: &str,
        shortcut: Shortcut,
        flag: MenuFlag,
        cb: F,
    ) -> i32 {
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
                *self.inner,
                temp.as_ptr(),
                shortcut.bits(),
                callback,
                data,
                flag.bits(),
            )
        }
    }

    /// Insert a menu item
    pub fn insert<F: FnMut(&mut Choice) + 'static>(
        &mut self,
        idx: i32,
        name: &str,
        shortcut: Shortcut,
        flag: MenuFlag,
        cb: F,
    ) -> i32 {
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
                *self.inner,
                idx,
                temp.as_ptr(),
                shortcut.bits(),
                callback,
                data,
                flag.bits(),
            )
        }
    }

    /// Add a menu item along with an emit (sender and message).
    pub fn add_emit<T: 'static + Clone + Send + Sync>(
        &mut self,
        label: &str,
        shortcut: Shortcut,
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
        shortcut: Shortcut,
        flag: MenuFlag,
        sender: crate::app::Sender<T>,
        msg: T,
    ) -> i32 {
        self.insert(idx, label, shortcut, flag, move |_| {
            sender.send(msg.clone())
        })
    }

    /// Set the menu item's shortcut
    pub fn set_shortcut(&mut self, shortcut: Shortcut) {
        unsafe {
            Fl_Menu_Item_set_shortcut(*self.inner, shortcut.bits());
        }
    }

    /// Set the menu item's shortcut
    pub fn set_flag(&mut self, flag: MenuFlag) {
        unsafe {
            Fl_Menu_Item_set_flag(*self.inner, flag.bits());
        }
    }
}

/// Delete a menu item
/// # Safety
/// The wrapper can't assure use after free when manually deleting a menu item
pub unsafe fn delete_menu_item(item: MenuItem) {
    Fl_Menu_Item_delete(*item.inner)
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

/// Wrapper around Fl_Mac_App_Menu which exposes several static methods
/// allowing the customization of the default system menu bar for the fltk application
#[derive(Debug, Clone, Copy)]
pub struct MacAppMenu;

impl MacAppMenu {
    /// Sets the about text
    pub fn set_about(about: &'static str) {
        unsafe {
            let about = CString::safe_new(about).as_ptr();
            Fl_Mac_App_Menu_set_about(about);
        }
    }

    /// Sets the print text
    pub fn set_print(print: &'static str) {
        unsafe {
            let print = CString::safe_new(print).as_ptr();
            Fl_Mac_App_Menu_set_print(print);
        }
    }

    /// Sets the print no titlebar text
    pub fn set_print_no_titlebar(print_no_titlebar: &'static str) {
        unsafe {
            let print_no_titlebar = CString::safe_new(print_no_titlebar).as_ptr();
            Fl_Mac_App_Menu_set_print_no_titlebar(print_no_titlebar);
        }
    }

    /// Sets the toggle print titlebar text
    pub fn set_toggle_print_titlebar(toggle_print_titlebar: &'static str) {
        unsafe {
            let toggle_print_titlebar = CString::safe_new(toggle_print_titlebar).as_ptr();
            Fl_Mac_App_Menu_set_toggle_print_titlebar(toggle_print_titlebar);
        }
    }

    /// Sets the services text
    pub fn set_services(services: &'static str) {
        unsafe {
            let services = CString::safe_new(services).as_ptr();
            Fl_Mac_App_Menu_set_services(services);
        }
    }

    /// Sets the hide text
    pub fn set_hide(hide: &'static str) {
        unsafe {
            let hide = CString::safe_new(hide).as_ptr();
            Fl_Mac_App_Menu_set_hide(hide);
        }
    }

    /// Sets the hide others text
    pub fn set_hide_others(hide_others: &'static str) {
        unsafe {
            let hide_others = CString::safe_new(hide_others).as_ptr();
            Fl_Mac_App_Menu_set_hide_others(hide_others);
        }
    }

    /// Sets the show text
    pub fn set_show(show: &'static str) {
        unsafe {
            let show = CString::safe_new(show).as_ptr();
            Fl_Mac_App_Menu_set_show(show);
        }
    }

    /// Sets the quit text
    pub fn set_quit(quit: &'static str) {
        unsafe {
            let quit = CString::safe_new(quit).as_ptr();
            Fl_Mac_App_Menu_set_quit(quit);
        }
    }

    /// Adds custom menu items to the application menu of the system menu bar.
    /// They are positioned after the "Print Front Window / Toggle printing of titlebar" items,
    /// or at their place if an item is removed by providing empty text
    pub fn custom_application_menu_items(m: MenuItem) {
        unsafe {
            Fl_Mac_App_Menu_custom_application_menu_items(m.as_ptr());
        }
    }
}
