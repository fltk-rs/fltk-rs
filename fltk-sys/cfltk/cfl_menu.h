#pragma once

#include "cfl_widget.h"

#ifdef __cplusplus
extern "C" {
#endif

#define MENU_DECLARE(widget)                                                   \
  void widget##_add(widget *, const char *name, int shortcut, Fl_Callback *,   \
                    void *, int);                                              \
  void widget##_insert(widget *, int index, const char *name, int shortcut,    \
                       Fl_Callback *, void *, int);                            \
  Fl_Menu_Item *widget##_get_item(widget *, const char *name);                 \
  int widget##_text_font(widget *);                                            \
  void widget##_set_text_font(widget *, int c);                                \
  int widget##_text_size(widget *);                                            \
  void widget##_set_text_size(widget *, int c);                                \
  unsigned int widget##_text_color(widget *);                                  \
  void widget##_set_text_color(widget *, unsigned int c);                      \
  void widget##_add_choice(widget *, const char *);                            \
  const char *widget##_get_choice(widget *);

typedef struct Fl_Menu_Item Fl_Menu_Item;

WIDGET_DECLARE(Fl_Menu_Bar)

MENU_DECLARE(Fl_Menu_Bar)

WIDGET_DECLARE(Fl_Menu_Button)

MENU_DECLARE(Fl_Menu_Button)

WIDGET_DECLARE(Fl_Choice)

MENU_DECLARE(Fl_Choice)

Fl_Menu_Item *Fl_Menu_Item_new(char** args, int sz);

void Fl_Menu_Item_delete(Fl_Menu_Item *self);

const Fl_Menu_Item *Fl_Menu_Item_popup(Fl_Menu_Item *self, int x, int y);

const char *Fl_Menu_Item_label(Fl_Menu_Item *);

void Fl_Menu_Item_set_label(Fl_Menu_Item *, const char *a);

int Fl_Menu_Item_label_type(Fl_Menu_Item *);

void Fl_Menu_Item_set_label_type(Fl_Menu_Item *, int a);

unsigned int Fl_Menu_Item_label_color(Fl_Menu_Item *);

void Fl_Menu_Item_set_label_color(Fl_Menu_Item *, unsigned int a);

int Fl_Menu_Item_label_font(Fl_Menu_Item *);

void Fl_Menu_Item_set_label_font(Fl_Menu_Item *, int a);

int Fl_Menu_Item_label_size(Fl_Menu_Item *);

void Fl_Menu_Item_set_label_size(Fl_Menu_Item *, int a);

int Fl_Menu_Item_value(Fl_Menu_Item *);

void Fl_Menu_Item_set(Fl_Menu_Item *);

void Fl_Menu_Item_clear(Fl_Menu_Item *);

int Fl_Menu_Item_visible(Fl_Menu_Item *);

void Fl_Menu_Item_show(Fl_Menu_Item *);

void Fl_Menu_Item_hide(Fl_Menu_Item *);

int Fl_Menu_Item_active(Fl_Menu_Item *);

void Fl_Menu_Item_activate(Fl_Menu_Item *);

void Fl_Menu_Item_deactivate(Fl_Menu_Item *);

#ifdef __cplusplus
}
#endif