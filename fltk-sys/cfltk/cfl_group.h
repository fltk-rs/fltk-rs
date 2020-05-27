#pragma once

#include "cfl_widget.h"

#ifdef __cplusplus
extern "C" {
#endif

#define GROUP_DECLARE(widget)                                                  \
    void widget##_begin(widget *self);                                         \
    void widget##_end(widget *self);                                           \
    int widget##_find(widget *self, const void *);                             \
    void widget##_add(widget *self, void *);                                   \
    void widget##_insert(widget *self, void *, int pos);                       \
    void widget##_remove(widget *self, void *wid);                             \
    void widget##_clear(widget *self);                                         \
    int widget##_children(widget *self);                                       \
    Fl_Widget *widget##_child(widget *, int index);                            \
    void widget##_resizable(widget *self, void *);

#define GROUP_DEFINE(widget)                                                   \
    void widget##_begin(widget *self) { LOCK(self->begin();) }                 \
    void widget##_end(widget *self) { LOCK(self->end();) }                     \
    int widget##_find(widget *self, const void *wid) {                         \
        return self->find((const Fl_Widget *)wid);                             \
    }                                                                          \
    void widget##_add(widget *self, void *wid) {                               \
        LOCK(self->add((Fl_Widget *)wid);)                                     \
    }                                                                          \
    void widget##_insert(widget *self, void *wid, int pos) {                   \
        LOCK(self->insert(*(Fl_Widget *)wid, pos);)                            \
    }                                                                          \
    void widget##_remove(widget *self, void *wid) {                            \
        LOCK(self->remove(*(Fl_Widget *)wid);)                                 \
    }                                                                          \
    void widget##_clear(widget *self) { LOCK(self->clear();) }                 \
    int widget##_children(widget *self) { return self->children(); }           \
    Fl_Widget *widget##_child(widget *self, int index) {                       \
        return self->child(index);                                             \
    }                                                                          \
    void widget##_resizable(widget *self, void *wid) {                         \
        LOCK(self->resizable((Fl_Widget *)wid);)                               \
    }

WIDGET_DECLARE(Fl_Group)

GROUP_DECLARE(Fl_Group)

WIDGET_DECLARE(Fl_Pack)

GROUP_DECLARE(Fl_Pack)

WIDGET_DECLARE(Fl_Scroll)

int Fl_Scroll_xposition(const Fl_Scroll *self);

int Fl_Scroll_yposition(const Fl_Scroll *self);

void Fl_Scroll_scroll_to(Fl_Scroll *self, int, int);

int Fl_Scroll_scrollbar_size(const Fl_Scroll *self);

void Fl_Scroll_set_scrollbar_size(Fl_Scroll *self, int newSize);

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

WIDGET_DECLARE(Fl_Color_Chooser)

double Fl_Color_Chooser_r(Fl_Color_Chooser *self);

double Fl_Color_Chooser_g(Fl_Color_Chooser *self);

double Fl_Color_Chooser_b(Fl_Color_Chooser *self);

GROUP_DECLARE(Fl_Color_Chooser)

int Fl_Pack_spacing(Fl_Pack *self);

void Fl_Pack_set_spacing(Fl_Pack *self,int spacing);

#ifdef __cplusplus
}
#endif