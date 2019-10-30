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

const char* Fl_Button_tooltip(Fl_Button *self) {
    return self->tooltip();
}

void Fl_Button_set_tooltip(Fl_Button *self, const char* txt) {
    self->tooltip(txt);
}

int Fl_Button_get_type(Fl_Button *self) {
    return self->type();
}

void Fl_Button_set_type(Fl_Button *self, int typ) {
    self->type(typ);
}

int Fl_Button_color(Fl_Button *self) {
    return self->color();
}

void Fl_Button_set_color(Fl_Button *self, int color) {
    self->color(color);
}

int Fl_Button_label_color(Fl_Button *self) {
    return self->labelcolor();
}

void Fl_Button_set_label_color(Fl_Button *self, int color) {
    self->labelcolor(color);
}

int Fl_Button_label_font(Fl_Button *self) {
    return self->labelfont();
}

void Fl_Button_set_label_font(Fl_Button *self, int font) {
    self->labelfont(font);
}

int Fl_Button_label_size(Fl_Button *self) {
    return self->labelsize();
}

void Fl_Button_set_label_size(Fl_Button *self, int sz) {
    self->labelsize(sz);
}

int Fl_Button_label_type(Fl_Button *self) {
    return self->labelsize();
}

void Fl_Button_set_label_type(Fl_Button *self, int sz) {
    self->labelsize(sz);
}