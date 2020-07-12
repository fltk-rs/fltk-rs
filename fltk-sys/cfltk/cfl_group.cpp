#include "cfl_group.h"
#include <FL/Fl.H>
#include <FL/Fl_Color_Chooser.H>
#include <FL/Fl_Group.H>
#include <FL/Fl_Image.H>
#include <FL/Fl_Pack.H>
#include <FL/Fl_Scroll.H>
#include <FL/Fl_Tabs.H>
#include <FL/Fl_Tile.H>
#include <FL/Fl_Widget.H>
#include <FL/Fl_Wizard.H>
#include <new>

WIDGET_DEFINE(Fl_Group)

GROUP_DEFINE(Fl_Group)

WIDGET_DEFINE(Fl_Pack)

GROUP_DEFINE(Fl_Pack)

WIDGET_DEFINE(Fl_Scroll)

void *Fl_Scroll_scrollbar(Fl_Scroll *self) {
    return &self->scrollbar;
}

void *Fl_Scroll_hscrollbar(Fl_Scroll *self) {
    return &self->hscrollbar;
}

void *Fl_Scroll_hscrollbar(const Fl_Scroll *self);

int Fl_Scroll_xposition(const Fl_Scroll *self) {
    return self->xposition();
}

int Fl_Scroll_yposition(const Fl_Scroll *self) {
    return self->yposition();
}

void Fl_Scroll_scroll_to(Fl_Scroll *self, int x, int y) {
    LOCK(self->scroll_to(x, y);)
}

int Fl_Scroll_scrollbar_size(const Fl_Scroll *self) {
    return self->scrollbar_size();
}

void Fl_Scroll_set_scrollbar_size(Fl_Scroll *self, int newSize) {
    LOCK(self->scrollbar_size(newSize);)
}

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
    return (Fl_Widget *)self->value();
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

int Fl_Pack_spacing(Fl_Pack *self) {
    return self->spacing();
}
void Fl_Pack_set_spacing(Fl_Pack *self, int spacing) {
    self->spacing(spacing);
}
