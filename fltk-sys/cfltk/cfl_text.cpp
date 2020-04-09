#include "cfl_text.h"
#include <FL/Fl_Image.H>
#include <FL/Fl_Simple_Terminal.H>
#include <FL/Fl_Text_Buffer.H>
#include <FL/Fl_Text_Display.H>
#include <FL/Fl_Text_Editor.H>
#include <FL/Fl_Widget.H>
#include <cstring>
#include <string>

Fl_Text_Buffer *Fl_Text_Buffer_new(void) {
  return new (std::nothrow) Fl_Text_Buffer;
}

void Fl_Text_Buffer_delete(Fl_Text_Buffer *self) { delete self; }

const char *Fl_Text_Buffer_text(Fl_Text_Buffer *self) { return self->text(); }

void Fl_Text_Buffer_set_text(Fl_Text_Buffer *self, const char *txt) {
  LOCK(self->text(txt);)
}

void Fl_Text_Buffer_append(Fl_Text_Buffer *self, const char *txt) {
  LOCK(self->append(txt);)
}

void Fl_Text_Buffer_remove(Fl_Text_Buffer *self, int start, int end) {
  LOCK(self->remove(start, end);)
}

int Fl_Text_Buffer_length(const Fl_Text_Buffer *self) { return self->length(); }

WIDGET_DEFINE(Fl_Text_Display)

void Fl_Text_Display_init(Fl_Text_Display *self) {
  Fl_Text_Buffer *buff = new (std::nothrow) Fl_Text_Buffer();
  self->buffer(buff);
}

Fl_Text_Buffer *Fl_Text_Display_get_buffer(Fl_Text_Display *self) {
  return self->buffer();
}

void Fl_Text_Display_set_buffer(Fl_Text_Display *self, Fl_Text_Buffer *buf) {
  LOCK(self->buffer(buf);)
}

DISPLAY_DEFINE(Fl_Text_Display)

WIDGET_DEFINE(Fl_Text_Editor)

void Fl_Text_Editor_init(Fl_Text_Editor *self) {
  Fl_Text_Buffer *buff = new (std::nothrow) Fl_Text_Buffer();
  self->buffer(buff);
}

Fl_Text_Buffer *Fl_Text_Editor_get_buffer(Fl_Text_Editor *self) {
  return self->buffer();
}

void Fl_Text_Editor_set_buffer(Fl_Text_Editor *self, Fl_Text_Buffer *buf) {
  LOCK(self->buffer(buf);)
}

DISPLAY_DEFINE(Fl_Text_Editor)

int kf_copy(Fl_Text_Editor *e) {
  int ret;
  LOCK(ret = Fl_Text_Editor::kf_copy(1, e));
  return ret;
}

int kf_cut(Fl_Text_Editor *e) {
  int ret;
  LOCK(ret = Fl_Text_Editor::kf_cut(1, e));
  return ret;
}

int kf_paste(Fl_Text_Editor *e) {
  int ret;
  LOCK(ret = Fl_Text_Editor::kf_paste(1, e));
  return ret;
}

int kf_undo(Fl_Text_Editor *e) {
  int ret;
  LOCK(ret = Fl_Text_Editor::kf_undo(1, e));
  return ret;
}

WIDGET_DEFINE(Fl_Simple_Terminal)

void Fl_Simple_Terminal_init(Fl_Simple_Terminal *self) {
  Fl_Text_Buffer *buff = new (std::nothrow) Fl_Text_Buffer();
  self->buffer(buff);
}

Fl_Text_Buffer *Fl_Simple_Terminal_get_buffer(Fl_Simple_Terminal *self) {
  return self->buffer();
}

void Fl_Simple_Terminal_set_buffer(Fl_Simple_Terminal *self,
                                   Fl_Text_Buffer *buf) {
  LOCK(self->buffer(buf);)
}

DISPLAY_DEFINE(Fl_Simple_Terminal)