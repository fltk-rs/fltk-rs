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
void Fl_Window_set_tooltip(Fl_Window *, const char* txt);
void Fl_Window_begin(Fl_Window *self);
void Fl_Window_end(Fl_Window *self);
void Fl_Window_make_modal(Fl_Window *self, unsigned int boolean);
void Fl_Window_fullscreen(Fl_Window *self, unsigned int boolean);
void Fl_Window_make_current(Fl_Window *self);

#ifdef __cplusplus
}
#endif