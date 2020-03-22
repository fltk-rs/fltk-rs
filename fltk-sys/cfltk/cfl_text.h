#pragma once

#include "global.h"

#ifdef __cplusplus
extern "C" {
#endif

WIDGET_DECLARE(Fl_Text_Display)

WIDGET_DECLARE(Fl_Text_Editor)

void Fl_Text_Display_init(Fl_Text_Display *);

const char *Fl_Text_Display_text(Fl_Text_Display *);

void Fl_Text_Display_set_text(Fl_Text_Display *, const char *);

void Fl_Text_Editor_init(Fl_Text_Editor *);

const char *Fl_Text_Editor_text(Fl_Text_Editor *);

void Fl_Text_Editor_set_text(Fl_Text_Editor *, const char *);

int kf_copy(Fl_Text_Editor *e);

int kf_cut(Fl_Text_Editor *e);

int kf_paste(Fl_Text_Editor *e);

int kf_undo(Fl_Text_Editor *e);

int text_font(const Fl_Text_Display*);

void set_text_font(Fl_Text_Display*, int s);

int text_size(const Fl_Text_Display*);

void set_text_size(Fl_Text_Display* , int s);

int text_color(const Fl_Text_Display*);

void set_text_color(Fl_Text_Display*, int n);

#ifdef __cplusplus
}
#endif