#pragma once

#include "global.h"

#ifdef __cplusplus
extern "C" {
#endif

WIDGET_DECLARE(Fl_Text_Display)

void Fl_Text_Display_init(Fl_Text_Display *);

DISPLAY_DECLARE(Fl_Text_Display)

WIDGET_DECLARE(Fl_Text_Editor)

void Fl_Text_Editor_init(Fl_Text_Editor *);

DISPLAY_DECLARE(Fl_Text_Editor)

int kf_copy(Fl_Text_Editor *e);

int kf_cut(Fl_Text_Editor *e);

int kf_paste(Fl_Text_Editor *e);

int kf_undo(Fl_Text_Editor *e);

WIDGET_DECLARE(Fl_Simple_Terminal)

void Fl_Simple_Terminal_init(Fl_Simple_Terminal *);

DISPLAY_DECLARE(Fl_Simple_Terminal)

#ifdef __cplusplus
}
#endif