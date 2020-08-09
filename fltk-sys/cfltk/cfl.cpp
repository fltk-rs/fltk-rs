#include <FL/Fl.H> // Has to be the first include!

#include "cfl.h"
#include "cfl_widget.h"
#include <FL/Enumerations.H>
#include <FL/Fl_Widget.H>
#include <new>
#include <stdarg.h>
#include <stdint.h>
#include <string.h>

int Fl_run(void) {
    return Fl::run();
}

int Fl_lock() {
    return Fl::lock();
}

void Fl_unlock() {
    Fl::unlock();
}

int Fl_awake(Fl_Awake_Handler handler, void *data) {
    return Fl::awake(handler, data);
}

int Fl_event(void) {
    return Fl::event();
}

int Fl_event_key(void) {
    return Fl::event_key();
}

const char *Fl_event_text(void) {
    return Fl::event_text();
}

int Fl_event_button(void) {
    return Fl::event_button();
}

int Fl_event_clicks(void) {
    return Fl::event_clicks();
}

int Fl_event_x(void) {
    return Fl::event_x();
}

int Fl_event_y(void) {
    return Fl::event_y();
}

int Fl_event_x_root(void){
    return Fl::event_x_root();
}

int Fl_event_y_root(void){
    return Fl::event_y_root();
}
int Fl_event_dx(void){
    return Fl::event_dx();
}

int Fl_event_dy(void){
    return Fl::event_dy();
}

void Fl_get_mouse(int *x, int *y) {
    Fl::get_mouse(*x, *y);
}

int Fl_event_is_click(void) {
    return Fl::event_is_click();
}

int Fl_event_length(void) {
    return Fl::event_length();
}

int Fl_event_state(void) {
    return Fl::event_state();
}

int Fl_screen_h(void) {
    return Fl::h();
}

int Fl_screen_w(void) {
    return Fl::w();
}

void Fl_paste(Fl_Widget *widget, int src) {
    Fl::paste(*widget, src, Fl::clipboard_plain_text);
}

void Fl_set_scheme(const char *scheme) {
    Fl::scheme(scheme);
}

int Fl_scheme(void) {
    const char *v = Fl::scheme();
    if (!strcmp(v, "base")) {
        return 0;
    } else if (!strcmp(v, "gtk+")) {
        return 1;
    } else if (!strcmp(v, "gleam")) {
        return 2;
    } else {
        return 3;
    }
}

unsigned int Fl_get_rgb_color(unsigned char r, unsigned char g, unsigned char b) {
    return fl_rgb_color(r, g, b);
}

const char *Fl_get_font(int idx) {
    return Fl::get_font(idx);
}

unsigned char Fl_set_fonts(const char *c) {
    return Fl::set_fonts(c);
}

void Fl_add_handler(int (*ev_handler)(int ev)) {
    Fl::add_handler(ev_handler);
}

void Fl_awake_msg(void *msg) {
    Fl::awake(msg);
}

void *Fl_thread_msg(void) {
    return Fl::thread_message();
}

int Fl_wait(void) {
    return Fl::wait();
}

double Fl_wait_for(double dur) {
    return Fl::wait(dur);
}

void Fl_add_timeout(double t, void (*timeout_h)(void *), void *data) {
    Fl::add_timeout(t, timeout_h, data);
}

void Fl_repeat_timeout(double t, void (*timeout_h)(void *), void *data) {
    Fl::repeat_timeout(t, timeout_h, data);
}

void Fl_remove_timeout(void (*timeout_h)(void *), void *data) {
    Fl::remove_timeout(timeout_h, data);
}

int Fl_dnd(void) {
    return Fl::dnd();
}

void *Fl_first_window(void) {
    return (void *)Fl::first_window();
}

void *Fl_next_window(const void *prev) {
    return (void *)Fl::next_window((Fl_Window *)prev);
}

int Fl_should_program_quit(void) {
    return Fl::program_should_quit();
}

void Fl_program_should_quit(int flag) {
    Fl::program_should_quit(flag);
}

// unsigned int Fl_rand(void) {
//     std::mt19937 rng;
//     std::uniform_int_distribution<std::mt19937::result_type> dist(0, UINT_FAST32_MAX);
//     rng.seed(std::random_device()());
//     return dist(rng);
// }

int Fl_event_inside(int x, int y, int w, int h) {
    return Fl::event_inside(x, y, w, h);
}

Fl_Widget *Fl_belowmouse() { return Fl::belowmouse(); }

void Fl_delete_widget(Fl_Widget *w) {
    Fl::delete_widget(w);
}

Fl_Widget_Tracker *Fl_Widget_Tracker_new(Fl_Widget *w) {
    return new (std::nothrow) Fl_Widget_Tracker(w);
}

int Fl_Widget_Tracker_deleted(Fl_Widget_Tracker *self) {
    return self->deleted();
}

void Fl_Widget_Tracker_delete(Fl_Widget_Tracker *self) {
    delete self;
}

void Fl_init_all(void) {
    fl_define_FL_ROUND_UP_BOX();
    fl_define_FL_SHADOW_BOX();
    fl_define_FL_ROUNDED_BOX();
    fl_define_FL_RFLAT_BOX();
    fl_define_FL_RSHADOW_BOX();
    fl_define_FL_DIAMOND_BOX();
    fl_define_FL_OVAL_BOX();
    fl_define_FL_PLASTIC_UP_BOX();
    fl_define_FL_GTK_UP_BOX();
    fl_define_FL_GLEAM_UP_BOX();
    fl_define_FL_SHADOW_LABEL();
    fl_define_FL_ENGRAVED_LABEL();
    fl_define_FL_EMBOSSED_LABEL();
    fl_define_FL_MULTI_LABEL();
    fl_define_FL_ICON_LABEL();
    fl_define_FL_IMAGE_LABEL();

#if defined(__APPLE__) && defined(CFLTK_USE_GL)
    Fl::use_high_res_GL(1);
#endif

}

void Fl_redraw(void) {
    Fl::redraw();
}

int Fl_event_shift(void) {
    return Fl::event_shift();
}

int Fl_event_ctrl(void) {
    return Fl::event_ctrl();
}

int Fl_event_command(void) {
    return Fl::event_command();
}

int Fl_event_alt(void) {
    return Fl::event_alt();
}

void Fl_set_damage(int flag) {
    Fl::damage(flag);
}

int Fl_damage(void) {
    return Fl::damage();
}

int Fl_visual(int mode) {
    return Fl::visual(mode);
}

void Fl_own_colormap(void) {
    Fl::own_colormap();
}

Fl_Widget *Fl_pushed(void) {
    return Fl::pushed();
}

Fl_Widget *Fl_focus(void) {
    return Fl::focus();
}

void Fl_set_focus(void *wid) {
    Fl::focus((Fl_Widget *)wid);
}

double Fl_version(void) {
    return Fl::version();
}

int Fl_api_version(void) {
    return Fl::api_version();
}

int Fl_abi_version(void) {
    return Fl::abi_version();
}

// void Fl_set_error(void (*error)(const char *, ...)) {
//     Fl::error = error;
// }

// void Fl_set_warning(void (*error)(const char *, ...)) {
//     Fl::warning = error;
// }

// void Fl_set_fatal(void (*error)(const char *, ...)) {
//     Fl::fatal = error;
// }