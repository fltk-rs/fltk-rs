#include "cfl_text.h"
#include <FL/Fl_Text_Display.H>
#include <FL/Fl_Text_Editor.H>
#include <FL/Fl_Image.H>
#include <FL/Fl_Widget.H>
#include <cstring>
#include <string>

WIDGET_DEFINE(Fl_Text_Display)

const char *Fl_Text_Display_text(Fl_Text_Display *self) {
  return self->buffer()->text();
}

void Fl_Text_Display_set_text(Fl_Text_Display *self, const char *txt) {
  self->buffer()->text(txt);
}

void Fl_Text_Display_init(Fl_Text_Display *self) {
  Fl_Text_Buffer *buff = new Fl_Text_Buffer();
  self->buffer(buff);
}

WIDGET_DEFINE(Fl_Text_Editor)

void Fl_Text_Editor_init(Fl_Text_Editor *self) {
  Fl_Text_Buffer *buff = new Fl_Text_Buffer();
  self->buffer(buff);
}

const char *Fl_Text_Editor_text(Fl_Text_Editor *self) {
  return self->buffer()->text();
}

void Fl_Text_Editor_set_text(Fl_Text_Editor *self, const char *txt) {
  self->buffer()->text(txt);
}
