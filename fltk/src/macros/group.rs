#[doc(hidden)]
#[macro_export]
/// Implements GroupExt
macro_rules! impl_group_ext {
    ($name: ident, $flname: ident) => {
        impl IntoIterator for $name {
            type Item = $crate::widget::Widget;
            type IntoIter = std::vec::IntoIter<Self::Item>;

            fn into_iter(self) -> Self::IntoIter {
                let mut v: Vec<$crate::widget::Widget> = vec![];
                for i in 0..self.children() {
                    v.push(self.child(i).unwrap());
                }
                v.into_iter()
            }
        }

        paste::paste! {
            unsafe impl GroupExt for $name {
                fn begin(&self) {

                    unsafe { [<$flname _begin>](self.inner.widget() as _) }
                }

                fn end(&self) {

                    unsafe { [<$flname _end>](self.inner.widget() as _) }
                }

                fn clear(&mut self) {

                    unsafe {
                        [<$flname _clear>](self.inner.widget() as _);
                    }
                }

                unsafe fn unsafe_clear(&mut self) {

                    [<$flname _clear>](self.inner.widget() as _);
                }

                fn children(&self) -> i32 {
                    unsafe {
                        [<$flname _children>](self.inner.widget() as _) as i32
                    }
                }

                fn child(&self, idx: i32) -> Option<$crate::widget::Widget> {
                    unsafe {
                        if idx >= self.children() || idx < 0 {
                            return None;
                        }
                        let child_widget = [<$flname _child>](self.inner.widget() as _, idx as i32);
                        if child_widget.is_null() {
                            None
                        } else {
                            Some($crate::widget::Widget::from_widget_ptr(
                                child_widget as *mut fltk_sys::widget::Fl_Widget,
                            ))
                        }
                    }
                }

                fn find<W: WidgetExt>(&self, widget: &W) -> i32 {
                    unsafe {
                        [<$flname _find>](self.inner.widget() as _, widget.as_widget_ptr() as *mut _)
                            as i32
                    }
                }

                fn add<W: WidgetExt>(&mut self, widget: &W) {
                    unsafe {
                        [<$flname _add>](self.inner.widget() as _, widget.as_widget_ptr() as *mut _)
                    }
                }

                fn insert<W: WidgetExt>(&mut self, widget: &W, index: i32) {
                    unsafe {
                        let index = if index < 0 {
                            0
                        } else {
                            index
                        };
                        [<$flname _insert>](
                            self.inner.widget() as _,
                            widget.as_widget_ptr() as *mut _,
                            index as i32,
                        )
                    }
                }

                fn remove<W: WidgetExt>(&mut self, widget: &W) {
                    unsafe {
                        [<$flname _remove>](self.inner.widget() as _, widget.as_widget_ptr() as *mut _)
                    }
                }

                fn remove_by_index(&mut self, idx: i32) {
                    unsafe {
                        assert!(idx < self.children());
                        [<$flname _remove_by_index>](self.inner.widget() as _, idx as i32);
                    }
                }

                fn resizable<W: WidgetExt>(&self, widget: &W) {
                    unsafe {
                        [<$flname _resizable>](self.inner.widget() as _, widget.as_widget_ptr() as *mut _)
                    }
                }

                fn make_resizable(&mut self, val: bool) {

                    let ptr = if val {
                        self.inner.widget() as _
                    } else {
                        std::ptr::null_mut()
                    };
                    unsafe { [<$flname _resizable>](self.inner.widget() as _, ptr as *mut _) }
                }

                fn add_resizable<W: WidgetExt>(&mut self, widget: &W) {
                    self.resizable(widget);
                    self.add(widget);
                }

                fn set_clip_children(&mut self, flag: bool) {

                    unsafe { [<$flname _set_clip_children>](self.inner.widget() as _, flag as i32) }
                }

                fn clip_children(&self) -> bool {

                    unsafe { [<$flname _clip_children>](self.inner.widget() as _) != 0 }
                }

                fn draw_child<W: WidgetExt>(&self, w: &mut W) {
                    unsafe {
                        $crate::app::open_display();
                        [<$flname _draw_child>](self.inner.widget() as _, w.as_widget_ptr() as _)
                    }
                }

                fn update_child<W: WidgetExt>(&self, w: &mut W) {
                    unsafe {
                        $crate::app::open_display();
                        [<$flname _update_child>](self.inner.widget() as _, w.as_widget_ptr() as _)
                    }
                }

                fn draw_outside_label<W: WidgetExt>(&self, w: &mut W) {
                    unsafe {
                        $crate::app::open_display();
                        [<$flname _draw_outside_label>](
                            self.inner.widget() as _,
                            w.as_widget_ptr() as _,
                        )
                    }
                }

                fn draw_children(&mut self) {

                    unsafe {
                        $crate::app::open_display();
                        [<$flname _draw_children>](self.inner.widget() as _)
                    }
                }

                fn init_sizes(&mut self) {
                    unsafe {
                        [<$flname _init_sizes>](self.inner.widget() as _)
                    }
                }

                fn bounds(&self) -> Vec<(i32, i32, i32, i32)> {
                    let children = self.children();
                    let mut vec = vec![];
                    for i in 0..children {
                        let child = self.child(i).unwrap();
                        let x = child.x();
                        let y = child.y();
                        let r = child.w() + x;
                        let b = child.h() + y;
                        vec.push((x, y, r, b));
                    }
                    vec
                }

                unsafe fn into_group(&self) -> $crate::group::Group {
                    $crate::group::Group::from_widget_ptr(self.inner.widget() as _)
                }
            }
        }
    };
}

