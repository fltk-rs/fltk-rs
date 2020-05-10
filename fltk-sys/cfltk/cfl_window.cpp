#include "cfl_window.h"
#include <FL/Fl.H>
#include <FL/Fl_Double_Window.H>
#include <FL/Fl_Image.H>
#include <FL/Fl_Menu_Window.H>
#include <FL/Fl_RGB_Image.H>
#include <FL/Fl_Single_Window.H>
#include <FL/Fl_Window.H>
#include <new>

#define WINDOW_DEFINE(widget)                                                  \
    void widget##_make_modal(widget *self, unsigned int boolean) {             \
        LOCK(if (boolean) { self->set_modal(); } else {                        \
            self->set_non_modal();                                             \
        })                                                                     \
    }                                                                          \
    void widget##_fullscreen(widget *self, unsigned int boolean) {             \
        LOCK(if (boolean) { self->fullscreen(); } else {                       \
            self->fullscreen_off();                                            \
        })                                                                     \
    }                                                                          \
    void widget##_make_current(widget *self) {                                 \
        LOCK(((Fl_Window *)self)->make_current();)                             \
    }                                                                          \
    void widget##_set_icon(widget *self, const void *image) {                  \
        LOCK(self->icon((const Fl_RGB_Image *)((Fl_Image *)image)->copy());)   \
    }                                                                          \
    void widget##_make_resizable(widget *self, void *wid) {                    \
        LOCK(self->resizable((Fl_Widget *)wid);)                               \
    }                                                                          \
    void *widget##_icon(const widget *self) { return (Fl_Image *)self->icon(); }

WIDGET_DEFINE(Fl_Window)

GROUP_DEFINE(Fl_Window)

WINDOW_DEFINE(Fl_Window)

WIDGET_DEFINE(Fl_Double_Window)

GROUP_DEFINE(Fl_Double_Window)

WINDOW_DEFINE(Fl_Double_Window)

WIDGET_DEFINE(Fl_Menu_Window)

GROUP_DEFINE(Fl_Menu_Window)

WINDOW_DEFINE(Fl_Menu_Window)