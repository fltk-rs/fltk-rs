#include <FL/Fl_Button.H>
#include "cfl_button.h"

Fl_Button *Fl_Button_new(int x, int y, int width, int height, const char *title) {
    return new Fl_Button(x, y, width, height, title);
}

void Fl_Button_set_label(Fl_Button *self, const char *title) {
    self->label(title);
}

void Fl_Button_redraw(Fl_Button *self) {
    self->redraw();
}

void Fl_Button_show(Fl_Button *self) {
    self->show();
}

void Fl_Button_hide(Fl_Button *self) {
    self->hide();
}

void Fl_Button_activate(Fl_Button *self) {
    self->activate();
}

void Fl_Button_deactivate(Fl_Button *self) {
    self->deactivate();
}

void Fl_Button_redraw_label(Fl_Button *self) {
    self->redraw_label();
}

void Fl_Button_resize(Fl_Button *self, int x, int y, int width, int height) {
    self->resize(x, y, width, height);
}

void Fl_Button_set_tooltip(Fl_Button *self, const char* txt) {
    self->tooltip(txt);
}

void Fl_Button_set_type(Fl_Button *self, int typ) {
    self->type(typ);
}