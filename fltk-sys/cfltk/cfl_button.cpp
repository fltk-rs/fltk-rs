#include "cfl_button.h"
#include <FL/Fl_Button.H>
#include <FL/Fl_Check_Button.H>
#include <FL/Fl_Image.H>
#include <FL/Fl_Light_Button.H>
#include <FL/Fl_Radio_Button.H>
#include <FL/Fl_Repeat_Button.H>
#include <FL/Fl_Return_Button.H>
#include <FL/Fl_Round_Button.H>
#include <FL/Fl_Toggle_Button.H>
#include <cstring>
#include <string>

WIDGET_DEFINE(Fl_Button)

WIDGET_DEFINE(Fl_Check_Button)

int Fl_Check_Button_is_checked(Fl_Check_Button* self) {
    return self->value(); 
}

WIDGET_DEFINE(Fl_Radio_Button)

int Fl_Radio_Button_is_toggled(Fl_Radio_Button* self) {
    return self->value(); 
}

WIDGET_DEFINE(Fl_Toggle_Button)

int Fl_Toggle_Button_is_toggled(Fl_Toggle_Button* self) {
    return self->value();
}

WIDGET_DEFINE(Fl_Round_Button)

WIDGET_DEFINE(Fl_Light_Button)

WIDGET_DEFINE(Fl_Repeat_Button)

WIDGET_DEFINE(Fl_Return_Button)
