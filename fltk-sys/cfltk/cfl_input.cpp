#include <FL/Fl_Input.H>
#include "cFl_Input.h"

Fl_Input *Fl_Input_new(int x, int y, int width, int height, const char *title) {
    return new Fl_Input(x, y, width, height, title);
}

void Fl_Input_set_label(Fl_Input *self, const char *title) {
    self->label(title);
}

void Fl_Input_redraw(Fl_Input *self) {
    self->redraw();
}

void Fl_Input_show(Fl_Input *self) {
    self->show();
}

void Fl_Input_hide(Fl_Input *self) {
    self->hide();
}

void Fl_Input_activate(Fl_Input *self) {
    self->activate();
}

void Fl_Input_deactivate(Fl_Input *self) {
    self->deactivate();
}

void Fl_Input_redraw_label(Fl_Input *self) {
    self->redraw_label();
}

void Fl_Input_resize(Fl_Input *self, int x, int y, int width, int height) {
    self->resize(x, y, width, height);
}

const char* Fl_Input_tooltip(Fl_Input *self) {
    return self->tooltip();
}

void Fl_Input_set_tooltip(Fl_Input *self, const char* txt) {
    self->tooltip(txt);
}

int Fl_Input_get_type(Fl_Input *self) {
    return self->type();
}

void Fl_Input_set_type(Fl_Input *self, int typ) {
    self->type(typ);
}

int Fl_Input_color(Fl_Input *self) {
    return self->color();
}

void Fl_Input_set_color(Fl_Input *self, int color) {
    self->color(color);
}

int Fl_Input_label_color(Fl_Input *self) {
    return self->labelcolor();
}

void Fl_Input_set_label_color(Fl_Input *self, int color) {
    self->labelcolor(color);
}

int Fl_Input_label_font(Fl_Input *self) {
    return self->labelfont();
}

void Fl_Input_set_label_font(Fl_Input *self, int font) {
    self->labelfont(font);
}

int Fl_Input_label_size(Fl_Input *self) {
    return self->labelsize();
}

void Fl_Input_set_label_size(Fl_Input *self, int sz) {
    self->labelsize(sz);
}

int Fl_Input_label_type(Fl_Input *self) {
    return self->labelsize();
}

void Fl_Input_set_label_type(Fl_Input *self, int sz) {
    self->labelsize(sz);
}

int Fl_Input_box(Fl_Input *self) {
    return self->box();
}

void Fl_Input_set_box(Fl_Input *self, int typ) {
    self->box(static_cast<Fl_Boxtype>(typ));
}