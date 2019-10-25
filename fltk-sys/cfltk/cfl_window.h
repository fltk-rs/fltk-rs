#pragma once

#ifdef __cplusplus
extern "C" {
#endif

typedef struct Fl_Window Fl_Window;

Fl_Window *Fl_Window_new(int x, int y, int width, int height, const char *title);
void Fl_Window_begin(Fl_Window *self);
void Fl_Window_end(Fl_Window *self);
void Fl_Window_show(Fl_Window *self);


#ifdef __cplusplus
}
#endif