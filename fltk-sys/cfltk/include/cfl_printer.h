#pragma once

#ifdef __cplusplus
extern "C" {
#endif

typedef struct Fl_Printer Fl_Printer;

Fl_Printer *Fl_Printer_new(void);

void Fl_Printer_delete(Fl_Printer *self);

int Fl_Printer_begin_job(Fl_Printer *self, int pagecount, int *frompage, int *topage,
                         char **perr_message);

int Fl_Printer_begin_page(Fl_Printer *self);

int Fl_Printer_printable_rect(Fl_Printer *self, int *w, int *h);

void Fl_Printer_margins(Fl_Printer *self, int *left, int *top, int *right, int *bottom);

void Fl_Printer_origin(Fl_Printer *self, int *x, int *y);

void Fl_Printer_set_origin(Fl_Printer *self, int x, int y);

void Fl_Printer_scale(Fl_Printer *self, float scale_x, float scale_y);

void Fl_Printer_rotate(Fl_Printer *self, float angle);

void Fl_Printer_translate(Fl_Printer *self, int x, int y);

void Fl_Printer_untranslate(Fl_Printer *self);

int Fl_Printer_end_page(Fl_Printer *self);

void Fl_Printer_end_job(Fl_Printer *self);

void Fl_Printer_set_current(Fl_Printer *self);

int Fl_Printer_is_current(Fl_Printer *self);

void Fl_Printer_print_widget(Fl_Printer *self, void* widget, int delta_x, int delta_y);

void Fl_Printer_print_window(Fl_Printer *self, void *win, int x_offset, int y_offset);

#ifdef __cplusplus
}
#endif
