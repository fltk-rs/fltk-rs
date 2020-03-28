#include "cfl_text.h"
#include <FL/Fl_Image.H>
#include <FL/Fl_Text_Display.H>
#include <FL/Fl_Text_Editor.H>
#include <FL/Fl_Widget.H>
#include <cstring>
#include <string>

WIDGET_DEFINE(Fl_Text_Display)

void Fl_Text_Display_init(Fl_Text_Display *self) {
  Fl_Text_Buffer *buff = new Fl_Text_Buffer();
  self->buffer(buff);
}

DISPLAY_DEFINE(Fl_Text_Display)

WIDGET_DEFINE(Fl_Text_Editor)

void Fl_Text_Editor_init(Fl_Text_Editor *self) {
  Fl_Text_Buffer *buff = new Fl_Text_Buffer();
  self->buffer(buff);
}

DISPLAY_DEFINE(Fl_Text_Editor)

int kf_copy(Fl_Text_Editor *e) { return Fl_Text_Editor::kf_copy(1, e); }

int kf_cut(Fl_Text_Editor *e) { return Fl_Text_Editor::kf_cut(1, e); }

int kf_paste(Fl_Text_Editor *e) { return Fl_Text_Editor::kf_paste(1, e); }

int kf_undo(Fl_Text_Editor *e) { return Fl_Text_Editor::kf_undo(1, e); }
