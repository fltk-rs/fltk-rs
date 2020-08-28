#pragma once

#include "cfl_widget.h"

#ifdef __cplusplus
extern "C" {
#endif

WIDGET_DECLARE(Fl_Spinner)

double Fl_Spinner_minimum(Fl_Spinner *);

void Fl_Spinner_set_minimum(Fl_Spinner *, double a);

double Fl_Spinner_maximum(Fl_Spinner *);

void Fl_Spinner_set_maximum(Fl_Spinner *, double a);

void Fl_Spinner_set_range(Fl_Spinner *, double a, double b);

void Fl_Spinner_set_step(Fl_Spinner *, double a);

double Fl_Spinner_step(Fl_Spinner *);

int Fl_Spinner_maxsize(const Fl_Spinner *self);

void Fl_Spinner_set_maxsize(Fl_Spinner *self, int m);

int Fl_Spinner_text_font(const Fl_Spinner *self);

void Fl_Spinner_set_text_font(Fl_Spinner *self, int s);

int Fl_Spinner_text_size(const Fl_Spinner *self);

void Fl_Spinner_set_textsize(Fl_Spinner *self, int s);

unsigned int Fl_Spinner_text_color(const Fl_Spinner *self);

void Fl_Spinner_set_text_color(Fl_Spinner *self, unsigned int n);

WIDGET_DECLARE(Fl_Clock)

WIDGET_DECLARE(Fl_Chart)

void Fl_Chart_clear(Fl_Chart *self);

void Fl_Chart_add(Fl_Chart *self, double val, const char *str, unsigned col);

void Fl_Chart_insert(Fl_Chart *self, int ind, double val, const char *str, unsigned col);

void Fl_Chart_replace(Fl_Chart *self, int ind, double val, const char *str, unsigned col);

void Fl_Chart_set_bounds(Fl_Chart *self, double a, double b);

int Fl_Chart_size(const Fl_Chart *self);

void Fl_Chart_set_size(Fl_Chart *self, int W, int H);

int Fl_Chart_maxsize(const Fl_Chart *self);

void Fl_Chart_set_maxsize(Fl_Chart *self, int m);

int Fl_Chart_text_font(const Fl_Chart *self);

void Fl_Chart_set_text_font(Fl_Chart *self, int s);

int Fl_Chart_text_size(const Fl_Chart *self);

void Fl_Chart_set_textsize(Fl_Chart *self, int s);

unsigned int Fl_Chart_text_color(const Fl_Chart *self);

void Fl_Chart_set_text_color(Fl_Chart *self, unsigned int n);

int Fl_Chart_is_autosize(const Fl_Chart *self);

void Fl_Chart_make_autosize(Fl_Chart *self, int n);

WIDGET_DECLARE(Fl_Progress)

double Fl_Progress_minimum(Fl_Progress *);

void Fl_Progress_set_minimum(Fl_Progress *, double a);

double Fl_Progress_maximum(Fl_Progress *);

void Fl_Progress_set_maximum(Fl_Progress *, double a);

double Fl_Progress_value(Fl_Progress *);

void Fl_Progress_set_value(Fl_Progress *, double);

float Fl_Tooltip_delay(void);

void Fl_Tooltip_set_delay(float f);

float Fl_Tooltip_hidedelay(void);

void Fl_Tooltip_set_hidedelay(float f);

float Fl_Tooltip_hoverdelay(void);

void Fl_Tooltip_set_hoverdelay(float f);

int Fl_Tooltip_enabled(void);

void Fl_Tooltip_enable(int b);

void Fl_Tooltip_disable(void);

void (*Fl_Tooltip_enter)(Fl_Widget *w);

void Fl_Tooltip_enter_area(Fl_Widget *w, int X, int Y, int W, int H, const char *tip);

void (*Fl_Tooltip_exit)(Fl_Widget *w);

void *Fl_Tooltip_current_widget(void);

void Fl_Tooltip_current(Fl_Widget *);

int Fl_Tooltip_font(void);

void Fl_Tooltip_set_font(int i);

int Fl_Tooltip_font_size(void);

void Fl_Tooltip_set_font_size(int s);

int Fl_Tooltip_color(void);

void Fl_Tooltip_set_color(unsigned int c);

int Fl_Tooltip_text_color(void);

void Fl_Tooltip_set_text_color(unsigned int c);

int Fl_Tooltip_margin_width(void);

void Fl_Tooltip_set_margin_width(int v);

int Fl_Tooltip_margin_height(void);

void Fl_Tooltip_set_margin_height(int v);

int Fl_Tooltip_wrap_width(void);

void Fl_Tooltip_set_wrap_width(int v);

void *Fl_Tooltip_current_window(void);

#ifdef __cplusplus
}
#endif