#include "cfl.h"
#include "cfl_widget.h"
#include <FL/Fl_Widget.H>
#include <FL/Fl.H>

int Fl_run(void) { return Fl::run(); }

int Fl_lock() { return Fl::lock(); }

void Fl_unlock() { Fl::unlock(); }

int Fl_awake(Fl_Awake_Handler handler, void* data) { return Fl::awake(handler, data); }

int Fl_event(void) { return Fl::event(); }

int Fl_event_key(void) { return Fl::event_key(); }

const char *Fl_event_text(void) { return Fl::event_text(); }

int Fl_event_button(void) { return Fl::event_button(); }

int Fl_event_clicks(void) { return Fl::event_clicks(); }

int Fl_event_dx(void) { return Fl::event_dx(); }

int Fl_event_dy(void) { return Fl::event_dy(); }

int Fl_event_is_click(void) { return Fl::event_is_click(); }

int Fl_event_length(void) { return Fl::event_length(); }

int Fl_event_state(void) { return Fl::event_state(); }

int Fl_screen_h(void) { return Fl::h(); }

int Fl_screen_w(void) { return Fl::w(); }

void Fl_paste(void *widget, int src) { 
  Fl::paste(*(Fl_Widget*)widget, src, Fl::clipboard_plain_text); 
}

void Fl_set_scheme(const char* scheme) {
  Fl::scheme(scheme);
}