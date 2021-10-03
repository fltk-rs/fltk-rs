/// Implements TableExt
#[macro_export]
macro_rules! impl_table_ext {
    ($name: ident, $flname: ident) => {
        paste! {
            unsafe impl TableExt for $name {
                fn clear(&mut self) {
                    unsafe {
                        assert!(!self.was_deleted());
                        [<$flname _clear>](self.inner)
                    }
                }

                fn set_table_frame(&mut self, frame: FrameType) {
                    unsafe {
                        assert!(!self.was_deleted());
                        [<$flname _set_table_box>](self.inner, frame as i32)
                    }
                }

                fn table_frame(&self) -> FrameType {
                    unsafe {
                        assert!(!self.was_deleted());
                        mem::transmute([<$flname _table_box>](self.inner))
                    }
                }

                fn set_rows(&mut self, val: i32) {
                    unsafe {
                        assert!(!self.was_deleted());
                        [<$flname _set_rows>](self.inner, val as i32)
                    }
                }

                fn rows(&self) -> i32 {
                    unsafe {
                        assert!(!self.was_deleted());
                        [<$flname _rows>](self.inner) as i32
                    }
                }

                fn set_cols(&mut self, val: i32) {
                    unsafe {
                        assert!(!self.was_deleted());
                        [<$flname _set_cols>](self.inner, val as i32)
                    }
                }

                fn cols(&self) -> i32 {
                    unsafe {
                        assert!(!self.was_deleted());
                        [<$flname _cols>](self.inner) as i32
                    }
                }

                fn visible_cells(&self) -> (i32, i32, i32, i32) {
                    unsafe {
                        assert!(!self.was_deleted());
                        let mut row_top = 0;
                        let mut col_left = 0;
                        let mut row_bot = 0;
                        let mut col_right = 0;
                        [<$flname _visible_cells>](
                            self.inner,
                            &mut row_top,
                            &mut col_left,
                            &mut row_bot,
                            &mut col_right,
                        );
                        (row_top, col_left, row_bot, col_right)
                    }
                }

                fn is_interactive_resize(&self) -> bool {
                    unsafe {
                        assert!(!self.was_deleted());
                        [<$flname _is_interactive_resize>](self.inner) != 0
                    }
                }

                fn row_resize(&self) -> bool {
                    unsafe {
                        assert!(!self.was_deleted());
                        [<$flname _row_resize>](self.inner) != 0
                    }
                }

                fn set_row_resize(&mut self, flag: bool) {
                    unsafe {
                        assert!(!self.was_deleted());
                        [<$flname _set_row_resize>](self.inner, flag as i32)
                    }
                }

                fn col_resize(&self) -> bool {
                    unsafe {
                        assert!(!self.was_deleted());
                        [<$flname _col_resize>](self.inner) != 0
                    }
                }

                fn set_col_resize(&mut self, flag: bool) {
                    unsafe {
                        assert!(!self.was_deleted());
                        [<$flname _set_col_resize>](self.inner, flag as i32)
                    }
                }

                fn col_resize_min(&self) -> i32 {
                    unsafe {
                        assert!(!self.was_deleted());
                        [<$flname _col_resize_min>](self.inner) as i32
                    }
                }

                fn set_col_resize_min(&mut self, val: i32) {
                    unsafe {
                        assert!(!self.was_deleted());
                        [<$flname _set_col_resize_min>](self.inner, val as i32)
                    }
                }

                fn row_resize_min(&self) -> i32 {
                    unsafe {
                        assert!(!self.was_deleted());
                        [<$flname _row_resize_min>](self.inner) as i32
                    }
                }

                fn set_row_resize_min(&mut self, val: i32) {
                    unsafe {
                        assert!(!self.was_deleted());
                        [<$flname _set_row_resize_min>](self.inner, val as i32)
                    }
                }

                fn row_header(&self) -> bool {
                    unsafe {
                        assert!(!self.was_deleted());
                        [<$flname _row_header>](self.inner) != 0
                    }
                }

                fn set_row_header(&mut self, flag: bool) {
                    unsafe {
                        assert!(!self.was_deleted());
                        [<$flname _set_row_header>](self.inner, flag as i32)
                    }
                }

                fn col_header(&self) -> bool {
                    unsafe {
                        assert!(!self.was_deleted());
                        [<$flname _col_header>](self.inner) != 0
                    }
                }

                fn set_col_header(&mut self, flag: bool) {
                    unsafe {
                        assert!(!self.was_deleted());
                        [<$flname _set_col_header>](self.inner, flag as i32)
                    }
                }

                fn set_col_header_height(&mut self, height: i32) {
                    unsafe {
                        assert!(!self.was_deleted());
                        [<$flname _set_col_header_height>](self.inner, height)
                    }
                }

                fn col_header_height(&self) -> i32 {
                    unsafe {
                        assert!(!self.was_deleted());
                        [<$flname _col_header_height>](self.inner)
                    }
                }

                fn set_row_header_width(&mut self, width: i32) {
                    unsafe {
                        assert!(!self.was_deleted());
                        [<$flname _set_row_header_width>](self.inner, width)
                    }
                }

                fn row_header_width(&self) -> i32 {
                    unsafe {
                        assert!(!self.was_deleted());
                        [<$flname _row_header_width>](self.inner)
                    }
                }

                fn set_row_header_color(&mut self, val: Color) {
                    unsafe {
                        assert!(!self.was_deleted());
                        [<$flname _set_row_header_color>](self.inner, val.bits() as u32)
                    }
                }

                fn row_header_color(&self) -> Color {
                    unsafe {
                        assert!(!self.was_deleted());
                        mem::transmute([<$flname _row_header_color>](self.inner))
                    }
                }

                fn set_col_header_color(&mut self, val: Color) {
                    unsafe {
                        assert!(!self.was_deleted());
                        [<$flname _set_col_header_color>](self.inner, val.bits() as u32)
                    }
                }

                fn col_header_color(&self) -> Color {
                    unsafe {
                        assert!(!self.was_deleted());
                        mem::transmute([<$flname _col_header_color>](self.inner))
                    }
                }

                fn set_row_height(&mut self, row: i32, height: i32) {
                    unsafe {
                        assert!(!self.was_deleted());
                        [<$flname _set_row_height>](self.inner, row, height)
                    }
                }

                fn row_height(&self, row: i32) -> i32 {
                    unsafe {
                        assert!(!self.was_deleted());
                        [<$flname _row_height>](self.inner, row)
                    }
                }

                fn set_col_width(&mut self, col: i32, width: i32) {
                    unsafe {
                        assert!(!self.was_deleted());
                        [<$flname _set_col_width>](self.inner, col, width)
                    }
                }

                fn col_width(&self, col: i32) -> i32 {
                    unsafe {
                        assert!(!self.was_deleted());
                        [<$flname _col_width>](self.inner, col)
                    }
                }

                fn set_row_height_all(&mut self, height: i32) {
                    unsafe {
                        assert!(!self.was_deleted());
                        [<$flname _set_row_height_all>](self.inner, height)
                    }
                }

                fn set_col_width_all(&mut self, width: i32) {
                    unsafe {
                        assert!(!self.was_deleted());
                        [<$flname _set_col_width_all>](self.inner, width)
                    }
                }

                fn set_row_position(&mut self, row: i32) {
                    unsafe {
                        assert!(!self.was_deleted());
                        [<$flname _set_row_position>](self.inner, row as i32)
                    }
                }

                fn set_col_position(&mut self, col: i32) {
                    unsafe {
                        assert!(!self.was_deleted());
                        [<$flname _set_col_position>](self.inner, col as i32)
                    }
                }

                fn row_position(&self) -> i32 {
                    unsafe {
                        assert!(!self.was_deleted());
                        [<$flname _row_position>](self.inner) as i32
                    }
                }

                fn col_position(&self) -> i32 {
                    unsafe {
                        assert!(!self.was_deleted());
                        [<$flname _col_position>](self.inner) as i32
                    }
                }

                fn set_top_row(&mut self, row: i32) {
                    unsafe {
                        assert!(!self.was_deleted());
                        [<$flname _set_top_row>](self.inner, row as i32)
                    }
                }

                fn top_row(&self) -> i32 {
                    unsafe {
                        assert!(!self.was_deleted());
                        [<$flname _top_row>](self.inner) as i32
                    }
                }

                fn is_selected(&self, r: i32, c: i32) -> bool {
                    unsafe {
                        assert!(!self.was_deleted());
                        [<$flname _is_selected>](self.inner, r, c) != 0
                    }
                }

                fn get_selection(&self) -> (i32, i32, i32, i32) {
                    unsafe {
                        assert!(!self.was_deleted());
                        let mut row_top = 0;
                        let mut col_left = 0;
                        let mut row_bot = 0;
                        let mut col_right = 0;
                        [<$flname _get_selection>](
                            self.inner,
                            &mut row_top,
                            &mut col_left,
                            &mut row_bot,
                            &mut col_right,
                        );
                        (row_top, col_left, row_bot, col_right)
                    }
                }

                fn set_selection(&mut self, row_top: i32, col_left: i32, row_bot: i32, col_right: i32) {
                    unsafe {
                        assert!(!self.was_deleted());
                        [<$flname _set_selection>](
                            self.inner, row_top, col_left, row_bot, col_right,
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
                        assert!(!self.was_deleted());
                        let x = [<$flname _move_cursor_with_shiftselect>](
                            self.inner,
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
                        assert!(!self.was_deleted());
                        let x = [<$flname _move_cursor>](self.inner, r, c);
                        if x == 0 {
                            return Err(FltkError::Internal(FltkErrorKind::FailedOperation));
                        }
                        Ok(())
                    }
                }

                fn scrollbar_size(&self) -> i32 {
                    unsafe {
                        assert!(!self.was_deleted());
                        [<$flname _scrollbar_size>](self.inner) as i32
                    }
                }

                fn set_scrollbar_size(&mut self, new_size: i32) {
                    unsafe {
                        assert!(!self.was_deleted());
                        [<$flname _set_scrollbar_size>](self.inner, new_size as i32)
                    }
                }

                fn set_tab_cell_nav(&mut self, val: bool) {
                    unsafe {
                        assert!(!self.was_deleted());
                        [<$flname _set_tab_cell_nav>](self.inner, val as i32)
                    }
                }

                fn tab_cell_nav(&self) -> bool {
                    unsafe {
                        assert!(!self.was_deleted());
                        [<$flname _tab_cell_nav>](self.inner) != 0
                    }
                }

                fn draw_cell<
                    F: FnMut(&mut Self, crate::table::TableContext, i32, i32, i32, i32, i32, i32)
                        + 'static,
                >(
                    &mut self,
                    cb: F,
                ) {
                    assert!(!self.was_deleted());
                    pub type CustomDrawCellCallback = Option<
                        unsafe extern "C" fn(
                            wid: *mut Fl_Widget,
                            ctx: raw::c_int,
                            arg2: raw::c_int,
                            arg3: raw::c_int,
                            arg4: raw::c_int,
                            arg5: raw::c_int,
                            arg6: raw::c_int,
                            arg7: raw::c_int,
                            data: *mut raw::c_void,
                        ),
                    >;
                    unsafe {
                        unsafe extern "C" fn shim(
                            wid: *mut Fl_Widget,
                            ctx: raw::c_int,
                            arg2: raw::c_int,
                            arg3: raw::c_int,
                            arg4: raw::c_int,
                            arg5: raw::c_int,
                            arg6: raw::c_int,
                            arg7: raw::c_int,
                            data: *mut raw::c_void,
                        ) {
                            let mut wid = $name::from_widget_ptr(wid as *mut _);
                            let ctx: TableContext = mem::transmute(ctx);
                            let a: *mut Box<
                                dyn FnMut(
                                    &mut $name,
                                    crate::table::TableContext,
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
                                    crate::table::TableContext,
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
                                crate::table::TableContext,
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
                        let _old_data = self.draw_cell_data();
                        let a: *mut Box<
                            dyn FnMut(
                                &mut Self,
                                crate::table::TableContext,
                                i32,
                                i32,
                                i32,
                                i32,
                                i32,
                                i32,
                            ),
                        > = Box::into_raw(Box::new(Box::new(cb)));
                        let data: *mut raw::c_void = a as *mut raw::c_void;
                        let callback: CustomDrawCellCallback = Some(shim);
                        [<$flname _draw_cell>](self.inner, callback, data);
                    }
                }

                unsafe fn draw_cell_data(&self) -> Option<Box<dyn FnMut()>> {
                    let ptr = [<$flname _draw_cell_data>](self.inner);
                    if ptr.is_null() {
                        None
                    } else {
                        let data = ptr as *mut Box<dyn FnMut()>;
                        let data = Box::from_raw(data);
                        Some(*data)
                    }
                }

                fn callback_col(&self) -> i32 {
                    unsafe { [<$flname _callback_col>](self.inner) }
                }

                fn callback_row(&self) -> i32 {
                    unsafe { [<$flname _callback_row>](self.inner) }
                }

                fn callback_context(&self) -> TableContext {
                    unsafe { mem::transmute([<$flname _callback_context>](self.inner)) }
                }

                fn scrollbar(&self) -> crate::valuator::Scrollbar {
                    assert!(!self.was_deleted());
                    unsafe {
                        let ptr = [<$flname _scrollbar>](self.inner);
                        assert!(!ptr.is_null());
                        crate::valuator::Scrollbar::from_widget_ptr(
                            ptr as *mut fltk_sys::widget::Fl_Widget,
                        )
                    }
                }

                fn hscrollbar(&self) -> crate::valuator::Scrollbar {
                    assert!(!self.was_deleted());
                    unsafe {
                        let ptr = [<$flname _hscrollbar>](self.inner);
                        assert!(!ptr.is_null());
                        crate::valuator::Scrollbar::from_widget_ptr(
                            ptr as *mut fltk_sys::widget::Fl_Widget,
                        )
                    }
                }
            }
        }
    };
}

pub(crate) use impl_table_ext;