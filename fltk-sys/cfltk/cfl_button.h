#pragma once

#include "cfl_widget.h"

#ifdef __cplusplus
extern "C" {
#endif

#define BUTTON_DECLARE(widget)                                                 \
  int widget##_shortcut(const widget *self);                                   \
  void widget##_set_shortcut(widget *self, int shortcut);

WIDGET_DECLARE(Fl_Button)

BUTTON_DECLARE(Fl_Button)

WIDGET_DECLARE(Fl_Check_Button)

int Fl_Check_Button_is_checked(Fl_Check_Button *);

BUTTON_DECLARE(Fl_Check_Button)

WIDGET_DECLARE(Fl_Radio_Button)

int Fl_Radio_Button_is_toggled(Fl_Radio_Button *);

BUTTON_DECLARE(Fl_Radio_Button)

WIDGET_DECLARE(Fl_Toggle_Button)

int Fl_Toggle_Button_is_toggled(Fl_Toggle_Button *);

BUTTON_DECLARE(Fl_Toggle_Button)

WIDGET_DECLARE(Fl_Round_Button)

BUTTON_DECLARE(Fl_Round_Button)

WIDGET_DECLARE(Fl_Light_Button)

int Fl_Light_Button_is_on(Fl_Light_Button *);

BUTTON_DECLARE(Fl_Light_Button)

WIDGET_DECLARE(Fl_Repeat_Button)

BUTTON_DECLARE(Fl_Repeat_Button)

WIDGET_DECLARE(Fl_Return_Button)

BUTTON_DECLARE(Fl_Return_Button)

#ifdef __cplusplus
}
#endif