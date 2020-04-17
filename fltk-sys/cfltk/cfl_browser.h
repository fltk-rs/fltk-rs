#pragma once

#include "cfl_widget.h"

#ifdef __cplusplus
extern "C" {
#endif

#define BROWSER_DECLARE(widget)                                                \
  void widget##_remove(widget *, int line);                                    \
  void widget##_add(widget *, const char *newtext);                            \
  void widget##_insert(widget *, int line, const char *newtext);               \
  void widget##_move(widget *, int to, int from);                              \
  void widget##_swap(widget *, int a, int b);                                  \
  void widget##_clear(widget *);                                               \
  int widget##_size(const widget *);                                           \
  void widget##_set_size(widget *, int W, int H);                              \
  int widget##_select(widget *, int line);                                     \
  int widget##_selected(const widget *, int line);                             \
  const char *widget##_text(const widget *, int line);                         \
  void widget##_set_text(widget *, int line, const char *newtext);             \
  void widget##_load_file(widget *, const char *file);                         \
  int widget##_text_size(widget *);                                            \
  void widget##_set_text_size(widget *, int s);                                \
  void widget##_set_icon(widget *, int line, void *icon);                      \
  void *widget##_icon(const widget *, int line);                               \
  void widget##_remove_icon(widget *, int line);

WIDGET_DECLARE(Fl_Browser)

BROWSER_DECLARE(Fl_Browser)

WIDGET_DECLARE(Fl_Hold_Browser)

BROWSER_DECLARE(Fl_Hold_Browser)

WIDGET_DECLARE(Fl_Select_Browser)

BROWSER_DECLARE(Fl_Select_Browser)

WIDGET_DECLARE(Fl_Multi_Browser)

BROWSER_DECLARE(Fl_Multi_Browser)

WIDGET_DECLARE(Fl_File_Browser)

BROWSER_DECLARE(Fl_File_Browser)

#ifdef __cplusplus
}
#endif