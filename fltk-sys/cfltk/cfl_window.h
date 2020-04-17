#pragma once

#include "cfl_group.h"
#include "cfl_widget.h"

#ifdef __cplusplus
extern "C" {
#endif

#define WINDOW_DECLARE(widget)                                                 \
  void widget##_make_modal(widget *, unsigned int boolean);                    \
  void widget##_fullscreen(widget *, unsigned int boolean);                    \
  void widget##_make_current(widget *);                                        \
  void widget##_set_icon(widget *, const void *);                              \
  void *widget##_icon(const widget *);                                         \
  void widget##_make_resizable(widget *self, void *);

WIDGET_DECLARE(Fl_Window)

GROUP_DECLARE(Fl_Window)

WINDOW_DECLARE(Fl_Window)

WIDGET_DECLARE(Fl_Double_Window)

GROUP_DECLARE(Fl_Double_Window)

WINDOW_DECLARE(Fl_Double_Window)

WIDGET_DECLARE(Fl_Menu_Window)

GROUP_DECLARE(Fl_Menu_Window)

WINDOW_DECLARE(Fl_Menu_Window)

#ifdef __cplusplus
}
#endif