#include "cfl_menu.h"
#include <Fl/Fl_Choice.H>
#include <Fl/Fl_Menu_Bar.H>
#include <Fl/Fl_Menu_Button.H>
#include <Fl/Fl_Menu_Item.H>
#include <FL/Fl_Image.H>
#include <cstring>
#include <string>

WIDGET_DEFINE(Fl_Menu_Bar)

MENU_DEFINE(Fl_Menu_Bar)

WIDGET_DEFINE(Fl_Menu_Button)

MENU_DEFINE(Fl_Menu_Button)

WIDGET_DEFINE(Fl_Choice)

MENU_DEFINE(Fl_Choice)

const char *Fl_Menu_Item_label(Fl_Menu_Item *self) { return self->label(); }

void Fl_Menu_Item_set_label(Fl_Menu_Item *self, const char *a) {
  self->label(a);
}

int Fl_Menu_Item_label_type(Fl_Menu_Item *self) { return self->labeltype(); }

void Fl_Menu_Item_set_label_type(Fl_Menu_Item *self, int a) {
  self->labeltype(static_cast<Fl_Labeltype>(a));
}

int Fl_Menu_Item_label_color(Fl_Menu_Item *self) { return self->labelcolor(); }

void Fl_Menu_Item_set_label_color(Fl_Menu_Item *self, int a) {
  self->labelcolor(a);
}

int Fl_Menu_Item_label_font(Fl_Menu_Item *self) { return self->labelfont(); }

void Fl_Menu_Item_set_label_font(Fl_Menu_Item *self, int a) {
  self->labelfont(a);
}

int Fl_Menu_Item_label_size(Fl_Menu_Item *self) { return self->labelsize(); }

void Fl_Menu_Item_set_label_size(Fl_Menu_Item *self, int a) {
  self->labelsize(a);
}

int Fl_Menu_Item_value(Fl_Menu_Item *self) { return self->value(); }

void Fl_Menu_Item_set(Fl_Menu_Item *self) { self->set(); }

void Fl_Menu_Item_clear(Fl_Menu_Item *self) { self->clear(); }

int Fl_Menu_Item_visible(Fl_Menu_Item *self) { return self->visible(); }

void Fl_Menu_Item_show(Fl_Menu_Item *self) { self->show(); }

void Fl_Menu_Item_hide(Fl_Menu_Item *self) { self->hide(); }

int Fl_Menu_Item_active(Fl_Menu_Item *self) { return self->active(); }

void Fl_Menu_Item_activate(Fl_Menu_Item *self) { self->activate(); }

void Fl_Menu_Item_deactivate(Fl_Menu_Item *self) { self->deactivate(); }
