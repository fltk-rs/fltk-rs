/// Implements GroupExt
macro_rules! impl_group_ext {
    ($name: ident, $flname: ident) => {
        impl IntoIterator for $name {
            type Item = Widget;
            type IntoIter = std::vec::IntoIter<Self::Item>;

            fn into_iter(self) -> Self::IntoIter {
                let mut v: Vec<Widget> = vec![];
                for i in 0..self.children() {
                    v.push(self.child(i).unwrap());
                }
                v.into_iter()
            }
        }

        paste::paste! {
            unsafe impl GroupExt for $name {
                fn begin(&self) {
                    assert!(!self.was_deleted());
                    unsafe { [<$flname _begin>](self.inner) }
                }

                fn end(&self) {
                    assert!(!self.was_deleted());
                    unsafe { [<$flname _end>](self.inner) }
                }

                fn clear(&mut self) {
                    assert!(!self.was_deleted());
                    unsafe {
                        [<$flname _clear>](self.inner);
                    }
                }

                unsafe fn unsafe_clear(&mut self) {
                    assert!(!self.was_deleted());
                    [<$flname _clear>](self.inner);
                }

                fn children(&self) -> i32 {
                    unsafe {
                        assert!(!self.was_deleted());
                        [<$flname _children>](self.inner) as i32
                    }
                }

                fn child(&self, idx: i32) -> Option<Widget> {
                    unsafe {
                        assert!(!self.was_deleted());
                        if idx >= self.children() || idx < 0 {
                            return None;
                        }
                        let child_widget = [<$flname _child>](self.inner, idx as i32);
                        if child_widget.is_null() {
                            None
                        } else {
                            Some(Widget::from_widget_ptr(
                                child_widget as *mut fltk_sys::widget::Fl_Widget,
                            ))
                        }
                    }
                }

                fn find<W: WidgetExt>(&self, widget: &W) -> i32 {
                    unsafe {
                        assert!(!self.was_deleted());
                        assert!(!widget.was_deleted());
                        [<$flname _find>](self.inner, widget.as_widget_ptr() as *mut _)
                            as i32
                    }
                }

                fn add<W: WidgetExt>(&mut self, widget: &W) {
                    unsafe {
                        assert!(!self.was_deleted());
                        assert!(!widget.was_deleted());
                        [<$flname _add>](self.inner, widget.as_widget_ptr() as *mut _)
                    }
                }

                fn insert<W: WidgetExt>(&mut self, widget: &W, index: i32) {
                    unsafe {
                        assert!(!self.was_deleted());
                        assert!(!widget.was_deleted());
                        [<$flname _insert>](
                            self.inner,
                            widget.as_widget_ptr() as *mut _,
                            index as i32,
                        )
                    }
                }

                fn remove<W: WidgetExt>(&mut self, widget: &W) {
                    unsafe {
                        assert!(!self.was_deleted());
                        assert!(!widget.was_deleted());
                        [<$flname _remove>](self.inner, widget.as_widget_ptr() as *mut _)
                    }
                }

                fn remove_by_index(&mut self, idx: i32) {
                    unsafe {
                        assert!(!self.was_deleted());
                        assert!(idx < self.children());
                        [<$flname _remove_by_index>](self.inner, idx as i32);
                    }
                }

                fn resizable<W: WidgetExt>(&self, widget: &W) {
                    unsafe {
                        assert!(!self.was_deleted());
                        assert!(!widget.was_deleted());
                        [<$flname _resizable>](self.inner, widget.as_widget_ptr() as *mut _)
                    }
                }

                fn make_resizable(&mut self, val: bool) {
                    assert!(!self.was_deleted());
                    let ptr = if val {
                        self.inner
                    } else {
                        std::ptr::null_mut()
                    };
                    unsafe { [<$flname _resizable>](self.inner, ptr as *mut _) }
                }

                fn add_resizable<W: WidgetExt>(&mut self, widget: &W) {
                    self.resizable(widget);
                    self.add(widget);
                }

                fn set_clip_children(&mut self, flag: bool) {
                    assert!(!self.was_deleted());
                    unsafe { [<$flname _set_clip_children>](self.inner, flag as i32) }
                }

                fn clip_children(&mut self) -> bool {
                    assert!(!self.was_deleted());
                    unsafe { [<$flname _clip_children>](self.inner) != 0 }
                }

                fn draw_child<W: WidgetExt>(&self, w: &mut W) {
                    assert!(!self.was_deleted());
                    assert!(!w.was_deleted());
                    unsafe {
                        crate::app::open_display();
                        [<$flname _draw_child>](self.inner as _, w.as_widget_ptr() as _)
                    }
                }

                fn update_child<W: WidgetExt>(&self, w: &mut W) {
                    assert!(!self.was_deleted());
                    assert!(!w.was_deleted());
                    unsafe {
                        crate::app::open_display();
                        [<$flname _update_child>](self.inner as _, w.as_widget_ptr() as _)
                    }
                }

                fn draw_outside_label<W: WidgetExt>(&self, w: &mut W) {
                    assert!(!self.was_deleted());
                    assert!(!w.was_deleted());
                    unsafe {
                        crate::app::open_display();
                        [<$flname _draw_outside_label>](
                            self.inner as _,
                            w.as_widget_ptr() as _,
                        )
                    }
                }

                fn draw_children(&mut self) {
                    assert!(!self.was_deleted());
                    unsafe {
                        crate::app::open_display();
                        [<$flname _draw_children>](self.inner as _)
                    }
                }

                fn init_sizes(&mut self) {
                    unsafe {
                        assert!(!self.was_deleted());
                        [<$flname _init_sizes>](self.inner)
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

                unsafe fn into_group(&self) -> crate::group::Group {
                    crate::group::Group::from_widget_ptr(self.inner as _)
                }
            }
        }
    };
}

pub(crate) use impl_group_ext;
