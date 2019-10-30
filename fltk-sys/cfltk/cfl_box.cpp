#include <FL/Fl_Box.H>
#include "cfl_box.h"

Fl_Box *Fl_Box_new(int x, int y, int width, int height, const char *title) {
    return new Fl_Box(x, y, width, height, title);
}

void Fl_Box_set_label(Fl_Box *self, const char *title) {
    self->label(title);
}

void Fl_Box_redraw(Fl_Box *self) {
    self->redraw();
}

void Fl_Box_show(Fl_Box *self) {
    self->show();
}

void Fl_Box_hide(Fl_Box *self) {
    self->hide();
}

void Fl_Box_activate(Fl_Box *self) {
    self->activate();
}

void Fl_Box_deactivate(Fl_Box *self) {
    self->deactivate();
}

void Fl_Box_redraw_label(Fl_Box *self) {
    self->redraw_label();
}

void Fl_Box_resize(Fl_Box *self, int x, int y, int width, int height) {
    self->resize(x, y, width, height);
}

const char* Fl_Box_tooltip(Fl_Box *self) {
    return self->tooltip();
}

void Fl_Box_set_tooltip(Fl_Box *self, const char* txt) {
    self->tooltip(txt);
}

int Fl_Box_get_type(Fl_Box *self) {
    return self->type();
}

void Fl_Box_set_type(Fl_Box *self, int typ) {
    self->type(typ);
}

int Fl_Box_color(Fl_Box *self) {
    return self->color();
}

void Fl_Box_set_color(Fl_Box *self, int color) {
    self->color(color);
}

int Fl_Box_label_color(Fl_Box *self) {
    return self->labelcolor();
}

void Fl_Box_set_label_color(Fl_Box *self, int color) {
    self->labelcolor(color);
}

int Fl_Box_label_font(Fl_Box *self) {
    return self->labelfont();
}

void Fl_Box_set_label_font(Fl_Box *self, int font) {
    self->labelfont(font);
}

int Fl_Box_label_size(Fl_Box *self) {
    return self->labelsize();
}

void Fl_Box_set_label_size(Fl_Box *self, int sz) {
    self->labelsize(sz);
}

int Fl_Box_label_type(Fl_Box *self) {
    return self->labelsize();
}

void Fl_Box_set_label_type(Fl_Box *self, int sz) {
    self->labelsize(sz);
}

int Fl_Box_box(Fl_Box *self) {
    return self->box();
}

void Fl_Box_set_box(Fl_Box *self, int typ) {
    self->box(static_cast<Fl_Boxtype>(typ));
}