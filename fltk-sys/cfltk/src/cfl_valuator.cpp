#include "cfl_valuator.h"
#include <FL/Fl.H>
#include <FL/Fl_Adjuster.H>
#include <FL/Fl_Counter.H>
#include <FL/Fl_Dial.H>
#include <FL/Fl_Fill_Dial.H>
#include <FL/Fl_Fill_Slider.H>
#include <FL/Fl_Hor_Fill_Slider.H>
#include <FL/Fl_Hor_Nice_Slider.H>
#include <FL/Fl_Hor_Slider.H>
#include <FL/Fl_Hor_Value_Slider.H>
#include <FL/Fl_Image.H>
#include <FL/Fl_Line_Dial.H>
#include <FL/Fl_Nice_Slider.H>
#include <FL/Fl_Roller.H>
#include <FL/Fl_Scrollbar.H>
#include <FL/Fl_Slider.H>
#include <FL/Fl_Value_Input.H>
#include <FL/Fl_Value_Output.H>
#include <FL/Fl_Value_Slider.H>

#define VALUATOR_DEFINE(widget)                                                                    \
    void widget##_set_bounds(widget *self, double a, double b) {                                   \
        LOCK(self->bounds(a, b);)                                                                  \
    }                                                                                              \
    double widget##_minimum(widget *self) {                                                        \
        return self->minimum();                                                                    \
    }                                                                                              \
    void widget##_set_minimum(widget *self, double a) {                                            \
        LOCK(self->minimum(a);)                                                                    \
    }                                                                                              \
    double widget##_maximum(widget *self) {                                                        \
        return self->maximum();                                                                    \
    }                                                                                              \
    void widget##_set_maximum(widget *self, double a) {                                            \
        LOCK(self->maximum(a);)                                                                    \
    }                                                                                              \
    void widget##_set_range(widget *self, double a, double b) {                                    \
        LOCK(self->range(a, b);)                                                                   \
    }                                                                                              \
    void widget##_set_step(widget *self, double a, int b) {                                        \
        LOCK(self->step(a, b);)                                                                    \
    }                                                                                              \
    double widget##_step(widget *self) {                                                           \
        return self->step();                                                                       \
    }                                                                                              \
    void widget##_set_precision(widget *self, int digits) {                                        \
        LOCK(self->precision(digits);)                                                             \
    }                                                                                              \
    double widget##_value(widget *self) {                                                          \
        return self->value();                                                                      \
    }                                                                                              \
    int widget##_set_value(widget *self, double val) {                                             \
        int ret = 0;                                                                               \
        LOCK(ret = self->value(val));                                                              \
        return ret;                                                                                \
    }                                                                                              \
    int widget##_format(widget *self, char *chr) {                                                 \
        int ret = 0;                                                                               \
        LOCK(ret = self->format(chr));                                                             \
        return ret;                                                                                \
    }                                                                                              \
    double widget##_round(widget *self, double val) {                                              \
        int ret = 0;                                                                               \
        LOCK(ret = self->round(val));                                                              \
        return ret;                                                                                \
    }                                                                                              \
    double widget##_clamp(widget *self, double val) {                                              \
        int ret = 0;                                                                               \
        LOCK(ret = self->clamp(val));                                                              \
        return ret;                                                                                \
    }                                                                                              \
    double widget##_increment(widget *self, double a, int b) {                                     \
        int ret = 0;                                                                               \
        LOCK(ret = self->increment(a, b));                                                         \
        return ret;                                                                                \
    }

WIDGET_CLASS(Fl_Slider)

WIDGET_DEFINE(Fl_Slider)

VALUATOR_DEFINE(Fl_Slider)

WIDGET_CLASS(Fl_Nice_Slider)

WIDGET_DEFINE(Fl_Nice_Slider)

VALUATOR_DEFINE(Fl_Nice_Slider)

WIDGET_CLASS(Fl_Counter)

WIDGET_DEFINE(Fl_Counter)

VALUATOR_DEFINE(Fl_Counter)

WIDGET_CLASS(Fl_Dial)

WIDGET_DEFINE(Fl_Dial)

VALUATOR_DEFINE(Fl_Dial)

WIDGET_CLASS(Fl_Line_Dial)

WIDGET_DEFINE(Fl_Line_Dial)

VALUATOR_DEFINE(Fl_Line_Dial)

WIDGET_CLASS(Fl_Roller)

WIDGET_DEFINE(Fl_Roller)

VALUATOR_DEFINE(Fl_Roller)

WIDGET_CLASS(Fl_Scrollbar)

WIDGET_DEFINE(Fl_Scrollbar)

VALUATOR_DEFINE(Fl_Scrollbar)

WIDGET_CLASS(Fl_Value_Slider)

WIDGET_DEFINE(Fl_Value_Slider)

VALUATOR_DEFINE(Fl_Value_Slider)

WIDGET_CLASS(Fl_Adjuster)

WIDGET_DEFINE(Fl_Adjuster)

VALUATOR_DEFINE(Fl_Adjuster)

WIDGET_CLASS(Fl_Value_Input)

WIDGET_DEFINE(Fl_Value_Input)

VALUATOR_DEFINE(Fl_Value_Input)

WIDGET_CLASS(Fl_Value_Output)

WIDGET_DEFINE(Fl_Value_Output)

VALUATOR_DEFINE(Fl_Value_Output)

WIDGET_CLASS(Fl_Fill_Slider)

WIDGET_DEFINE(Fl_Fill_Slider)

VALUATOR_DEFINE(Fl_Fill_Slider)

WIDGET_CLASS(Fl_Fill_Dial)

WIDGET_DEFINE(Fl_Fill_Dial)

VALUATOR_DEFINE(Fl_Fill_Dial)

WIDGET_CLASS(Fl_Hor_Slider)

WIDGET_DEFINE(Fl_Hor_Slider)

VALUATOR_DEFINE(Fl_Hor_Slider)

WIDGET_CLASS(Fl_Hor_Fill_Slider)

WIDGET_DEFINE(Fl_Hor_Fill_Slider)

VALUATOR_DEFINE(Fl_Hor_Fill_Slider)

WIDGET_CLASS(Fl_Hor_Nice_Slider)

WIDGET_DEFINE(Fl_Hor_Nice_Slider)

VALUATOR_DEFINE(Fl_Hor_Nice_Slider)

WIDGET_CLASS(Fl_Hor_Value_Slider)

WIDGET_DEFINE(Fl_Hor_Value_Slider)

VALUATOR_DEFINE(Fl_Hor_Value_Slider)
