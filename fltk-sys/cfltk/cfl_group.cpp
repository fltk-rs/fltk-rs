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

const char* Fl_Group_tooltip(Fl_Group *self) {
    return self->tooltip();
}

void Fl_Group_set_tooltip(Fl_Group *self, const char* txt) {
    self->tooltip(txt);
}

int Fl_Group_get_type(Fl_Group *self) {
    return self->type();
}

void Fl_Group_set_type(Fl_Group *self, int typ) {
    self->type(typ);
}

int Fl_Group_color(Fl_Group *self) {
    return self->color();
}

void Fl_Group_set_color(Fl_Group *self, int color) {
    self->color(color);
}

int Fl_Group_label_color(Fl_Group *self) {
    return self->labelcolor();
}

void Fl_Group_set_label_color(Fl_Group *self, int color) {
    self->labelcolor(color);
}

int Fl_Group_label_font(Fl_Group *self) {
    return self->labelfont();
}

void Fl_Group_set_label_font(Fl_Group *self, int font) {
    self->labelfont(font);
}

int Fl_Group_label_size(Fl_Group *self) {
    return self->labelsize();
}

void Fl_Group_set_label_size(Fl_Group *self, int sz) {
    self->labelsize(sz);
}

int Fl_Group_label_type(Fl_Group *self) {
    return self->labelsize();
}

void Fl_Group_set_label_type(Fl_Group *self, int sz) {
    self->labelsize(sz);
}