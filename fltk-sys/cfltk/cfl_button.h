#pragma once

#include "global.h"

#ifdef __cplusplus
extern "C" {
#endif

WIDGET_DECLARE(Fl_Button)   

WIDGET_DECLARE(Fl_Check_Button)

int Fl_Check_Button_is_checked(Fl_Check_Button*);

WIDGET_DECLARE(Fl_Radio_Button)

int Fl_Radio_Button_is_toggled(Fl_Radio_Button*);

WIDGET_DECLARE(Fl_Toggle_Button)

int Fl_Toggle_Button_is_toggled(Fl_Toggle_Button*);

WIDGET_DECLARE(Fl_Round_Button)

WIDGET_DECLARE(Fl_Light_Button)

WIDGET_DECLARE(Fl_Repeat_Button)

WIDGET_DECLARE(Fl_Return_Button)

#ifdef __cplusplus
}
#endif