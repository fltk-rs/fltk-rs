#include "cfl_misc.h"
#include <FL/Fl_Chart.H>
#include <FL/Fl_Clock.H>
#include <FL/Fl_Image.H>
#include <FL/Fl_Progress.H>
#include <FL/Fl_Spinner.H>
#include <FL/Fl_Timer.H>
#include <new>

WIDGET_DEFINE(Fl_Spinner)

double Fl_Spinner_minimum(Fl_Spinner *self) { return self->minimum(); }

void Fl_Spinner_set_minimum(Fl_Spinner *self, double a) { LOCK(self->minimum(a);) }

double Fl_Spinner_maximum(Fl_Spinner *self) { return self->maximum(); }

void Fl_Spinner_set_maximum(Fl_Spinner *self, double a) { LOCK(self->maximum(a);) }

void Fl_Spinner_set_range(Fl_Spinner *self, double a, double b) {
  LOCK(self->range(a, b);)
}

void Fl_Spinner_set_step(Fl_Spinner *self, double a) { LOCK(self->step(a);) }

double Fl_Spinner_step(Fl_Spinner *self) { return self->step(); }

int Fl_Spinner_maxsize(const Fl_Spinner *self) { return self->maximum_size(); }

void Fl_Spinner_Fl_Spinner_set_maxsize(Fl_Spinner *self, int m) {
  LOCK(self->maximum_size(m);)
}

int Fl_Spinner_text_font(const Fl_Spinner *self) { return self->textfont(); }

void Fl_Spinner_set_text_font(Fl_Spinner *self, int s) { LOCK(self->textfont(s);) }

int Fl_Spinner_text_size(const Fl_Spinner *self) { return self->textsize(); }

void Fl_Spinner_set_textsize(Fl_Spinner *self, int s) { LOCK(self->textsize(s);) }

unsigned int Fl_Spinner_text_color(const Fl_Spinner *self) {
  return self->textcolor();
}

void Fl_Spinner_set_text_color(Fl_Spinner *self, unsigned int n) {
  LOCK(self->textcolor(n);)
}

WIDGET_DEFINE(Fl_Clock)

WIDGET_DEFINE(Fl_Chart)

void Fl_Chart_clear(Fl_Chart *self) { LOCK(self->clear();) }

void Fl_Chart_add(Fl_Chart *self, double val, const char *str, unsigned col) {
  LOCK(self->add(val, str, col);)
}

void Fl_Chart_insert(Fl_Chart *self, int ind, double val, const char *str,
                     unsigned col) {
  LOCK(self->insert(ind, val, str, col);)
}

void Fl_Chart_replace(Fl_Chart *self, int ind, double val, const char *str,
                      unsigned col) {
  LOCK(self->replace(ind, val, str, col);)
}

void Fl_Chart_set_bounds(Fl_Chart *self, double a, double b) {
  LOCK(self->bounds(a, b);)
}

int Fl_Chart_size(const Fl_Chart *self) { return self->size(); }

void Fl_Chart_set_size(Fl_Chart *self, int W, int H) { LOCK(self->size(W, H);) }

int Fl_Chart_maxsize(const Fl_Chart *self) { return self->maxsize(); }

void Fl_Chart_Fl_Chart_set_maxsize(Fl_Chart *self, int m) { LOCK(self->maxsize(m);) }

int Fl_Chart_text_font(const Fl_Chart *self) { return self->textfont(); }

void Fl_Chart_set_text_font(Fl_Chart *self, int s) { LOCK(self->textfont(s);) }

int Fl_Chart_text_size(const Fl_Chart *self) { return self->textsize(); }

void Fl_Chart_set_textsize(Fl_Chart *self, int s) { LOCK(self->textsize(s);) }

unsigned int Fl_Chart_text_color(const Fl_Chart *self) {
  return self->textcolor();
}

void Fl_Chart_set_text_color(Fl_Chart *self, unsigned int n) {
  LOCK(self->textcolor(n);)
}

int Fl_Chart_is_autosize(const Fl_Chart *self) { return self->autosize(); }

void Fl_Chart_make_autosize(Fl_Chart *self, int n) { LOCK(self->autosize(n);) }

WIDGET_DEFINE(Fl_Progress)

double Fl_Progress_minimum(Fl_Progress *self) { return self->minimum(); }

void Fl_Progress_set_minimum(Fl_Progress *self, double a) { LOCK(self->minimum(a);) }

double Fl_Progress_maximum(Fl_Progress *self) { return self->maximum(); }

void Fl_Progress_set_maximum(Fl_Progress *self, double a) { LOCK(self->maximum(a);) }

double Fl_Progress_value(Fl_Progress *self) { return self->value(); }   

void Fl_Progress_set_value(Fl_Progress *self, double val) {                          
  LOCK(self->value(val);)                                                
}

