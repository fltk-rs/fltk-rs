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

int Fl_Button_handle(Fl_Button *self, int event) {
    return self->handle(event);
}