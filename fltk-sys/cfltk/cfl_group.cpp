#include <FL/Fl_Group.H>
#include "cfl_group.h"

Fl_Group *Fl_Group_new(int x, int y, int width, int height, const char *title) {
    return new Fl_Group(x, y, width, height, title);
}

void Fl_Group_begin(Fl_Group *self) {
    self->begin();
}

void Fl_Group_end(Fl_Group *self) {
    self->end();
}

void Fl_Group_show(Fl_Group *self) {
    self->show();
}

void Fl_Group_set_label(Fl_Group *self, const char *title) {
    self->label(title);
}

void Fl_Group_redraw(Fl_Group *self) {
    self->redraw();
}


void Fl_Group_hide(Fl_Group *self) {
    self->hide();
}

void Fl_Group_activate(Fl_Group *self) {
    self->activate();
}

void Fl_Group_deactivate(Fl_Group *self) {
    self->deactivate();
}

void Fl_Group_redraw_label(Fl_Group *self) {
    self->redraw_label();
}

void Fl_Group_resize(Fl_Group *self, int x, int y, int width, int height) {
    self->resize(x, y, width, height);
}

void Fl_Group_set_tooltip(Fl_Group *self, const char* txt) {
    self->tooltip(txt);
}