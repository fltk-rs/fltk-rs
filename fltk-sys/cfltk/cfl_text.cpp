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

int kf_copy(Fl_Text_Editor* e) {
    return Fl_Text_Editor::kf_copy(1, e);
}

int kf_cut(Fl_Text_Editor* e) {
    return Fl_Text_Editor::kf_cut(1, e);
}

int kf_paste(Fl_Text_Editor* e) {
    return Fl_Text_Editor::kf_paste(1, e);
}

int kf_undo(Fl_Text_Editor* e) {
    return Fl_Text_Editor::kf_undo(1, e);
}

int text_font(const Fl_Text_Display* self) {
    return self->textfont();
}

void set_text_font(Fl_Text_Display* self, int s) {
    self->textfont(s);
}

int text_size(const Fl_Text_Display* self) {
    return self->textsize();
}

void set_text_size(Fl_Text_Display* self, int s) {
    self->textsize(s);
}

int text_color(const Fl_Text_Display* self) {
    return self->textcolor();
}

void set_text_color(Fl_Text_Display* self, int n) {
    self->textcolor(n);
}