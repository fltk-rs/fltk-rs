#pragma once

#include "cfl_widget.h"

#ifdef __cplusplus
extern "C" {
#endif

#define VALUATOR_DECLARE(widget)                                               \
    void widget##_set_bounds(widget *, double a, double b);                    \
    double widget##_minimum(widget *);                                         \
    void widget##_set_minimum(widget *, double a);                             \
    double widget##_maximum(widget *);                                         \
    void widget##_set_maximum(widget *, double a);                             \
    void widget##_set_range(widget *, double a, double b);                     \
    void widget##_set_step(widget *, double a, int b);                         \
    double widget##_step(widget *);                                            \
    void widget##_set_precision(widget *, int digits);                         \
    double widget##_value(widget *);                                           \
    int widget##_set_value(widget *, double);                                  \
    int widget##_format(widget *, char *);                                     \
    double widget##_round(widget *, double);                                   \
    double widget##_clamp(widget *, double);                                   \
    double widget##_increment(widget *, double, int);

WIDGET_DECLARE(Fl_Slider)

VALUATOR_DECLARE(Fl_Slider)

WIDGET_DECLARE(Fl_Nice_Slider)

VALUATOR_DECLARE(Fl_Nice_Slider)

WIDGET_DECLARE(Fl_Counter)

VALUATOR_DECLARE(Fl_Counter)

WIDGET_DECLARE(Fl_Dial)

VALUATOR_DECLARE(Fl_Dial)

WIDGET_DECLARE(Fl_Line_Dial)

VALUATOR_DECLARE(Fl_Line_Dial)

WIDGET_DECLARE(Fl_Roller)

VALUATOR_DECLARE(Fl_Roller)

WIDGET_DECLARE(Fl_Scrollbar)

VALUATOR_DECLARE(Fl_Scrollbar)

WIDGET_DECLARE(Fl_Value_Slider)

VALUATOR_DECLARE(Fl_Value_Slider)

WIDGET_DECLARE(Fl_Adjuster)

VALUATOR_DECLARE(Fl_Adjuster)

WIDGET_DECLARE(Fl_Value_Input)

VALUATOR_DECLARE(Fl_Value_Input)

WIDGET_DECLARE(Fl_Value_Output)

VALUATOR_DECLARE(Fl_Value_Output)

WIDGET_DECLARE(Fl_Fill_Slider)

VALUATOR_DECLARE(Fl_Fill_Slider)

WIDGET_DECLARE(Fl_Fill_Dial)

VALUATOR_DECLARE(Fl_Fill_Dial)

WIDGET_DECLARE(Fl_Hor_Slider)

VALUATOR_DECLARE(Fl_Hor_Slider)

WIDGET_DECLARE(Fl_Hor_Fill_Slider)

VALUATOR_DECLARE(Fl_Hor_Fill_Slider)

WIDGET_DECLARE(Fl_Hor_Nice_Slider)

VALUATOR_DECLARE(Fl_Hor_Nice_Slider)

WIDGET_DECLARE(Fl_Hor_Value_Slider)

VALUATOR_DECLARE(Fl_Hor_Value_Slider)

#ifdef __cplusplus
}
#endif