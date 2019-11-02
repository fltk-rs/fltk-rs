#pragma once

#include "global.h"
#include "cfl_widget.h"

#ifdef __cplusplus
extern "C" {
#endif

WIDGET_DECLARE(Fl_Menu_Bar)

WIDGET_DECLARE(Fl_Menu_Button)

WIDGET_DECLARE(Fl_Choice)

void Fl_Menu_Bar_add(Fl_Menu_Bar*, const char* name, int shortcut, Fl_Callback*, void *, int);

#ifdef __cplusplus
}
#endif