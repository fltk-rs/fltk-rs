#include "cfl_window.h"
#include <FL/Fl.H>
#include <FL/Fl_Double_Window.H>
#include <FL/Fl_Gl_Window.H>
#include <FL/Fl_Image.H>
#include <FL/Fl_Menu_Window.H>
#include <FL/Fl_RGB_Image.H>
#include <FL/Fl_Single_Window.H>
#include <FL/Fl_Window.H>
#include <FL/platform.H>
#include <new>

#define WINDOW_DEFINE(widget)                                                                      \
    void widget##_make_modal(widget *self, unsigned int boolean) {                                 \
        LOCK(if (boolean) { self->set_modal(); } else { self->set_non_modal(); })                  \
    }                                                                                              \
    void widget##_fullscreen(widget *self, unsigned int boolean) {                                 \
        LOCK(if (boolean) { self->fullscreen(); } else { self->fullscreen_off(); })                \
    }                                                                                              \
    void widget##_make_current(widget *self) {                                                     \
        LOCK(((Fl_Window *)self)->make_current();)                                                 \
    }                                                                                              \
    void widget##_set_icon(widget *self, const void *image) {                                      \
        LOCK(self->icon((const Fl_RGB_Image *)((Fl_Image *)image));)                               \
    }                                                                                              \
    void widget##_make_resizable(widget *self, void *wid) {                                        \
        LOCK(self->resizable((Fl_Widget *)wid);)                                                   \
    }                                                                                              \
    void *widget##_icon(const widget *self) {                                                      \
        return (Fl_Image *)self->icon();                                                           \
    }                                                                                              \
    void widget##_set_cursor(widget *self, int cursor) {                                           \
        LOCK(self->cursor((Fl_Cursor)cursor);)                                                     \
    }                                                                                              \
    int widget##_shown(widget *self) {                                                             \
        return self->shown();                                                                      \
    }                                                                                              \
    void *widget##_raw_handle(const widget *w) {                                                   \
        return (void *)fl_xid_(w);                                                                 \
    }                                                                                              \
    void widget##_set_border(widget *self, int flag) {                                             \
        LOCK(self->border(flag);)                                                                  \
    }                                                                                              \
    int widget##_border(const widget *self) {                                                      \
        return self->border();                                                                     \
    }

WIDGET_DEFINE(Fl_Window)

GROUP_DEFINE(Fl_Window)

WINDOW_DEFINE(Fl_Window)

WIDGET_DEFINE(Fl_Double_Window)

GROUP_DEFINE(Fl_Double_Window)

WINDOW_DEFINE(Fl_Double_Window)

WIDGET_DEFINE(Fl_Menu_Window)

GROUP_DEFINE(Fl_Menu_Window)

WINDOW_DEFINE(Fl_Menu_Window)

#ifdef CFLTK_USE_GL

WIDGET_DEFINE(Fl_Gl_Window)

GROUP_DEFINE(Fl_Gl_Window)

WINDOW_DEFINE(Fl_Gl_Window)

void Fl_Gl_Window_flush(Fl_Gl_Window *self) {
    LOCK(self->flush();)
}

char Fl_Gl_Window_valid(const Fl_Gl_Window *self) {
    return self->valid();
}

void Fl_Gl_Window_set_valid(Fl_Gl_Window *self, char v) {
    LOCK(self->valid(v);)
}

char Fl_Gl_Window_context_valid(const Fl_Gl_Window *self) {
    return self->context_valid();
}

void Fl_Gl_Window_set_context_valid(Fl_Gl_Window *self, char v) {
    LOCK(self->context_valid(v);)
}

int Fl_Gl_Window_can_do(Fl_Gl_Window *self) {
    return self->can_do();
}

void *Fl_Gl_Window_context(const Fl_Gl_Window *self) {
    return self->context();
}

void Fl_Gl_Window_set_context(Fl_Gl_Window *self, void *ctx, int destroy_flag) {
    LOCK(self->context((GLContext)ctx, destroy_flag);)
}

void Fl_Gl_Window_swap_buffers(Fl_Gl_Window *self) {
    LOCK(self->swap_buffers();)
}

void Fl_Gl_Window_ortho(Fl_Gl_Window *self) {
    LOCK(self->ortho();)
}

int Fl_Gl_Window_can_do_overlay(Fl_Gl_Window *self) {
    return self->can_do_overlay();
}

void Fl_Gl_Window_redraw_overlay(Fl_Gl_Window *self) {
    LOCK(self->redraw_overlay();)
}

void Fl_Gl_Window_hide_overlay(Fl_Gl_Window *self) {
    LOCK(self->hide_overlay();)
}

void Fl_Gl_Window_make_overlay_current(Fl_Gl_Window *self) {
    LOCK(self->make_overlay_current();)
}

float Fl_Gl_Window_pixels_per_unit(Fl_Gl_Window *self) {
    return self->pixels_per_unit();
}

int Fl_Gl_Window_pixel_w(Fl_Gl_Window *self) {
    return self->pixel_w();
}

int Fl_Gl_Window_pixel_h(Fl_Gl_Window *self) {
    return self->pixel_h();
}

int Fl_Gl_Window_mode(const Fl_Gl_Window *self) {
    return self->mode();
}

void Fl_Gl_Window_set_mode(Fl_Gl_Window *self, int mode) {
    self->mode(mode);
}

#endif