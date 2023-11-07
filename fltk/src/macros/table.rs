#[doc(hidden)]
#[macro_export]
/// Implements TableExt
macro_rules! impl_table_ext {
    ($name: ident, $flname: ident) => {
        paste::paste! {
            unsafe impl TableExt for $name {
                fn clear(&mut self) {
                    unsafe {
                        [<$flname _clear>](self.inner.widget() as _)
                    }
                }

                fn set_table_frame(&mut self, frame: $crate::enums::FrameType) {
                    unsafe {
                        [<$flname _set_table_box>](self.inner.widget() as _, frame.as_i32())
                    }
                }

                fn table_frame(&self) -> $crate::enums::FrameType {

                    unsafe { $crate::enums::FrameType::from_i32([<$flname _table_box>](self.inner.widget() as _)) }
                }

                fn set_rows(&mut self, val: i32) {
                    unsafe {
                        [<$flname _set_rows>](self.inner.widget() as _, val as i32)
                    }
                }

                fn rows(&self) -> i32 {
                    unsafe {
                        [<$flname _rows>](self.inner.widget() as _) as i32
                    }
                }

                fn set_cols(&mut self, val: i32) {
                    unsafe {
                        [<$flname _set_cols>](self.inner.widget() as _, val as i32)
                    }
                }

                fn cols(&self) -> i32 {
                    unsafe {
                        [<$flname _cols>](self.inner.widget() as _) as i32
                    }
                }

                fn visible_cells(&self) -> (i32, i32, i32, i32) {
                    unsafe {
                        let mut row_top = 0;
                        let mut col_left = 0;
                        let mut row_bot = 0;
                        let mut col_right = 0;
                        [<$flname _visible_cells>](
                            self.inner.widget() as _,
                            &mut row_top,
                            &mut col_left,
                            &mut row_bot,
                            &mut col_right,
                        );
                        (row_top, col_left, row_bot, col_right)
                    }
                }

                fn try_visible_cells(&self) -> Option<(i32, i32, i32, i32)> {
                    let (a, b, c, d) = self.visible_cells();
                    if a == -1 || b == -1 || c == -1 || d == -1 {
                        None
                    } else {
                        Some((a, b, c, d))
                    }
                }

                fn is_interactive_resize(&self) -> bool {
                    unsafe {
                        [<$flname _is_interactive_resize>](self.inner.widget() as _) != 0
                    }
                }

                fn row_resize(&self) -> bool {
                    unsafe {
                        [<$flname _row_resize>](self.inner.widget() as _) != 0
                    }
                }

                fn set_row_resize(&mut self, flag: bool) {
                    unsafe {
                        [<$flname _set_row_resize>](self.inner.widget() as _, flag as i32)
                    }
                }

                fn col_resize(&self) -> bool {
                    unsafe {
                        [<$flname _col_resize>](self.inner.widget() as _) != 0
                    }
                }

                fn set_col_resize(&mut self, flag: bool) {
                    unsafe {
                        [<$flname _set_col_resize>](self.inner.widget() as _, flag as i32)
                    }
                }

                fn col_resize_min(&self) -> i32 {
                    unsafe {
                        [<$flname _col_resize_min>](self.inner.widget() as _) as i32
                    }
                }

                fn set_col_resize_min(&mut self, val: i32) {
                    unsafe {
                        [<$flname _set_col_resize_min>](self.inner.widget() as _, val as i32)
                    }
                }

                fn row_resize_min(&self) -> i32 {
                    unsafe {
                        [<$flname _row_resize_min>](self.inner.widget() as _) as i32
                    }
                }

                fn set_row_resize_min(&mut self, val: i32) {
                    unsafe {
                        [<$flname _set_row_resize_min>](self.inner.widget() as _, val as i32)
                    }
                }

                fn row_header(&self) -> bool {
                    unsafe {
                        [<$flname _row_header>](self.inner.widget() as _) != 0
                    }
                }

                fn set_row_header(&mut self, flag: bool) {
                    unsafe {
                        [<$flname _set_row_header>](self.inner.widget() as _, flag as i32)
                    }
                }

                fn col_header(&self) -> bool {
                    unsafe {
                        [<$flname _col_header>](self.inner.widget() as _) != 0
                    }
                }

                fn set_col_header(&mut self, flag: bool) {
                    unsafe {
                        [<$flname _set_col_header>](self.inner.widget() as _, flag as i32)
                    }
                }

                fn set_col_header_height(&mut self, height: i32) {
                    unsafe {
                        [<$flname _set_col_header_height>](self.inner.widget() as _, height)
                    }
                }

                fn col_header_height(&self) -> i32 {
                    unsafe {
                        [<$flname _col_header_height>](self.inner.widget() as _)
                    }
                }

                fn set_row_header_width(&mut self, width: i32) {
                    unsafe {
                        [<$flname _set_row_header_width>](self.inner.widget() as _, width)
                    }
                }

                fn row_header_width(&self) -> i32 {
                    unsafe {
                        [<$flname _row_header_width>](self.inner.widget() as _)
                    }
                }

                fn set_row_header_color(&mut self, val: $crate::enums::Color) {
                    unsafe {
                        [<$flname _set_row_header_color>](self.inner.widget() as _, val.bits() as u32)
                    }
                }

                fn row_header_color(&self) -> $crate::enums::Color {
                    unsafe {
                        std::mem::transmute([<$flname _row_header_color>](self.inner.widget() as _))
                    }
                }

                fn set_col_header_color(&mut self, val: $crate::enums::Color) {
                    unsafe {
                        [<$flname _set_col_header_color>](self.inner.widget() as _, val.bits() as u32)
                    }
                }

                fn col_header_color(&self) -> $crate::enums::Color {
                    unsafe {
                        std::mem::transmute([<$flname _col_header_color>](self.inner.widget() as _))
                    }
                }

                fn set_row_height(&mut self, row: i32, height: i32) {
                    unsafe {
                        [<$flname _set_row_height>](self.inner.widget() as _, row, height)
                    }
                }

                fn row_height(&self, row: i32) -> i32 {
                    unsafe {
                        [<$flname _row_height>](self.inner.widget() as _, row)
                    }
                }

                fn set_col_width(&mut self, col: i32, width: i32) {
                    unsafe {
                        [<$flname _set_col_width>](self.inner.widget() as _, col, width)
                    }
                }

                fn col_width(&self, col: i32) -> i32 {
                    unsafe {
                        [<$flname _col_width>](self.inner.widget() as _, col)
                    }
                }

                fn set_row_height_all(&mut self, height: i32) {
                    unsafe {
                        [<$flname _set_row_height_all>](self.inner.widget() as _, height)
                    }
                }

                fn set_col_width_all(&mut self, width: i32) {
                    unsafe {
                        [<$flname _set_col_width_all>](self.inner.widget() as _, width)
                    }
                }

                fn set_row_position(&mut self, row: i32) {
                    unsafe {
                        [<$flname _set_row_position>](self.inner.widget() as _, row as i32)
                    }
                }

                fn set_col_position(&mut self, col: i32) {
                    unsafe {
                        [<$flname _set_col_position>](self.inner.widget() as _, col as i32)
                    }
                }

                fn row_position(&self) -> i32 {
                    unsafe {
                        [<$flname _row_position>](self.inner.widget() as _) as i32
                    }
                }

                fn col_position(&self) -> i32 {
                    unsafe {
                        [<$flname _col_position>](self.inner.widget() as _) as i32
                    }
                }

                fn set_top_row(&mut self, row: i32) {
                    unsafe {
                        [<$flname _set_top_row>](self.inner.widget() as _, row as i32)
                    }
                }

                fn top_row(&self) -> i32 {
                    unsafe {
                        [<$flname _top_row>](self.inner.widget() as _) as i32
                    }
                }

                fn is_selected(&self, r: i32, c: i32) -> bool {
                    unsafe {
                        [<$flname _is_selected>](self.inner.widget() as _, r, c) != 0
                    }
                }

                fn get_selection(&self) -> (i32, i32, i32, i32) {
                    unsafe {
                        let mut row_top = 0;
                        let mut col_left = 0;
                        let mut row_bot = 0;
                        let mut col_right = 0;
                        [<$flname _get_selection>](
                            self.inner.widget() as _,
                            &mut row_top,
                            &mut col_left,
                            &mut row_bot,
                            &mut col_right,
                        );
                        (row_top, col_left, row_bot, col_right)
                    }
                }

                fn try_get_selection(&self) -> Option<(i32, i32, i32, i32)> {
                    let (a, b, c, d) = self.get_selection();
                    if a < 0 && b < 0 && c >= 0 && d >= 0 {
                        Some((0, 0, c, d))
                    } else if a >= 0 && b >=0 && c >=0 && d >= 0 {
                        Some((a, b, c, d))
                    } else {
                        None
                    }
                }

                fn set_selection(&mut self, row_top: i32, col_left: i32, row_bot: i32, col_right: i32) {
                    unsafe {
                        [<$flname _set_selection>](
                            self.inner.widget() as _, row_top, col_left, row_bot, col_right,
                        )
                    }
                }

                fn unset_selection(&mut self) {
                    self.set_selection(-1, -1, -1, -1)
                }

                fn move_cursor_with_shift_select(
                    &mut self,
                    r: i32,
                    c: i32,
                    shiftselect: bool,
                ) -> Result<(), FltkError> {
                    unsafe {
                        let x = [<$flname _move_cursor_with_shiftselect>](
                            self.inner.widget() as _,
                            r,
                            c,
                            shiftselect as i32,
                        );
                        if x == 0 {
                            return Err(FltkError::Internal(FltkErrorKind::FailedOperation));
                        }
                        Ok(())
                    }
                }

                fn move_cursor(&mut self, r: i32, c: i32) -> Result<(), FltkError> {
                    unsafe {
                        let x = [<$flname _move_cursor>](self.inner.widget() as _, r, c);
                        if x == 0 {
                            return Err(FltkError::Internal(FltkErrorKind::FailedOperation));
                        }
                        Ok(())
                    }
                }

                fn scrollbar_size(&self) -> i32 {
                    unsafe {
                        [<$flname _scrollbar_size>](self.inner.widget() as _) as i32
                    }
                }

                fn set_scrollbar_size(&mut self, new_size: i32) {
                    unsafe {
                        [<$flname _set_scrollbar_size>](self.inner.widget() as _, new_size as i32)
                    }
                }

                fn set_tab_cell_nav(&mut self, val: bool) {
                    unsafe {
                        [<$flname _set_tab_cell_nav>](self.inner.widget() as _, val as i32)
                    }
                }

                fn tab_cell_nav(&self) -> bool {
                    unsafe {
                        [<$flname _tab_cell_nav>](self.inner.widget() as _) != 0
                    }
                }

                fn draw_cell<
                    F: FnMut(&mut Self, $crate::table::TableContext, i32, i32, i32, i32, i32, i32)
                        + 'static,
                >(
                    &mut self,
                    cb: F,
                ) {

                    assert!(self.is_derived);
                    pub type CustomDrawCellCallback = Option<
                        unsafe extern "C" fn(
                            wid: *mut Fl_Widget,
                            ctx: std::os::raw::c_int,
                            arg2: std::os::raw::c_int,
                            arg3: std::os::raw::c_int,
                            arg4: std::os::raw::c_int,
                            arg5: std::os::raw::c_int,
                            arg6: std::os::raw::c_int,
                            arg7: std::os::raw::c_int,
                            data: *mut std::os::raw::c_void,
                        ),
                    >;
                    unsafe {
                    unsafe extern "C" fn shim(
                            wid: *mut Fl_Widget,
                            ctx: std::os::raw::c_int,
                            arg2: std::os::raw::c_int,
                            arg3: std::os::raw::c_int,
                            arg4: std::os::raw::c_int,
                            arg5: std::os::raw::c_int,
                            arg6: std::os::raw::c_int,
                            arg7: std::os::raw::c_int,
                            data: *mut std::os::raw::c_void,
                        ) {
                            let mut wid = $name::from_widget_ptr(wid as *mut _);
                            let ctx: TableContext = std::mem::transmute(ctx);
                            let a: *mut Box<
                                dyn FnMut(
                                    &mut $name,
                                    $crate::table::TableContext,
                                    i32,
                                    i32,
                                    i32,
                                    i32,
                                    i32,
                                    i32,
                                ),
                            > = data as *mut Box<
                                dyn FnMut(
                                    &mut $name,
                                    $crate::table::TableContext,
                                    i32,
                                    i32,
                                    i32,
                                    i32,
                                    i32,
                                    i32,
                                ),
                            >;
                            let f: &mut (dyn FnMut(
                                &mut $name,
                                $crate::table::TableContext,
                                i32,
                                i32,
                                i32,
                                i32,
                                i32,
                                i32,
                            )) = &mut **a;
                            let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                                f(&mut wid, ctx, arg2, arg3, arg4, arg5, arg6, arg7)
                            }));
                        }
                        let mut _old_data = None;
                        if self.is_derived {
                            _old_data = self.draw_cell_data();
                        }
                        let a: *mut Box<
                            dyn FnMut(
                                &mut Self,
                                $crate::table::TableContext,
                                i32,
                                i32,
                                i32,
                                i32,
                                i32,
                                i32,
                            ),
                        > = Box::into_raw(Box::new(Box::new(cb)));
                        let data: *mut std::os::raw::c_void = a as *mut std::os::raw::c_void;
                        let callback: CustomDrawCellCallback = Some(shim);
                        [<$flname _draw_cell>](self.inner.widget() as _, callback, data);
                    }
                }

                unsafe fn draw_cell_data(&self) -> Option<Box<dyn FnMut()>> {
                    let ptr = [<$flname _draw_cell_data>](self.inner.widget() as _);
                    if ptr.is_null() {
                        None
                    } else {
                        let data = ptr as *mut Box<dyn FnMut()>;
                        let data = Box::from_raw(data);
                        Some(*data)
                    }
                }

                fn callback_col(&self) -> i32 {
                    unsafe { [<$flname _callback_col>](self.inner.widget() as _) }
                }

                fn callback_row(&self) -> i32 {
                    unsafe { [<$flname _callback_row>](self.inner.widget() as _) }
                }

                fn callback_context(&self) -> TableContext {
                    unsafe { std::mem::transmute([<$flname _callback_context>](self.inner.widget() as _)) }
                }

                fn scrollbar(&self) -> $crate::valuator::Scrollbar {

                    unsafe {
                        let ptr = [<$flname _scrollbar>](self.inner.widget() as _);
                        assert!(!ptr.is_null());
                        $crate::valuator::Scrollbar::from_widget_ptr(
                            ptr as *mut fltk_sys::widget::Fl_Widget,
                        )
                    }
                }

                fn hscrollbar(&self) -> $crate::valuator::Scrollbar {

                    unsafe {
                        let ptr = [<$flname _hscrollbar>](self.inner.widget() as _);
                        assert!(!ptr.is_null());
                        $crate::valuator::Scrollbar::from_widget_ptr(
                            ptr as *mut fltk_sys::widget::Fl_Widget,
                        )
                    }
                }

                fn find_cell(
                    &self,
                    ctx: $crate::table::TableContext,
                    row: i32,
                    col: i32,
                ) -> Option<(i32, i32, i32, i32)> {

                    let mut x = 0;
                    let mut y = 0;
                    let mut w = 0;
                    let mut h = 0;
                    unsafe {
                        let ret = [<$flname _find_cell>](self.inner.widget() as _, ctx as i32, row, col, &mut x, &mut y, &mut w, &mut h);
                        if ret == 0 {
                            Some((x, y, w, h))
                        } else {
                            None
                        }
                    }
                }

                fn cursor2rowcol(
                    &self,
                ) -> Option<(
                    $crate::table::TableContext,
                    i32,
                    i32,
                    $crate::table::TableResizeFlag,
                )> {

                    let mut r = 0;
                    let mut c = 0;
                    let mut flag = 0;
                    unsafe {
                        let ret = [<$flname _cursor2rowcol>](self.inner.widget() as _, &mut r, &mut c, &mut flag);
                        let ctx: $crate::table::TableContext = std::mem::transmute(ret);
                        if ctx == $crate::table::TableContext::None {
                            None
                        } else {
                            let flag: $crate::table::TableResizeFlag = std::mem::transmute(flag);
                            Some((ctx, r, c, flag))
                        }
                    }
                }
            }
        }
    };
}

pub use impl_table_ext;
