#include "cfl.h"
#include "cfl_widget.h"
#include <FL/Fl.H>
#include <FL/Fl_Widget.H>
#include <new>
#include <string.h>
#include <random>

int Fl_run(void) { return Fl::run(); }

int Fl_lock() { return Fl::lock(); }

void Fl_unlock() { Fl::unlock(); }

int Fl_awake(Fl_Awake_Handler handler, void *data) {
    return Fl::awake(handler, data);
}

int Fl_event(void) { return Fl::event(); }

int Fl_event_key(void) { return Fl::event_key(); }

const char *Fl_event_text(void) {
    char *buf = (char *)malloc(Fl::event_length() + 1);
    const char *ev_text = Fl::event_text();
    int len = Fl::event_length();
    if (buf && len != 0 && ev_text) {
        strncpy(buf, ev_text, len + 1);
        return buf;
    } else {
        free(buf);
        return NULL;
    }
}

int Fl_event_button(void) { return Fl::event_button(); }

int Fl_event_clicks(void) { return Fl::event_clicks(); }

int Fl_event_x(void) { return Fl::event_x(); }

int Fl_event_y(void) { return Fl::event_y(); }

int Fl_event_is_click(void) { return Fl::event_is_click(); }

int Fl_event_length(void) { return Fl::event_length(); }

int Fl_event_state(void) { return Fl::event_state(); }

int Fl_screen_h(void) { return Fl::h(); }

int Fl_screen_w(void) { return Fl::w(); }

void Fl_paste(void *widget, int src) {
    Fl::paste(*(Fl_Widget *)widget, src, Fl::clipboard_plain_text);
}

void Fl_set_scheme(const char *scheme) { Fl::scheme(scheme); }

unsigned int Fl_get_color(unsigned char r, unsigned char g, unsigned char b) {
    return fl_rgb_color(r, g, b);
}

const char *Fl_get_font(int idx) { return Fl::get_font(idx); }

unsigned char Fl_set_fonts(const char *c) { return Fl::set_fonts(c); }

void Fl_add_handler(int (*ev_handler)(int ev)) { Fl::add_handler(ev_handler); }

void Fl_awake_msg(void *msg) { Fl::awake(msg); }

void *Fl_thread_msg(void) { return Fl::thread_message(); }

int Fl_wait(void) { return Fl::wait(); }

void Fl_add_timeout(double t, void (*timeout_h)(void *), void *data) {
    Fl::add_timeout(t, timeout_h, data);
}

void Fl_repeat_timeout(double t, void (*timeout_h)(void *), void *data) {
    Fl::repeat_timeout(t, timeout_h, data);
}

void Fl_remove_timeout(void (*timeout_h)(void *), void *data) {
    Fl::remove_timeout(timeout_h, data);
}

int Fl_dnd(void) { return Fl::dnd(); }

void *Fl_first_window(void) { return (void *)Fl::first_window(); }

int Fl_should_program_quit(void) {
    return Fl::program_should_quit();
}

void Fl_program_should_quit(int flag) {
    Fl::program_should_quit(flag);
}

unsigned int Fl_rand(void) {
    std::mt19937 rng;
    std::uniform_int_distribution<std::mt19937::result_type> dist(0, UINT_FAST32_MAX);
    rng.seed(std::random_device()());
    return dist(rng);
}