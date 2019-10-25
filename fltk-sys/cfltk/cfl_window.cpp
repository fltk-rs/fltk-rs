#include <FL/Fl_Window.H>
#include "cfl_window.h"

Fl_Window *Fl_Window_new(int x, int y, int width, int height, const char *title) {
    return new Fl_Window(x, y, width, height, title);
}

void Fl_Window_begin(Fl_Window *self) {
    self->begin();
}

void Fl_Window_end(Fl_Window *self) {
    self->end();
}

void Fl_Window_show(Fl_Window *self) {
    self->show();
}