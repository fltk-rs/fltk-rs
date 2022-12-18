# Roadmap for version 2 (when FLTK 1.4 is released). 

- Update to edition2021.
- Rename WidgetExt::into_widget() to as_widget() and GroupExt::into_group() to as_group() to conform to Rust's self convention.
- Rename TreeItem::try_widget() to widget() and remove old widget() method.
- Rename TreeItem::label_fg/bg_color() and TreeItem::set_label_fg/bg_color() to label_fg/bg_color() and set_label_fg/bg_color(), respectively
- Rename Wizard::try_current_widget() to current_widget() and remove old current_widget() method.
- Rename TableExt::try_get_selection() and try_visible_cells() to get_selection and visible_cells().
- FileChooser::directory() should return a PathBuf.
- Rename timeout3 functions to timeout, as well as idle and clipboard callbacks.
- Support opacity and platform_hide/platform_show for the wayland backend.
- Rename no-pango feature to no-pango-cairo, to better reflect the linked libs.
- app::screen_size() should return (i32, i32) since the implementation changed.
- Refactor drawing code to use Coord and Rect.
- Rust 1.63 constifies Mutex::new, so lazy_static can be removed.
- Replace WindowType::Normal with WindowType::Single.
- Default Flex to Column as is now the default in FLTK.
- Rename Flex::set_size to fixed. 
