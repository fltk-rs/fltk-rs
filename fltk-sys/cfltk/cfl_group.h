#pragma once

#include "global.h"

#ifdef __cplusplus
extern "C" {
#endif

WIDGET_DECLARE(Fl_Group)

GROUP_DECLARE(Fl_Group)

WIDGET_DECLARE(Fl_Pack)

GROUP_DECLARE(Fl_Pack)

WIDGET_DECLARE(Fl_Scroll)

GROUP_DECLARE(Fl_Scroll)

WIDGET_DECLARE(Fl_Tabs)

GROUP_DECLARE(Fl_Tabs)

WIDGET_DECLARE(Fl_Tile)

GROUP_DECLARE(Fl_Tile)

WIDGET_DECLARE(Fl_Text_Display)

GROUP_DECLARE(Fl_Text_Display)

WIDGET_DECLARE(Fl_Text_Editor)

GROUP_DECLARE(Fl_Text_Editor)

void Fl_Text_Display_init(Fl_Text_Display*);

const char* Fl_Text_Display_text(Fl_Text_Display*);

void Fl_Text_Display_set_text(Fl_Text_Display*, const char*);

void Fl_Text_Editor_init(Fl_Text_Editor*);

const char* Fl_Text_Editor_text(Fl_Text_Editor*);

void Fl_Text_Editor_set_text(Fl_Text_Editor*, const char*);


#ifdef __cplusplus
}
#endif