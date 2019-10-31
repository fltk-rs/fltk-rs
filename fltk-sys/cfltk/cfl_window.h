#pragma once

#include "global.h"

#ifdef __cplusplus
extern "C" {
#endif

WIDGET_DECLARE(Fl_Window)

GROUP_DECLARE(Fl_Window)

void Fl_Window_make_modal(Fl_Window *self, unsigned int boolean);
void Fl_Window_fullscreen(Fl_Window *self, unsigned int boolean);
void Fl_Window_make_current(Fl_Window *self);

#ifdef __cplusplus
}
#endif