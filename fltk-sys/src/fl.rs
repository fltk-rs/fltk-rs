/* automatically generated by rust-bindgen 0.71.1 */

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Fl_Widget {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Fl_Widget_Tracker {
    _unused: [u8; 0],
}
pub type Fl_Awake_Handler =
    ::core::option::Option<unsafe extern "C" fn(data: *mut ::core::ffi::c_void)>;
extern "C" {
    pub fn Fl_run() -> ::core::ffi::c_int;
}
extern "C" {
    pub fn Fl_check() -> ::core::ffi::c_int;
}
extern "C" {
    pub fn Fl_ready() -> ::core::ffi::c_int;
}
extern "C" {
    pub fn Fl_release();
}
extern "C" {
    pub fn Fl_reload_scheme() -> ::core::ffi::c_int;
}
extern "C" {
    pub fn Fl_menu_linespacing() -> ::core::ffi::c_int;
}
extern "C" {
    pub fn Fl_set_menu_linespacing(H: ::core::ffi::c_int);
}
extern "C" {
    pub fn Fl_lock() -> ::core::ffi::c_int;
}
extern "C" {
    pub fn Fl_unlock();
}
extern "C" {
    pub fn Fl_awake_callback(
        handler: Fl_Awake_Handler,
        data: *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
}
extern "C" {
    pub fn Fl_awake();
}
extern "C" {
    pub fn Fl_set_scrollbar_size(arg1: ::core::ffi::c_int);
}
extern "C" {
    pub fn Fl_scrollbar_size() -> ::core::ffi::c_int;
}
extern "C" {
    pub fn Fl_event() -> ::core::ffi::c_int;
}
extern "C" {
    pub fn Fl_event_key() -> ::core::ffi::c_int;
}
extern "C" {
    pub fn Fl_event_original_key() -> ::core::ffi::c_int;
}
extern "C" {
    pub fn Fl_event_key_down(arg1: ::core::ffi::c_int) -> ::core::ffi::c_int;
}
extern "C" {
    pub fn Fl_event_text() -> *const ::core::ffi::c_char;
}
extern "C" {
    pub fn Fl_event_button() -> ::core::ffi::c_int;
}
extern "C" {
    pub fn Fl_event_clicks() -> ::core::ffi::c_int;
}
extern "C" {
    pub fn Fl_event_x() -> ::core::ffi::c_int;
}
extern "C" {
    pub fn Fl_event_y() -> ::core::ffi::c_int;
}
extern "C" {
    pub fn Fl_event_x_root() -> ::core::ffi::c_int;
}
extern "C" {
    pub fn Fl_event_y_root() -> ::core::ffi::c_int;
}
extern "C" {
    pub fn Fl_event_dx() -> ::core::ffi::c_int;
}
extern "C" {
    pub fn Fl_event_dy() -> ::core::ffi::c_int;
}
extern "C" {
    pub fn Fl_get_mouse(arg1: *mut ::core::ffi::c_int, arg2: *mut ::core::ffi::c_int);
}
extern "C" {
    pub fn Fl_event_is_click() -> ::core::ffi::c_int;
}
extern "C" {
    pub fn Fl_event_length() -> ::core::ffi::c_int;
}
extern "C" {
    pub fn Fl_event_state() -> ::core::ffi::c_int;
}
extern "C" {
    pub fn Fl_w() -> ::core::ffi::c_int;
}
extern "C" {
    pub fn Fl_h() -> ::core::ffi::c_int;
}
extern "C" {
    pub fn Fl_screen_x() -> ::core::ffi::c_int;
}
extern "C" {
    pub fn Fl_screen_y() -> ::core::ffi::c_int;
}
extern "C" {
    pub fn Fl_screen_h() -> ::core::ffi::c_int;
}
extern "C" {
    pub fn Fl_screen_w() -> ::core::ffi::c_int;
}
extern "C" {
    pub fn Fl_compose(del: *mut ::core::ffi::c_int) -> ::core::ffi::c_int;
}
extern "C" {
    pub fn Fl_compose_reset();
}
extern "C" {
    pub fn Fl_compose_state() -> ::core::ffi::c_int;
}
extern "C" {
    pub fn Fl_copy(
        stuff: *const ::core::ffi::c_char,
        len: ::core::ffi::c_int,
        destination: ::core::ffi::c_int,
    );
}
extern "C" {
    pub fn Fl_paste_text(arg1: *mut Fl_Widget, src: ::core::ffi::c_int);
}
extern "C" {
    pub fn Fl_paste_image(widget: *mut Fl_Widget, src: ::core::ffi::c_int);
}
extern "C" {
    pub fn Fl_set_scheme(scheme: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
}
extern "C" {
    pub fn Fl_scheme() -> ::core::ffi::c_int;
}
extern "C" {
    pub fn Fl_scheme_string() -> *const ::core::ffi::c_char;
}
extern "C" {
    pub fn Fl_visible_focus() -> ::core::ffi::c_int;
}
extern "C" {
    pub fn Fl_set_visible_focus(arg1: ::core::ffi::c_int);
}
extern "C" {
    pub fn Fl_set_box_type(arg1: ::core::ffi::c_int, arg2: ::core::ffi::c_int);
}
extern "C" {
    pub fn Fl_box_shadow_width() -> ::core::ffi::c_int;
}
extern "C" {
    pub fn Fl_set_box_shadow_width(W: ::core::ffi::c_int);
}
extern "C" {
    pub fn Fl_box_border_radius_max() -> ::core::ffi::c_int;
}
extern "C" {
    pub fn Fl_set_box_border_radius_max(R: ::core::ffi::c_int);
}
extern "C" {
    pub fn Fl_get_rgb_color(
        r: ::core::ffi::c_uchar,
        g: ::core::ffi::c_uchar,
        b: ::core::ffi::c_uchar,
    ) -> ::core::ffi::c_uint;
}
extern "C" {
    pub fn Fl_set_color(
        c: ::core::ffi::c_uint,
        r: ::core::ffi::c_uchar,
        g: ::core::ffi::c_uchar,
        b: ::core::ffi::c_uchar,
    );
}
extern "C" {
    pub fn Fl_set_color_with_alpha(
        c: ::core::ffi::c_uint,
        r: ::core::ffi::c_uchar,
        g: ::core::ffi::c_uchar,
        b: ::core::ffi::c_uchar,
        a: ::core::ffi::c_uchar,
    );
}
extern "C" {
    pub fn Fl_get_font(idx: ::core::ffi::c_int) -> *const ::core::ffi::c_char;
}
extern "C" {
    pub fn Fl_get_font_name(idx: ::core::ffi::c_int) -> *const ::core::ffi::c_char;
}
extern "C" {
    pub fn Fl_get_font_name2(
        idx: ::core::ffi::c_int,
        attributes: *mut ::core::ffi::c_int,
    ) -> *const ::core::ffi::c_char;
}
extern "C" {
    pub fn Fl_get_font_sizes(
        font: ::core::ffi::c_int,
        sizep: *mut *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
}
extern "C" {
    pub fn Fl_set_fonts(c: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
}
extern "C" {
    pub fn Fl_set_font(arg1: ::core::ffi::c_int, arg2: ::core::ffi::c_int);
}
extern "C" {
    pub fn Fl_set_font2(arg1: ::core::ffi::c_int, arg2: *const ::core::ffi::c_char);
}
extern "C" {
    pub fn Fl_set_font_size(arg1: ::core::ffi::c_int);
}
extern "C" {
    pub fn Fl_font_size() -> ::core::ffi::c_int;
}
extern "C" {
    pub fn Fl_add_handler(
        ev_handler: ::core::option::Option<
            unsafe extern "C" fn(ev: ::core::ffi::c_int) -> ::core::ffi::c_int,
        >,
    );
}
extern "C" {
    pub fn Fl_awake_msg(msg: *mut ::core::ffi::c_void);
}
extern "C" {
    pub fn Fl_thread_msg() -> *mut ::core::ffi::c_void;
}
extern "C" {
    pub fn Fl_wait() -> ::core::ffi::c_int;
}
extern "C" {
    pub fn Fl_wait_for(arg1: f64) -> f64;
}
extern "C" {
    pub fn Fl_add_timeout(
        t: f64,
        arg1: ::core::option::Option<unsafe extern "C" fn(arg1: *mut ::core::ffi::c_void)>,
        arg2: *mut ::core::ffi::c_void,
    );
}
extern "C" {
    pub fn Fl_repeat_timeout(
        t: f64,
        arg1: ::core::option::Option<unsafe extern "C" fn(arg1: *mut ::core::ffi::c_void)>,
        arg2: *mut ::core::ffi::c_void,
    );
}
extern "C" {
    pub fn Fl_remove_timeout(
        arg1: ::core::option::Option<unsafe extern "C" fn(arg1: *mut ::core::ffi::c_void)>,
        arg2: *mut ::core::ffi::c_void,
    );
}
extern "C" {
    pub fn Fl_has_timeout(
        arg1: ::core::option::Option<unsafe extern "C" fn(arg1: *mut ::core::ffi::c_void)>,
        arg2: *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
}
extern "C" {
    pub fn Fl_dnd() -> ::core::ffi::c_int;
}
extern "C" {
    pub fn Fl_grab() -> *mut ::core::ffi::c_void;
}
extern "C" {
    pub fn Fl_set_grab(arg1: *mut ::core::ffi::c_void);
}
extern "C" {
    pub fn Fl_first_window() -> *mut ::core::ffi::c_void;
}
extern "C" {
    pub fn Fl_next_window(arg1: *const ::core::ffi::c_void) -> *mut ::core::ffi::c_void;
}
extern "C" {
    pub fn Fl_modal() -> *mut ::core::ffi::c_void;
}
extern "C" {
    pub fn Fl_should_program_quit() -> ::core::ffi::c_int;
}
extern "C" {
    pub fn Fl_program_should_quit(flag: ::core::ffi::c_int);
}
extern "C" {
    pub fn Fl_event_inside(
        arg1: ::core::ffi::c_int,
        arg2: ::core::ffi::c_int,
        arg3: ::core::ffi::c_int,
        arg4: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
}
extern "C" {
    pub fn Fl_belowmouse() -> *mut Fl_Widget;
}
extern "C" {
    pub fn Fl_set_belowmouse(w: *mut Fl_Widget);
}
extern "C" {
    pub fn Fl_delete_widget(w: *mut Fl_Widget);
}
extern "C" {
    pub fn Fl_Widget_Tracker_new(w: *mut Fl_Widget) -> *mut Fl_Widget_Tracker;
}
extern "C" {
    pub fn Fl_Widget_Tracker_deleted(self_: *mut Fl_Widget_Tracker) -> ::core::ffi::c_int;
}
extern "C" {
    pub fn Fl_Widget_Tracker_delete(self_: *mut Fl_Widget_Tracker);
}
extern "C" {
    pub fn Fl_init_all();
}
extern "C" {
    pub fn Fl_redraw();
}
extern "C" {
    pub fn Fl_event_shift() -> ::core::ffi::c_int;
}
extern "C" {
    pub fn Fl_event_ctrl() -> ::core::ffi::c_int;
}
extern "C" {
    pub fn Fl_event_command() -> ::core::ffi::c_int;
}
extern "C" {
    pub fn Fl_event_alt() -> ::core::ffi::c_int;
}
extern "C" {
    pub fn Fl_set_damage(flag: ::core::ffi::c_int);
}
extern "C" {
    pub fn Fl_damage() -> ::core::ffi::c_int;
}
extern "C" {
    pub fn Fl_visual(arg1: ::core::ffi::c_int) -> ::core::ffi::c_int;
}
extern "C" {
    pub fn Fl_own_colormap();
}
extern "C" {
    pub fn Fl_pushed() -> *mut Fl_Widget;
}
extern "C" {
    pub fn Fl_focus() -> *mut Fl_Widget;
}
extern "C" {
    pub fn Fl_set_focus(arg1: *mut ::core::ffi::c_void);
}
extern "C" {
    pub fn Fl_version() -> f64;
}
extern "C" {
    pub fn Fl_api_version() -> ::core::ffi::c_int;
}
extern "C" {
    pub fn Fl_abi_version() -> ::core::ffi::c_int;
}
extern "C" {
    pub fn Fl_load_font(path: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
}
extern "C" {
    pub fn Fl_unload_font(path: *const ::core::ffi::c_char);
}
extern "C" {
    pub fn Fl_foreground(r: ::core::ffi::c_uchar, g: ::core::ffi::c_uchar, b: ::core::ffi::c_uchar);
}
extern "C" {
    pub fn Fl_background(r: ::core::ffi::c_uchar, g: ::core::ffi::c_uchar, b: ::core::ffi::c_uchar);
}
extern "C" {
    pub fn Fl_background2(
        r: ::core::ffi::c_uchar,
        g: ::core::ffi::c_uchar,
        b: ::core::ffi::c_uchar,
    );
}
extern "C" {
    pub fn Fl_selection_color(
        r: ::core::ffi::c_uchar,
        g: ::core::ffi::c_uchar,
        b: ::core::ffi::c_uchar,
    );
}
extern "C" {
    pub fn Fl_inactive_color(
        r: ::core::ffi::c_uchar,
        g: ::core::ffi::c_uchar,
        b: ::core::ffi::c_uchar,
    );
}
extern "C" {
    pub fn Fl_get_system_colors();
}
extern "C" {
    pub fn Fl_handle(
        arg1: ::core::ffi::c_int,
        arg2: *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
}
extern "C" {
    pub fn Fl_handle_(
        arg1: ::core::ffi::c_int,
        arg2: *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
}
extern "C" {
    pub fn Fl_add_idle(
        arg1: ::core::option::Option<unsafe extern "C" fn(arg1: *mut ::core::ffi::c_void)>,
        arg2: *mut ::core::ffi::c_void,
    );
}
extern "C" {
    pub fn Fl_has_idle(
        arg1: ::core::option::Option<unsafe extern "C" fn(arg1: *mut ::core::ffi::c_void)>,
        arg2: *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
}
extern "C" {
    pub fn Fl_remove_idle(
        arg1: ::core::option::Option<unsafe extern "C" fn(arg1: *mut ::core::ffi::c_void)>,
        arg2: *mut ::core::ffi::c_void,
    );
}
extern "C" {
    pub fn Fl_add_check(
        arg1: ::core::option::Option<unsafe extern "C" fn(arg1: *mut ::core::ffi::c_void)>,
        arg2: *mut ::core::ffi::c_void,
    );
}
extern "C" {
    pub fn Fl_has_check(
        arg1: ::core::option::Option<unsafe extern "C" fn(arg1: *mut ::core::ffi::c_void)>,
        arg2: *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
}
extern "C" {
    pub fn Fl_remove_check(
        arg1: ::core::option::Option<unsafe extern "C" fn(arg1: *mut ::core::ffi::c_void)>,
        arg2: *mut ::core::ffi::c_void,
    );
}
extern "C" {
    pub fn Fl_flush();
}
extern "C" {
    pub fn Fl_set_screen_scale(n: ::core::ffi::c_int, val: f32);
}
extern "C" {
    pub fn Fl_screen_scale(n: ::core::ffi::c_int) -> f32;
}
extern "C" {
    pub fn Fl_screen_scaling_supported() -> ::core::ffi::c_int;
}
extern "C" {
    pub fn Fl_screen_count() -> ::core::ffi::c_int;
}
extern "C" {
    pub fn Fl_screen_num(x: ::core::ffi::c_int, y: ::core::ffi::c_int) -> ::core::ffi::c_int;
}
extern "C" {
    pub fn Fl_screen_num_inside(
        x: ::core::ffi::c_int,
        y: ::core::ffi::c_int,
        w: ::core::ffi::c_int,
        h: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
}
extern "C" {
    pub fn Fl_screen_xywh(
        X: *mut ::core::ffi::c_int,
        Y: *mut ::core::ffi::c_int,
        W: *mut ::core::ffi::c_int,
        H: *mut ::core::ffi::c_int,
        n: ::core::ffi::c_int,
    );
}
extern "C" {
    pub fn Fl_screen_xywh_at(
        X: *mut ::core::ffi::c_int,
        Y: *mut ::core::ffi::c_int,
        W: *mut ::core::ffi::c_int,
        H: *mut ::core::ffi::c_int,
        mx: ::core::ffi::c_int,
        my: ::core::ffi::c_int,
    );
}
extern "C" {
    pub fn Fl_screen_xywh_inside(
        X: *mut ::core::ffi::c_int,
        Y: *mut ::core::ffi::c_int,
        W: *mut ::core::ffi::c_int,
        H: *mut ::core::ffi::c_int,
        mx: ::core::ffi::c_int,
        my: ::core::ffi::c_int,
        mw: ::core::ffi::c_int,
        mh: ::core::ffi::c_int,
    );
}
extern "C" {
    pub fn Fl_screen_xywh_mouse(
        X: *mut ::core::ffi::c_int,
        Y: *mut ::core::ffi::c_int,
        W: *mut ::core::ffi::c_int,
        H: *mut ::core::ffi::c_int,
    );
}
extern "C" {
    pub fn Fl_screen_dpi(h: *mut f32, v: *mut f32, n: ::core::ffi::c_int);
}
extern "C" {
    pub fn Fl_screen_work_area(
        X: *mut ::core::ffi::c_int,
        Y: *mut ::core::ffi::c_int,
        W: *mut ::core::ffi::c_int,
        H: *mut ::core::ffi::c_int,
        n: ::core::ffi::c_int,
    );
}
extern "C" {
    pub fn Fl_screen_work_area_at(
        X: *mut ::core::ffi::c_int,
        Y: *mut ::core::ffi::c_int,
        W: *mut ::core::ffi::c_int,
        H: *mut ::core::ffi::c_int,
        mx: ::core::ffi::c_int,
        my: ::core::ffi::c_int,
    );
}
extern "C" {
    pub fn Fl_screen_work_area_mouse(
        X: *mut ::core::ffi::c_int,
        Y: *mut ::core::ffi::c_int,
        W: *mut ::core::ffi::c_int,
        H: *mut ::core::ffi::c_int,
    );
}
extern "C" {
    pub fn Fl_keyboard_screen_scaling(value: ::core::ffi::c_int);
}
extern "C" {
    pub fn Fl_open_display();
}
extern "C" {
    pub fn Fl_close_display();
}
extern "C" {
    pub fn Fl_box_dx(boxtype: ::core::ffi::c_int) -> ::core::ffi::c_int;
}
extern "C" {
    pub fn Fl_box_dy(boxtype: ::core::ffi::c_int) -> ::core::ffi::c_int;
}
extern "C" {
    pub fn Fl_box_dw(boxtype: ::core::ffi::c_int) -> ::core::ffi::c_int;
}
extern "C" {
    pub fn Fl_box_dh(boxtype: ::core::ffi::c_int) -> ::core::ffi::c_int;
}
extern "C" {
    pub fn Fl_mac_os_version() -> ::core::ffi::c_int;
}
extern "C" {
    pub fn Fl_event_clipboard() -> *mut ::core::ffi::c_void;
}
extern "C" {
    pub fn Fl_event_clipboard_type() -> *const ::core::ffi::c_char;
}
extern "C" {
    pub fn Fl_clipboard_contains(type_: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
}
extern "C" {
    pub fn Fl_event_dispatch(
        cb: ::core::option::Option<
            unsafe extern "C" fn(
                event: ::core::ffi::c_int,
                arg1: *mut ::core::ffi::c_void,
            ) -> ::core::ffi::c_int,
        >,
    );
}
extern "C" {
    pub fn Fl_inactive(c: ::core::ffi::c_uint) -> ::core::ffi::c_uint;
}
extern "C" {
    pub fn Fl_lighter(c: ::core::ffi::c_uint) -> ::core::ffi::c_uint;
}
extern "C" {
    pub fn Fl_darker(c: ::core::ffi::c_uint) -> ::core::ffi::c_uint;
}
extern "C" {
    pub fn Fl_set_box_type_cb(
        arg1: ::core::ffi::c_int,
        cb: ::core::option::Option<
            unsafe extern "C" fn(
                arg1: ::core::ffi::c_int,
                arg2: ::core::ffi::c_int,
                arg3: ::core::ffi::c_int,
                arg4: ::core::ffi::c_int,
                arg5: ::core::ffi::c_uint,
            ),
        >,
        arg2: ::core::ffi::c_int,
        arg3: ::core::ffi::c_int,
        arg4: ::core::ffi::c_int,
        arg5: ::core::ffi::c_int,
    );
}
extern "C" {
    pub fn Fl_draw_box_active() -> ::core::ffi::c_int;
}
extern "C" {
    pub fn Fl_gray_ramp(i: ::core::ffi::c_int) -> ::core::ffi::c_uint;
}
extern "C" {
    pub fn Fl_color_average(
        arg1: ::core::ffi::c_uint,
        arg2: ::core::ffi::c_uint,
        f: f32,
    ) -> ::core::ffi::c_uint;
}
extern "C" {
    pub fn Fl_contrast(c1: ::core::ffi::c_uint, c2: ::core::ffi::c_uint) -> ::core::ffi::c_uint;
}
extern "C" {
    pub fn Fl_rgb_color(
        r: ::core::ffi::c_uchar,
        g: ::core::ffi::c_uchar,
        b: ::core::ffi::c_uchar,
    ) -> ::core::ffi::c_uint;
}
extern "C" {
    pub fn Fl_rgb_color2(g: ::core::ffi::c_uchar) -> ::core::ffi::c_uint;
}
extern "C" {
    pub fn Fl_cmap(c: ::core::ffi::c_uint) -> ::core::ffi::c_uint;
}
extern "C" {
    pub fn Fl_box_color(c: ::core::ffi::c_uint) -> ::core::ffi::c_uint;
}
extern "C" {
    pub fn Fl_set_box_color(c: ::core::ffi::c_uint);
}
extern "C" {
    pub fn Fl_add_system_handler(
        arg1: ::core::option::Option<
            unsafe extern "C" fn(
                arg1: *mut ::core::ffi::c_void,
                arg2: *mut ::core::ffi::c_void,
            ) -> ::core::ffi::c_int,
        >,
        arg2: *mut ::core::ffi::c_void,
    );
}
extern "C" {
    pub fn Fl_remove_system_handler(
        arg1: ::core::option::Option<
            unsafe extern "C" fn(
                arg1: *mut ::core::ffi::c_void,
                arg2: *mut ::core::ffi::c_void,
            ) -> ::core::ffi::c_int,
        >,
    );
}
extern "C" {
    pub fn Fl_gl_visual(mode: ::core::ffi::c_int) -> ::core::ffi::c_int;
}
extern "C" {
    pub fn Fl_add_clipboard_notify(
        cb: ::core::option::Option<
            unsafe extern "C" fn(source: ::core::ffi::c_int, data: *mut ::core::ffi::c_void),
        >,
        data: *mut ::core::ffi::c_void,
    );
}
extern "C" {
    pub fn Fl_remove_clipboard_notify(
        cb: ::core::option::Option<
            unsafe extern "C" fn(source: ::core::ffi::c_int, data: *mut ::core::ffi::c_void),
        >,
    );
}
extern "C" {
    pub fn Fl_open_callback(
        cb: ::core::option::Option<unsafe extern "C" fn(arg1: *const ::core::ffi::c_char)>,
    );
}
extern "C" {
    pub fn Fl_disable_wayland();
}
extern "C" {
    pub fn Fl_Widget_Tracker_widget(t: *mut Fl_Widget_Tracker) -> *mut Fl_Widget;
}
extern "C" {
    pub fn Fl_Widget_Tracker_exists(t: *mut Fl_Widget_Tracker) -> ::core::ffi::c_int;
}
extern "C" {
    pub fn Fl_get_color_rgb(
        col: ::core::ffi::c_uint,
        r: *mut ::core::ffi::c_uchar,
        g: *mut ::core::ffi::c_uchar,
        b: *mut ::core::ffi::c_uchar,
    );
}
extern "C" {
    pub fn Fl_callback_reason() -> ::core::ffi::c_int;
}
extern "C" {
    pub fn Fl_get_fl_msg() -> *mut ::core::ffi::c_void;
}
extern "C" {
    pub fn Fl_cairo_make_current(w: *mut Fl_Widget) -> *mut ::core::ffi::c_void;
}
extern "C" {
    pub fn Fl_set_cairo_autolink_context(alink: ::core::ffi::c_int);
}
extern "C" {
    pub fn Fl_cairo_autolink_context() -> ::core::ffi::c_int;
}
extern "C" {
    pub fn Fl_cairo_cc() -> *mut ::core::ffi::c_void;
}
extern "C" {
    pub fn Fl_set_cairo_cc(c: *mut ::core::ffi::c_void, own: ::core::ffi::c_int);
}
extern "C" {
    pub fn Fl_cairo_flush(c: *mut ::core::ffi::c_void);
}
extern "C" {
    pub fn Fl_option(opt: ::core::ffi::c_int) -> ::core::ffi::c_int;
}
extern "C" {
    pub fn Fl_set_option(opt: ::core::ffi::c_int, val: ::core::ffi::c_int);
}
extern "C" {
    pub fn Fl_load_system_icons();
}
extern "C" {
    pub fn Fl_set_contrast_level(level: ::core::ffi::c_int);
}
extern "C" {
    pub fn Fl_contrast_level() -> ::core::ffi::c_int;
}
extern "C" {
    pub fn Fl_set_contrast_mode(mode: ::core::ffi::c_int);
}
extern "C" {
    pub fn Fl_contrast_mode() -> ::core::ffi::c_int;
}
extern "C" {
    pub fn Fl_set_contrast_function(
        f: ::core::option::Option<
            unsafe extern "C" fn(
                arg1: ::core::ffi::c_uint,
                arg2: ::core::ffi::c_uint,
                arg3: ::core::ffi::c_int,
                arg4: ::core::ffi::c_int,
            ) -> ::core::ffi::c_uint,
        >,
    );
}
extern "C" {
    pub fn Fl_using_wayland() -> ::core::ffi::c_int;
}
