#include "cfl_menu.h"
#include <FL/Fl_Choice.H>
#include <FL/Fl_Image.H>
#include <FL/Fl_Menu_Bar.H>
#include <FL/Fl_Menu_Button.H>
#include <FL/Fl_Menu_Item.H>
#include <cstring>
#include <string>

WIDGET_DEFINE(Fl_Menu_Bar)

MENU_DEFINE(Fl_Menu_Bar)

WIDGET_DEFINE(Fl_Menu_Button)

MENU_DEFINE(Fl_Menu_Button)

WIDGET_DEFINE(Fl_Choice)

MENU_DEFINE(Fl_Choice)

Fl_Menu_Item *Fl_Menu_Item_new(void) {
  Fl_Menu_Item *items = new (std::nothrow) Fl_Menu_Item[10];
  if (!items) return NULL;
  std::memset(items, 0, 10);
  return items;
}

void Fl_Menu_Item_delete(Fl_Menu_Item *self) { delete[] self; }

void Fl_Menu_Item_add_choice(Fl_Menu_Item *self, const char *choice) {
  Fl_Menu_Item* i = self->next();
  *i = {choice};
}

const Fl_Menu_Item *Fl_Menu_Item_popup(Fl_Menu_Item *self, int x, int y) {
  return self->popup(x, y);
}

const char *Fl_Menu_Item_label(Fl_Menu_Item *self) { return self->label(); }

void Fl_Menu_Item_set_label(Fl_Menu_Item *self, const char *a) {
  LOCK(self->label(a);)
}

int Fl_Menu_Item_label_type(Fl_Menu_Item *self) { return self->labeltype(); }

void Fl_Menu_Item_set_label_type(Fl_Menu_Item *self, int a) {
  LOCK(self->labeltype(static_cast<Fl_Labeltype>(a));)
}

unsigned int Fl_Menu_Item_label_color(Fl_Menu_Item *self) {
  return self->labelcolor();
}

void Fl_Menu_Item_set_label_color(Fl_Menu_Item *self, unsigned int a) {
  LOCK(self->labelcolor(a);)
}

int Fl_Menu_Item_label_font(Fl_Menu_Item *self) { return self->labelfont(); }

void Fl_Menu_Item_set_label_font(Fl_Menu_Item *self, int a) {
  LOCK(self->labelfont(a);)
}

int Fl_Menu_Item_label_size(Fl_Menu_Item *self) { return self->labelsize(); }

void Fl_Menu_Item_set_label_size(Fl_Menu_Item *self, int a) {
  LOCK(self->labelsize(a);)
}

int Fl_Menu_Item_value(Fl_Menu_Item *self) { return self->value(); }

void Fl_Menu_Item_set(Fl_Menu_Item *self) { LOCK(self->set();) }

void Fl_Menu_Item_clear(Fl_Menu_Item *self) { LOCK(self->clear();) }

int Fl_Menu_Item_visible(Fl_Menu_Item *self) { return self->visible(); }

void Fl_Menu_Item_show(Fl_Menu_Item *self) { LOCK(self->show();) }

void Fl_Menu_Item_hide(Fl_Menu_Item *self) { LOCK(self->hide();) }

int Fl_Menu_Item_active(Fl_Menu_Item *self) { return self->active(); }

void Fl_Menu_Item_activate(Fl_Menu_Item *self) { LOCK(self->activate();) }

void Fl_Menu_Item_deactivate(Fl_Menu_Item *self) { LOCK(self->deactivate();) }
