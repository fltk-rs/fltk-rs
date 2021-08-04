# Roadmap for version 2 (ETA mid 2022). 

- Remove draw::scale_x() in favor of scale(). 
- Return Option String for WidgetExt::label(). 
- Use i32 for app::set_font_size(). 
- Enable adding a backend (graphics driver) through Rust. 
- Export app submodules. 
- Remove WidgetExt::width() and height() in favor of w() & h(). 
- Add an fltk-extras crate to the current workspace consolidating fltk-theme, fltk-flex, fltk-calendar, fl2rust and fltk-fluid when they've matured enough, these can be individually enabled using feature flags.  