pub use impl_group_ext;

#[doc(hidden)]
#[macro_export]
/// Implements GroupExt via a member
macro_rules! impl_group_ext_via {
    ($widget:ty, $member:tt) => {
        unsafe impl GroupExt for $widget {
            fn begin(&self) {
                self.$member.begin()
            }

            fn end(&self) {
                self.$member.end()
            }

            fn clear(&mut self) {
                self.$member.clear()
            }

            unsafe fn unsafe_clear(&mut self) {
                self.$member.unsafe_clear()
            }

            fn children(&self) -> i32 {
                self.$member.children()
            }

            fn child(&self, idx: i32) -> Option<$crate::widget::Widget> {
                self.$member.child(idx)
            }

            fn find<W: WidgetExt>(&self, widget: &W) -> i32 {
                self.$member.find(widget)
            }

            fn add<W: WidgetExt>(&mut self, widget: &W) {
                self.$member.add(widget)
            }

            fn insert<W: WidgetExt>(&mut self, widget: &W, index: i32) {
                self.$member.insert(widget, index)
            }

            fn remove<W: WidgetExt>(&mut self, widget: &W) {
                self.$member.remove(widget)
            }

            fn remove_by_index(&mut self, idx: i32) {
                self.$member.remove_by_index(idx)
            }

            fn resizable<W: WidgetExt>(&self, widget: &W) {
                self.$member.resizable(widget)
            }

            fn make_resizable(&mut self, val: bool) {
                self.$member.make_resizable(val)
            }

            fn add_resizable<W: WidgetExt>(&mut self, widget: &W) {
                self.$member.add_resizable(widget)
            }

            fn set_clip_children(&mut self, flag: bool) {
                self.$member.set_clip_children(flag)
            }

            fn clip_children(&self) -> bool {
                self.$member.clip_children()
            }

            fn draw_child<W: WidgetExt>(&self, w: &mut W) {
                self.$member.draw_child(w)
            }

            fn update_child<W: WidgetExt>(&self, w: &mut W) {
                self.$member.update_child(w)
            }

            fn draw_outside_label<W: WidgetExt>(&self, w: &mut W) {
                self.$member.draw_outside_label(w)
            }

            fn draw_children(&mut self) {
                self.$member.draw_children()
            }

            fn init_sizes(&mut self) {
                self.$member.init_sizes()
            }

            fn bounds(&self) -> Vec<(i32, i32, i32, i32)> {
                self.$member.bounds()
            }

            unsafe fn into_group(&self) -> $crate::group::Group {
                self.$member.into_group()
            }
        }
    };
}

pub use impl_group_ext_via;
