#include <FL/Fl_Window.H>
#include "cfl_window.h"

cfl_window *cfl_window_new(int width, int height, const char *title) {
    return (cfl_window *)new Fl_Window(width, height, title);
}

void cfl_window_end(cfl_window *self) {
    static_cast<Fl_Window *>(self)->end();
}

void cfl_window_show(cfl_window *self) {
    static_cast<Fl_Window *>(self)->show();
}