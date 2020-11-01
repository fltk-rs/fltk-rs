#include "cfl_printer.h"
#include "FL/Fl_Widget.H"
#include "FL/Fl_Window.H"
#include <FL/Fl_Printer.H>

Fl_Printer *Fl_Printer_new(void) {
    return new Fl_Printer;
}

void Fl_Printer_delete(Fl_Printer *self) {
    delete self;
}

int Fl_Printer_begin_job(Fl_Printer *self, int pagecount, int *frompage, int *topage,
                         char **perr_message) {
    return self->begin_job(pagecount, frompage, topage, perr_message);
}

int Fl_Printer_begin_page(Fl_Printer *self) {
    return self->begin_page();
}

int Fl_Printer_printable_rect(Fl_Printer *self, int *w, int *h) {
    return self->printable_rect(w, h);
}

void Fl_Printer_margins(Fl_Printer *self, int *left, int *top, int *right, int *bottom) {
    return self->margins(left, top, right, bottom);
}

void Fl_Printer_origin(Fl_Printer *self, int *x, int *y) {
    return self->origin(x, y);
}

void Fl_Printer_set_origin(Fl_Printer *self, int x, int y) {
    return self->origin(x, y);
}

void Fl_Printer_scale(Fl_Printer *self, float scale_x, float scale_y) {
    return self->scale(scale_x, scale_y);
}

void Fl_Printer_rotate(Fl_Printer *self, float angle) {
    return self->rotate(angle);
}

void Fl_Printer_translate(Fl_Printer *self, int x, int y) {
    return self->translate(x, y);
}

void Fl_Printer_untranslate(Fl_Printer *self) {
    return self->untranslate();
}

int Fl_Printer_end_page(Fl_Printer *self) {
    return self->end_page();
}

void Fl_Printer_end_job(Fl_Printer *self) {
    return self->end_job();
}

void Fl_Printer_set_current(Fl_Printer *self) {
    return self->set_current();
}

int Fl_Printer_is_current(Fl_Printer *self) {
    return self->is_current();
}

void Fl_Printer_print_widget(Fl_Printer *self, void *widget, int delta_x, int delta_y) {
    self->print_widget((Fl_Widget *)widget, delta_x, delta_y);
}

void Fl_Printer_print_window(Fl_Printer *self, void *win, int x_offset, int y_offset) {
    self->print_window((Fl_Window *)win, x_offset, y_offset);
}