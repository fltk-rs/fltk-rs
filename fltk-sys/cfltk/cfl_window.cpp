#include <FL/Fl_Window.H>
#include "cfl_window.h"

WIDGET_DEFINE(Fl_Window)

GROUP_DEFINE(Fl_Window)

void Fl_Window_make_modal(Fl_Window *self, unsigned int boolean) {
    if (boolean) {
        self->set_modal();
    } else {
        self->set_non_modal();
    }
}

void Fl_Window_fullscreen(Fl_Window *self, unsigned int boolean) {
    if (boolean) {
        self->fullscreen();
    } else {
        self->fullscreen_off();
    }
}

void Fl_Window_make_current(Fl_Window *self) {
    self->make_current();
}