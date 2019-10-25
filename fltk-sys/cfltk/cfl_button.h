#pragma once

#include "cfl_widget.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef struct Fl_Button Fl_Button;

Fl_Button *Fl_Button_new(int x, int y, int width, int height, const char *title);
void Fl_Button_callback(Fl_Button *, Fl_Callback* cb, void* data);
void Fl_Button_set_label(Fl_Button *, const char *title);
void Fl_Button_redraw(Fl_Button *);

#ifdef __cplusplus
}
#endif