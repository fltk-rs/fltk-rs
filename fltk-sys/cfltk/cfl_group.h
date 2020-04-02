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

WIDGET_DECLARE(Fl_Wizard)

void Fl_Wizard_next(Fl_Wizard *);
void Fl_Wizard_prev(Fl_Wizard *);
Fl_Widget *Fl_Wizard_value(Fl_Wizard *);
void Fl_Wizard_set_value(Fl_Wizard *, Fl_Widget *);

GROUP_DECLARE(Fl_Wizard)

#ifdef __cplusplus
}
#endif