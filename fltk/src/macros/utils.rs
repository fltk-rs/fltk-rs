#[doc(hidden)]
#[macro_export]
macro_rules! widget_props {
    ($name: ty) => {
        impl WidgetProps for $name {
            /// Initialize to position x, y
            fn with_pos(mut self, x: i32, y: i32) -> Self {
                let w = self.w();
                let h = self.h();
                self.resize(x, y, w, h);
                self
            }

            /// Initialize to size width, height
            fn with_size(mut self, width: i32, height: i32) -> Self {
                let x = self.x();
                let y = self.y();
                let w = self.w();
                let h = self.h();
                if w == 0 || h == 0 {
                    self.widget_resize(x, y, width, height);
                } else {
                    self.resize(x, y, width, height);
                }
                self
            }

            /// Initialize with a label
            fn with_label(mut self, title: &str) -> Self {
                self.set_label(title);
                self
            }

            /// Initialize with alignment
            fn with_align(mut self, align: $crate::enums::Align) -> Self {
                self.set_align(align);
                self
            }

            /// Initialize with type
            fn with_type<T: $crate::prelude::WidgetType>(mut self, typ: T) -> Self {
                self.set_type(typ);
                self
            }

            /// Initialize at bottom of another widget
            fn below_of<W: $crate::prelude::WidgetExt>(mut self, wid: &W, padding: i32) -> Self {
                let w = self.w();
                let h = self.h();
                debug_assert!(
                    w != 0 && h != 0,
                    "below_of requires the size of the widget to be known!"
                );
                self.resize(wid.x(), wid.y() + wid.h() + padding, w, h);
                self
            }

            /// Initialize above of another widget
            fn above_of<W: $crate::prelude::WidgetExt>(mut self, wid: &W, padding: i32) -> Self {
                let w = self.w();
                let h = self.h();
                debug_assert!(
                    w != 0 && h != 0,
                    "above_of requires the size of the widget to be known!"
                );
                self.resize(wid.x(), wid.y() - padding - h, w, h);
                self
            }

            /// Initialize right of another widget
            fn right_of<W: $crate::prelude::WidgetExt>(mut self, wid: &W, padding: i32) -> Self {
                let w = self.w();
                let h = self.h();
                debug_assert!(
                    w != 0 && h != 0,
                    "right_of requires the size of the widget to be known!"
                );
                self.resize(wid.x() + wid.w() + padding, wid.y(), w, h);
                self
            }

            /// Initialize left of another widget
            fn left_of<W: $crate::prelude::WidgetExt>(mut self, wid: &W, padding: i32) -> Self {
                let w = self.w();
                let h = self.h();
                debug_assert!(
                    w != 0 && h != 0,
                    "left_of requires the size of the widget to be known!"
                );
                self.resize(wid.x() - w - padding, wid.y(), w, h);
                self
            }

            /// Initialize center of another widget
            fn center_of<W: $crate::prelude::WidgetExt>(mut self, w: &W) -> Self {
                debug_assert!(
                    w.w() != 0 && w.h() != 0,
                    "center_of requires the size of the widget to be known!"
                );
                let sw = self.w() as f64;
                let sh = self.h() as f64;
                let ww = w.w() as f64;
                let wh = w.h() as f64;
                let sx = (ww - sw) / 2.0;
                let sy = (wh - sh) / 2.0;
                let wx = if w.as_window().is_some() { 0 } else { w.x() };
                let wy = if w.as_window().is_some() { 0 } else { w.y() };
                self.resize(sx as i32 + wx, sy as i32 + wy, sw as i32, sh as i32);
                self.redraw();
                self
            }

            /// Initialize center of another widget on the x axis
            fn center_x<W: $crate::prelude::WidgetExt>(mut self, w: &W) -> Self {
                debug_assert!(
                    w.w() != 0 && w.h() != 0,
                    "center_of requires the size of the widget to be known!"
                );
                let sw = self.w() as f64;
                let sh = self.h() as f64;
                let ww = w.w() as f64;
                let sx = (ww - sw) / 2.0;
                let sy = self.y();
                let wx = if w.as_window().is_some() { 0 } else { w.x() };
                self.resize(sx as i32 + wx, sy, sw as i32, sh as i32);
                self.redraw();
                self
            }

            /// Initialize center of another widget on the y axis
            fn center_y<W: $crate::prelude::WidgetExt>(mut self, w: &W) -> Self {
                debug_assert!(
                    w.w() != 0 && w.h() != 0,
                    "center_of requires the size of the widget to be known!"
                );
                let sw = self.w() as f64;
                let sh = self.h() as f64;
                let wh = w.h() as f64;
                let sx = self.x();
                let sy = (wh - sh) / 2.0;
                let wy = if w.as_window().is_some() { 0 } else { w.y() };
                self.resize(sx, sy as i32 + wy, sw as i32, sh as i32);
                self.redraw();
                self
            }

            /// Initialize center of parent
            fn center_of_parent(mut self) -> Self {
                if let Some(w) = self.parent() {
                    debug_assert!(
                        w.w() != 0 && w.h() != 0,
                        "center_of requires the size of the widget to be known!"
                    );
                    let sw = self.w() as f64;
                    let sh = self.h() as f64;
                    let ww = w.w() as f64;
                    let wh = w.h() as f64;
                    let sx = (ww - sw) / 2.0;
                    let sy = (wh - sh) / 2.0;
                    let wx = if w.as_window().is_some() { 0 } else { w.x() };
                    let wy = if w.as_window().is_some() { 0 } else { w.y() };
                    self.resize(sx as i32 + wx, sy as i32 + wy, sw as i32, sh as i32);
                    self.redraw();
                }
                self
            }

            /// Initialize to the size of another widget
            fn size_of<W: $crate::prelude::WidgetExt>(mut self, w: &W) -> Self {
                debug_assert!(
                    w.w() != 0 && w.h() != 0,
                    "size_of requires the size of the widget to be known!"
                );
                let x = self.x();
                let y = self.y();
                self.resize(x, y, w.w(), w.h());
                self
            }

            /// Initialize to the size of the parent
            fn size_of_parent(mut self) -> Self {
                if let Some(parent) = self.parent() {
                    let w = parent.w();
                    let h = parent.h();
                    let x = self.x();
                    let y = self.y();
                    self.resize(x, y, w, h);
                }
                self
            }
        }
    };
}

pub use widget_props;
