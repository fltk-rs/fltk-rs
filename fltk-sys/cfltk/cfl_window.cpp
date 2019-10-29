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

void Fl_Window_set_label(Fl_Window *self, const char *title) {
    self->label(title);
}

void Fl_Window_redraw(Fl_Window *self) {
    self->redraw();
}


void Fl_Window_hide(Fl_Window *self) {
    self->hide();
}

void Fl_Window_activate(Fl_Window *self) {
    self->activate();
}

void Fl_Window_deactivate(Fl_Window *self) {
    self->deactivate();
}

void Fl_Window_redraw_label(Fl_Window *self) {
    self->redraw_label();
}

void Fl_Window_resize(Fl_Window *self, int x, int y, int width, int height) {
    self->resize(x, y, width, height);
}

void Fl_Window_set_tooltip(Fl_Window *self, const char* txt) {
    self->tooltip(txt);
}

void Fl_Window_set_type(Fl_Window *self, int typ) {
    self->type(typ);
}

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