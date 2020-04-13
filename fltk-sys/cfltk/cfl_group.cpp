#include "cfl_group.h"
#include <FL/Fl_Color_Chooser.H>
#include <FL/Fl_Group.H>
#include <FL/Fl_Image.H>
#include <FL/Fl_Pack.H>
#include <FL/Fl_Scroll.H>
#include <FL/Fl_Tabs.H>
#include <FL/Fl_Tile.H>
#include <FL/Fl_Widget.H>
#include <FL/Fl_Wizard.H>
#include <cstring>
#include <string>

WIDGET_DEFINE(Fl_Group)

GROUP_DEFINE(Fl_Group)

WIDGET_DEFINE(Fl_Pack)

GROUP_DEFINE(Fl_Pack)

WIDGET_DEFINE(Fl_Scroll)

GROUP_DEFINE(Fl_Scroll)

WIDGET_DEFINE(Fl_Tabs)

GROUP_DEFINE(Fl_Tabs)

WIDGET_DEFINE(Fl_Tile)

GROUP_DEFINE(Fl_Tile)

WIDGET_DEFINE(Fl_Wizard)

void Fl_Wizard_next(Fl_Wizard *self) {
    self->next();
}
void Fl_Wizard_prev(Fl_Wizard *self) {
    self->prev();
}
Fl_Widget *Fl_Wizard_value(Fl_Wizard *self) {
    return (Fl_Widget*)self->value();
}
void Fl_Wizard_set_value(Fl_Wizard *self, Fl_Widget *wid) {
    LOCK(self->value(wid);)
}

GROUP_DEFINE(Fl_Wizard)

WIDGET_DEFINE(Fl_Color_Chooser)

double Fl_Color_Chooser_r(Fl_Color_Chooser *self) {
    return self->r();
}
double Fl_Color_Chooser_g(Fl_Color_Chooser *self) {
    return self->g();
}
double Fl_Color_Chooser_b(Fl_Color_Chooser *self) {
    return self->b();
}

GROUP_DEFINE(Fl_Color_Chooser)

// WIDGET_DEFINE(Fl_Table)

// GROUP_DEFINE(Fl_Table)