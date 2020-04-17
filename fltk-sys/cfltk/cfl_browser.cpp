#include "cfl_browser.h"
#include <FL/Fl_Browser.H>
#include <FL/Fl_File_Browser.H>
#include <FL/Fl_Hold_Browser.H>
#include <FL/Fl_Image.H>
#include <FL/Fl_Multi_Browser.H>
#include <FL/Fl_Select_Browser.H>
#include <new>

#define BROWSER_DEFINE(widget)                                                 \
  void widget##_remove(widget *self, int line) { LOCK(self->remove(line);) }   \
  void widget##_add(widget *self, const char *newtext) {                       \
    LOCK(self->add(newtext);)                                                  \
  }                                                                            \
  void widget##_insert(widget *self, int line, const char *newtext) {          \
    LOCK(self->insert(line, newtext);)                                         \
  }                                                                            \
  void widget##_move(widget *self, int to, int from) {                         \
    LOCK(self->move(to, from);)                                                \
  }                                                                            \
  void widget##_swap(widget *self, int a, int b) { LOCK(self->swap(a, b);) }   \
  void widget##_clear(widget *self) { LOCK(self->clear();) }                   \
  int widget##_size(const widget *self) { return self->size(); }               \
  void widget##_set_size(widget *self, int W, int H) {                         \
    LOCK(self->size(W, H);)                                                    \
  }                                                                            \
  int widget##_select(widget *self, int line) {                                \
    int ret;                                                                   \
    LOCK(ret = self->select(line));                                            \
    return ret;                                                                \
  }                                                                            \
  int widget##_selected(const widget *self, int line) {                        \
    return self->selected(line);                                               \
  }                                                                            \
  const char *widget##_text(const widget *self, int line) {                    \
    return self->text(line);                                                   \
  }                                                                            \
  void widget##_set_text(widget *self, int line, const char *newtext) {        \
    LOCK(self->text(line, newtext);)                                           \
  }                                                                            \
  void widget##_load_file(widget *self, const char *file) {                    \
    LOCK(self->load(file);)                                                    \
  }                                                                            \
  int widget##_text_size(widget *self) { return self->textsize(); }            \
  void widget##_set_text_size(widget *self, int s) {                           \
    LOCK(self->textsize(s);)                                                   \
  }                                                                            \
  void widget##_set_icon(widget *self, int line, void *icon) {                 \
    LOCK(self->icon(line, ((Fl_Image *)icon)->copy());)                        \
  }                                                                            \
  void *widget##_icon(const widget *self, int line) {                          \
    return (Fl_Image *)self->icon(line);                                       \
  }                                                                            \
  void widget##_remove_icon(widget *self, int l) { LOCK(self->remove_icon(l);) }

WIDGET_DEFINE(Fl_Browser)

BROWSER_DEFINE(Fl_Browser)

WIDGET_DEFINE(Fl_Hold_Browser)

BROWSER_DEFINE(Fl_Hold_Browser)

WIDGET_DEFINE(Fl_Select_Browser)

BROWSER_DEFINE(Fl_Select_Browser)

WIDGET_DEFINE(Fl_Multi_Browser)

BROWSER_DEFINE(Fl_Multi_Browser)

WIDGET_DEFINE(Fl_File_Browser)

BROWSER_DEFINE(Fl_File_Browser)