#pragma once

#ifdef __cplusplus
extern "C" {
#endif

typedef struct Fl_Window Fl_Window;

Fl_Window *Fl_Window_new(int x, int y, int width, int height, const char *title);
void Fl_Window_show(Fl_Window *self);
void Fl_Window_hide(Fl_Window *self);
void Fl_Window_set_label(Fl_Window *self, const char *title);
void Fl_Window_redraw(Fl_Window *self);
void Fl_Window_activate(Fl_Window *);
void Fl_Window_deactivate(Fl_Window *); 
void Fl_Window_redraw_label(Fl_Window *);
void Fl_Window_resize(Fl_Window *, int x, int y, int width, int height);
const char* Fl_Window_tooltip(Fl_Window *);
void Fl_Window_set_tooltip(Fl_Window *, const char* txt);
int Fl_Window_get_type(Fl_Window *);
void Fl_Window_set_type(Fl_Window *, int typ);
int Fl_Window_color(Fl_Window *);
void Fl_Window_set_color(Fl_Window *, int color);
int Fl_Window_label_color(Fl_Window *);
void Fl_Window_set_label_color(Fl_Window *, int color);
int Fl_Window_label_font(Fl_Window *);
void Fl_Window_set_label_font(Fl_Window *, int font);
int Fl_Window_label_size(Fl_Window *);
void Fl_Window_set_label_size(Fl_Window *, int sz);
int Fl_Window_label_type(Fl_Window *);
void Fl_Window_set_label_type(Fl_Window *, int typ);
int Fl_Window_box(Fl_Window *);
void Fl_Window_set_box(Fl_Window *, int typ);
void Fl_Window_begin(Fl_Window *self);
void Fl_Window_end(Fl_Window *self);
void Fl_Window_make_modal(Fl_Window *self, unsigned int boolean);
void Fl_Window_fullscreen(Fl_Window *self, unsigned int boolean);
void Fl_Window_make_current(Fl_Window *self);

#ifdef __cplusplus
}
#endif