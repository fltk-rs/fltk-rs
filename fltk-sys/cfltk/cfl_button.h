#pragma once

#include "cfl_widget.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef struct Fl_Button Fl_Button;
typedef void (*Fl_button_cb)(struct Fl_Widget *widget);

Fl_Button *Fl_Button_new(int x, int y, int width, int height, const char *title);


#ifdef __cplusplus
}
#endif