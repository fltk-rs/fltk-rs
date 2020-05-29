use crate::utils::get_fl_name;
use proc_macro::TokenStream;
use quote::*;
use syn::*;

pub fn impl_table_trait(ast: &DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let name_str = get_fl_name(name.to_string());

    let new = Ident::new(format!("{}_{}", name_str, "new").as_str(), name.span());
    let clear = Ident::new(format!("{}_{}", name_str, "clear").as_str(), name.span());
    let set_table_box = Ident::new(
        format!("{}_{}", name_str, "set_table_box").as_str(),
        name.span(),
    );
    let table_box = Ident::new(
        format!("{}_{}", name_str, "table_box").as_str(),
        name.span(),
    );
    let set_rows = Ident::new(format!("{}_{}", name_str, "set_rows").as_str(), name.span());
    let rows = Ident::new(format!("{}_{}", name_str, "rows").as_str(), name.span());
    let set_cols = Ident::new(format!("{}_{}", name_str, "set_cols").as_str(), name.span());
    let cols = Ident::new(format!("{}_{}", name_str, "cols").as_str(), name.span());
    let visible_cells = Ident::new(
        format!("{}_{}", name_str, "visible_cells").as_str(),
        name.span(),
    );
    let is_interactive_resize = Ident::new(
        format!("{}_{}", name_str, "is_interactive_resize").as_str(),
        name.span(),
    );
    let row_resize = Ident::new(
        format!("{}_{}", name_str, "row_resize").as_str(),
        name.span(),
    );
    let set_row_resize = Ident::new(
        format!("{}_{}", name_str, "set_row_resize").as_str(),
        name.span(),
    );
    let col_resize = Ident::new(
        format!("{}_{}", name_str, "col_resize").as_str(),
        name.span(),
    );
    let set_col_resize = Ident::new(
        format!("{}_{}", name_str, "set_col_resize").as_str(),
        name.span(),
    );
    let col_resize_min = Ident::new(
        format!("{}_{}", name_str, "col_resize_min").as_str(),
        name.span(),
    );
    let set_col_resize_min = Ident::new(
        format!("{}_{}", name_str, "set_col_resize_min").as_str(),
        name.span(),
    );
    let row_resize_min = Ident::new(
        format!("{}_{}", name_str, "row_resize_min").as_str(),
        name.span(),
    );
    let set_row_resize_min = Ident::new(
        format!("{}_{}", name_str, "set_row_resize_min").as_str(),
        name.span(),
    );
    let row_header = Ident::new(
        format!("{}_{}", name_str, "row_header").as_str(),
        name.span(),
    );
    let set_row_header = Ident::new(
        format!("{}_{}", name_str, "set_row_header").as_str(),
        name.span(),
    );
    let col_header = Ident::new(
        format!("{}_{}", name_str, "col_header").as_str(),
        name.span(),
    );
    let set_col_header = Ident::new(
        format!("{}_{}", name_str, "set_col_header").as_str(),
        name.span(),
    );
    let set_col_header_height = Ident::new(
        format!("{}_{}", name_str, "set_col_header_height").as_str(),
        name.span(),
    );
    let col_header_height = Ident::new(
        format!("{}_{}", name_str, "col_header_height").as_str(),
        name.span(),
    );
    let set_row_header_width = Ident::new(
        format!("{}_{}", name_str, "set_row_header_width").as_str(),
        name.span(),
    );
    let row_header_width = Ident::new(
        format!("{}_{}", name_str, "row_header_width").as_str(),
        name.span(),
    );
    let set_row_header_color = Ident::new(
        format!("{}_{}", name_str, "set_row_header_color").as_str(),
        name.span(),
    );
    let row_header_color = Ident::new(
        format!("{}_{}", name_str, "row_header_color").as_str(),
        name.span(),
    );
    let set_col_header_color = Ident::new(
        format!("{}_{}", name_str, "set_col_header_color").as_str(),
        name.span(),
    );
    let col_header_color = Ident::new(
        format!("{}_{}", name_str, "col_header_color").as_str(),
        name.span(),
    );
    let set_row_height = Ident::new(
        format!("{}_{}", name_str, "set_row_height").as_str(),
        name.span(),
    );
    let row_height = Ident::new(
        format!("{}_{}", name_str, "row_height").as_str(),
        name.span(),
    );
    let set_col_width = Ident::new(
        format!("{}_{}", name_str, "set_col_width").as_str(),
        name.span(),
    );
    let col_width = Ident::new(
        format!("{}_{}", name_str, "col_width").as_str(),
        name.span(),
    );
    let set_row_height_all = Ident::new(
        format!("{}_{}", name_str, "set_row_height_all").as_str(),
        name.span(),
    );
    let set_col_width_all = Ident::new(
        format!("{}_{}", name_str, "set_col_width_all").as_str(),
        name.span(),
    );
    let set_row_position = Ident::new(
        format!("{}_{}", name_str, "set_row_position").as_str(),
        name.span(),
    );
    let set_col_position = Ident::new(
        format!("{}_{}", name_str, "set_col_position").as_str(),
        name.span(),
    );
    let row_position = Ident::new(
        format!("{}_{}", name_str, "row_position").as_str(),
        name.span(),
    );
    let col_position = Ident::new(
        format!("{}_{}", name_str, "col_position").as_str(),
        name.span(),
    );
    let set_top_row = Ident::new(
        format!("{}_{}", name_str, "set_top_row").as_str(),
        name.span(),
    );
    let top_row = Ident::new(format!("{}_{}", name_str, "top_row").as_str(), name.span());
    let is_selected = Ident::new(
        format!("{}_{}", name_str, "is_selected").as_str(),
        name.span(),
    );
    let get_selection = Ident::new(
        format!("{}_{}", name_str, "get_selection").as_str(),
        name.span(),
    );
    let set_selection = Ident::new(
        format!("{}_{}", name_str, "set_selection").as_str(),
        name.span(),
    );
    let move_cursor_with_shiftselect = Ident::new(
        format!("{}_{}", name_str, "move_cursor_with_shiftselect").as_str(),
        name.span(),
    );
    let move_cursor = Ident::new(
        format!("{}_{}", name_str, "move_cursor").as_str(),
        name.span(),
    );
    let resize = Ident::new(format!("{}_{}", name_str, "resize").as_str(), name.span());
    let init_sizes = Ident::new(
        format!("{}_{}", name_str, "init_sizes").as_str(),
        name.span(),
    );
    let scrollbar_size = Ident::new(
        format!("{}_{}", name_str, "scrollbar_size").as_str(),
        name.span(),
    );
    let set_scrollbar_size = Ident::new(
        format!("{}_{}", name_str, "set_scrollbar_size").as_str(),
        name.span(),
    );
    let set_tab_cell_nav = Ident::new(
        format!("{}_{}", name_str, "set_tab_cell_nav").as_str(),
        name.span(),
    );
    let tab_cell_nav = Ident::new(
        format!("{}_{}", name_str, "tab_cell_nav").as_str(),
        name.span(),
    );

    let gen = quote! {
        unsafe impl TableExt for #name {
            fn clear(&mut self) {
                unsafe {
                    assert!(!self.was_deleted());
                    #clear(self._inner)
                }
            }

            fn set_table_frame(&mut self, frame: FrameType) {
                unsafe {
                    assert!(!self.was_deleted());
                    #set_table_box(self._inner, frame as i32)
                }
            }

            fn table_frame(&self) -> FrameType {
                unsafe {
                    assert!(!self.was_deleted());
                    mem::transmute(#table_box(self._inner))
                }
            }

            fn set_rows(&mut self, val: u32) {
                debug_assert!(val <= std::i32::MAX as u32, "u32 entries have to be < std::i32::MAX for compatibility!");
                unsafe {
                    assert!(!self.was_deleted());
                    #set_rows(self._inner, val as i32)
                }
            }

            fn rows(&self) -> u32 {
                unsafe {
                    assert!(!self.was_deleted());
                    #rows(self._inner) as u32
                }
            }

            fn set_cols(&mut self, val: u32) {
                debug_assert!(val <= std::i32::MAX as u32, "u32 entries have to be < std::i32::MAX for compatibility!");
                unsafe {
                    assert!(!self.was_deleted());
                    #set_cols(self._inner, val as i32)
                }
            }

            fn cols(&self) -> u32 {
                unsafe {
                    assert!(!self.was_deleted());
                    #cols(self._inner) as u32
                }
            }

            fn visible_cells( &mut self, r1: &mut i32, r2: &mut i32, c1: &mut i32, c2: &mut i32,) {
                unsafe {
                    assert!(!self.was_deleted());
                    #visible_cells(self._inner, r1, r2, c1, c2)
                }
            }

            fn is_interactive_resize(&self) -> bool {
                unsafe {
                    assert!(!self.was_deleted());
                    match #is_interactive_resize(self._inner) {
                        0 => false,
                        _ => true,
                    }
                }
            }

            fn row_resize(&self) -> bool {
                unsafe {
                    assert!(!self.was_deleted());
                    match #row_resize(self._inner) {
                        0 => false,
                        _ => true,
                    }
                }
            }

            fn set_row_resize(&mut self, flag: bool) {
                unsafe {
                    assert!(!self.was_deleted());
                    #set_row_resize(self._inner, flag as i32)
                }
            }

            fn col_resize(&self) -> bool {
                unsafe {
                    assert!(!self.was_deleted());
                    match #col_resize(self._inner) {
                        0 => false,
                        _ => true,
                    }
                }
            }

            fn set_col_resize(&mut self, flag: bool) {
                unsafe {
                    assert!(!self.was_deleted());
                    #set_col_resize(self._inner, flag as i32)
                }
            }

            fn col_resize_min(&self) -> u32 {
                unsafe {
                    assert!(!self.was_deleted());
                    #col_resize_min(self._inner)  as u32
                }
            }

            fn set_col_resize_min(&mut self, val: u32) {
                debug_assert!(val <= std::i32::MAX as u32, "u32 entries have to be < std::i32::MAX for compatibility!");
                unsafe {
                    assert!(!self.was_deleted());
                    #set_col_resize_min(self._inner, val as i32)
                }
            }

            fn row_resize_min(&self) -> u32 {
                unsafe {
                    assert!(!self.was_deleted());
                    #row_resize_min(self._inner) as u32
                }
            }

            fn set_row_resize_min(&mut self, val: u32) {
                debug_assert!(val <= std::i32::MAX as u32, "u32 entries have to be < std::i32::MAX for compatibility!");
                unsafe {
                    assert!(!self.was_deleted());
                    #set_row_resize_min(self._inner, val as i32)
                }
            }

            fn row_header(&self) -> bool {
                unsafe {
                    assert!(!self.was_deleted());
                    match #row_header(self._inner) {
                        0 => false,
                        _ => true,
                    }
                }
            }

            fn set_row_header(&mut self, flag: bool) {
                unsafe {
                    assert!(!self.was_deleted());
                    #set_row_header(self._inner, flag as i32)
                }
            }

            fn col_header(&self) -> bool {
                unsafe {
                    assert!(!self.was_deleted());
                    match #col_header(self._inner) {
                        0 => false,
                        _ => true,
                    }
                }
            }

            fn set_col_header(&mut self, flag: bool) {
                unsafe {
                    assert!(!self.was_deleted());
                    #set_col_header(self._inner, flag as i32)
                }
            }

            fn set_col_header_height(&mut self, height: i32) {
                unsafe {
                    assert!(!self.was_deleted());
                    #set_col_header_height(self._inner, height)
                }
            }

            fn col_header_height(&self) -> i32 {
                unsafe {
                    assert!(!self.was_deleted());
                    #col_header_height(self._inner)
                }
            }

            fn set_row_header_width(&mut self, width: i32) {
                unsafe {
                    assert!(!self.was_deleted());
                    #set_row_header_width(self._inner, width)
                }
            }

            fn row_header_width(&self) -> i32 {
                unsafe {
                    assert!(!self.was_deleted());
                    #row_header_width(self._inner)
                }
            }

            fn set_row_header_color(&mut self, val: Color) {
                unsafe {
                    assert!(!self.was_deleted());
                    #set_row_header_color(self._inner, val as u32)
                }
            }

            fn row_header_color(&self) -> Color {
                unsafe {
                    assert!(!self.was_deleted());
                    mem::transmute(#row_header_color(self._inner))
                }
            }

            fn set_col_header_color(&mut self, val: Color) {
                unsafe {
                    assert!(!self.was_deleted());
                    #set_col_header_color(self._inner, val as u32)
                }
            }

            fn col_header_color(&self) -> Color {
                unsafe {
                    assert!(!self.was_deleted());
                    mem::transmute(#col_header_color(self._inner))
                }
            }

            fn set_row_height(&mut self, row: i32, height: i32) {
                unsafe {
                    assert!(!self.was_deleted());
                    #set_row_height(self._inner, row, height)
                }
            }

            fn row_height(&self, row: i32) -> i32 {
                unsafe {
                    assert!(!self.was_deleted());
                    #row_height(self._inner, row)
                }
            }

            fn set_col_width(&mut self, col: i32, width: i32) {
                unsafe {
                    assert!(!self.was_deleted());
                    #set_col_width(self._inner, col, width)
                }
            }

            fn col_width(&self, col: i32) -> i32 {
                unsafe {
                    assert!(!self.was_deleted());
                    #col_width(self._inner, col)
                }
            }

            fn set_row_height_all(&mut self, height: i32) {
                unsafe {
                    assert!(!self.was_deleted());
                    #set_row_height_all(self._inner, height)
                }
            }

            fn set_col_width_all(&mut self, width: i32) {
                unsafe {
                    assert!(!self.was_deleted());
                    #set_col_width_all(self._inner, width)
                }
            }

            fn set_row_position(&mut self, row: i32) {
                unsafe {
                    assert!(!self.was_deleted());
                    #set_row_position(self._inner, row as i32)
                }
            }

            fn set_col_position(&mut self, col: i32) {
                unsafe {
                    assert!(!self.was_deleted());
                    #set_col_position(self._inner, col as i32)
                }
            }

            fn row_position(&self) -> i32 {
                unsafe {
                    assert!(!self.was_deleted());
                    #row_position(self._inner) as i32
                }
            }

            fn col_position(&self) -> i32 {
                unsafe {
                    assert!(!self.was_deleted());
                    #col_position(self._inner) as i32
                }
            }

            fn set_top_row(&mut self, row: i32) {
                unsafe {
                    assert!(!self.was_deleted());
                    #set_top_row(self._inner, row as i32)
                }
            }

            fn top_row(&self) -> i32 {
                unsafe {
                    assert!(!self.was_deleted());
                    #top_row(self._inner) as i32
                }
            }

            fn is_selected(&self, r: i32, c: i32) -> bool {
                unsafe {
                    assert!(!self.was_deleted());
                    match #is_selected(self._inner, r, c) {
                        0 => false,
                        _ => true,
                    }
                }
            }

            fn get_selection(&self, row_top: &mut i32, col_left: &mut i32, row_bot: &mut i32, col_right: &mut i32,) {
                unsafe {
                    assert!(!self.was_deleted());
                    #get_selection(self._inner, row_top, col_left, row_bot, col_right)
                }
            }

            fn set_selection(&mut self, row_top: i32, col_left: i32, row_bot: i32, col_right: i32) {
                unsafe {
                    assert!(!self.was_deleted());
                    #set_selection(self._inner, row_top, col_left, row_bot, col_right)
                }
            }

            fn move_cursor_with_shift_select(&mut self, r: i32, c: i32, shiftselect: bool) -> Result<(), FltkError> {
                unsafe {
                    assert!(!self.was_deleted());
                    let x = #move_cursor_with_shiftselect(self._inner, r, c, shiftselect as i32);
                    if x == 0 {
                        return Err(FltkError::Internal(FltkErrorKind::FailedOperation));
                    }
                    Ok(())
                }
            }

            fn move_cursor(&mut self, r: i32, c: i32) -> Result<(), FltkError> {
                unsafe {
                    assert!(!self.was_deleted());
                    let x = #move_cursor(self._inner, r, c);
                    if x == 0 {
                        return Err(FltkError::Internal(FltkErrorKind::FailedOperation));
                    }
                    Ok(())
                }
            }

            fn init_sizes(&mut self) {
                unsafe {
                    assert!(!self.was_deleted());
                    #init_sizes(self._inner)
                }
            }

            fn scrollbar_size(&self) -> u32 {
                unsafe {
                    assert!(!self.was_deleted());
                    #scrollbar_size(self._inner) as u32
                }
            }

            fn set_scrollbar_size(&mut self, new_size: u32) {
                debug_assert!(new_size <= std::i32::MAX as u32, "u32 entries have to be < std::i32::MAX for compatibility!");
                unsafe {
                    assert!(!self.was_deleted());
                    #set_scrollbar_size(self._inner, new_size as i32)
                }
            }

            fn set_tab_cell_nav(&mut self, val: u32) {
                debug_assert!(val <= std::i32::MAX as u32, "u32 entries have to be < std::i32::MAX for compatibility!");
                unsafe {
                    assert!(!self.was_deleted());
                    #set_tab_cell_nav(self._inner, val as i32)
                }
            }

            fn tab_cell_nav(&self) -> u32 {
                unsafe {
                    assert!(!self.was_deleted());
                    #tab_cell_nav(self._inner) as u32
                }
            }
        }
    };
    gen.into()
}
