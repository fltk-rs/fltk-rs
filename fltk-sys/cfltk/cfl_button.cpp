#include "cfl_button.h"
#include <FL/Fl.H>
#include <FL/Fl_Button.H>
#include <FL/Fl_Check_Button.H>
#include <FL/Fl_Image.H>
#include <FL/Fl_Light_Button.H>
#include <FL/Fl_Radio_Button.H>
#include <FL/Fl_Radio_Light_Button.H>
#include <FL/Fl_Radio_Round_Button.H>
#include <FL/Fl_Repeat_Button.H>
#include <FL/Fl_Return_Button.H>
#include <FL/Fl_Round_Button.H>
#include <FL/Fl_Toggle_Button.H>
#include <FL/Fl_Toggle_Light_Button.H>
#include <FL/Fl_Toggle_Round_Button.H>
#include <new>

#define BUTTON_DEFINE(widget)                                                  \
    int widget##_shortcut(const widget *self) { return self->shortcut(); }     \
    void widget##_set_shortcut(widget *self, int shortcut) {                   \
        self->shortcut(shortcut);                                              \
    }                                                                          \
    int widget##_clear(widget *self) {                                         \
        int ret = 0;                                                           \
        LOCK(ret = self->clear());                                             \
        return ret;                                                            \
    }                                                                          \
    int widget##_value(widget *self) { return self->value(); }                 \
    void widget##_set_value(widget *self, int flag) { LOCK(self->value(flag);) }

WIDGET_DEFINE(Fl_Button)

BUTTON_DEFINE(Fl_Button)

WIDGET_DEFINE(Fl_Check_Button)

int Fl_Check_Button_is_checked(Fl_Check_Button *self) { return self->value(); }

void Fl_Check_Button_set_checked(Fl_Check_Button *self, int checked) {
    LOCK(self->value(checked);)
}

BUTTON_DEFINE(Fl_Check_Button)

WIDGET_DEFINE(Fl_Radio_Button)

int Fl_Radio_Button_is_toggled(Fl_Radio_Button *self) { return self->value(); }

void Fl_Radio_Button_toggle(Fl_Radio_Button *self, int boolean) {
    LOCK(self->value(boolean);)
}

BUTTON_DEFINE(Fl_Radio_Button)

WIDGET_DEFINE(Fl_Toggle_Button)

int Fl_Toggle_Button_is_toggled(Fl_Toggle_Button *self) {
    return self->value();
}

void Fl_Toggle_Button_toggle(Fl_Toggle_Button *self, int boolean) {
    LOCK(self->value(boolean);)
}

BUTTON_DEFINE(Fl_Toggle_Button)

// WIDGET_DEFINE(Fl_Toggle_Round_Button)

// int Fl_Toggle_Round_Button_is_toggled(Fl_Toggle_Round_Button *self) {
//     return self->value();
// }

// void Fl_Toggle_Round_Button_toggle(Fl_Toggle_Round_Button *self, int boolean)
// {
//     LOCK(self->value(boolean);)
// }

// BUTTON_DEFINE(Fl_Toggle_Round_Button)

// WIDGET_DEFINE(Fl_Toggle_Light_Button)

// int Fl_Toggle_Light_Button_is_toggled(Fl_Toggle_Light_Button *self) {
//     return self->value();
// }

// void Fl_Toggle_Light_Button_toggle(Fl_Toggle_Light_Button *self, int boolean)
// {
//     LOCK(self->value(boolean);)
// }

// BUTTON_DEFINE(Fl_Toggle_Light_Button)

WIDGET_DEFINE(Fl_Round_Button)

int Fl_Round_Button_is_toggled(Fl_Round_Button *self) { return self->value(); }

void Fl_Round_Button_toggle(Fl_Round_Button *self, int boolean) {
    LOCK(self->value(boolean);)
}

BUTTON_DEFINE(Fl_Round_Button)

WIDGET_DEFINE(Fl_Radio_Round_Button)

int Fl_Radio_Round_Button_is_toggled(Fl_Radio_Round_Button *self) {
    return self->value();
}

void Fl_Radio_Round_Button_toggle(Fl_Radio_Round_Button *self, int boolean) {
    LOCK(self->value(boolean);)
}

BUTTON_DEFINE(Fl_Radio_Round_Button)

WIDGET_DEFINE(Fl_Radio_Light_Button)

int Fl_Radio_Light_Button_is_toggled(Fl_Radio_Light_Button *self) {
    return self->value();
}

void Fl_Radio_Light_Button_toggle(Fl_Radio_Light_Button *self, int boolean) {
    LOCK(self->value(boolean);)
}

BUTTON_DEFINE(Fl_Radio_Light_Button)

WIDGET_DEFINE(Fl_Light_Button)

int Fl_Light_Button_is_on(Fl_Light_Button *self) { return self->value(); }

void Fl_Light_Button_turn_on(Fl_Light_Button *self, int boolean) {
    LOCK(self->value(boolean);)
}

BUTTON_DEFINE(Fl_Light_Button)

WIDGET_DEFINE(Fl_Repeat_Button)

BUTTON_DEFINE(Fl_Repeat_Button)

WIDGET_DEFINE(Fl_Return_Button)

BUTTON_DEFINE(Fl_Return_Button)
