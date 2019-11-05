#pragma once

#ifdef __cplusplus
extern "C" {
#endif

typedef struct Fl_Widget Fl_Widget;
typedef void(Fl_Callback)(Fl_Widget *, void *);

void Fl_Widget_callback_with_captures(Fl_Widget *, Fl_Callback *cb, void *);

#define WIDGET_DECLARE(widget)                                                 \
  typedef struct widget widget;                                                \
  widget *widget##_new(int x, int y, int width, int height,                    \
                       const char *title);                                     \
  void widget##_set_label(widget *, const char *title);                        \
  void widget##_redraw(widget *);                                              \
  void widget##_show(widget *);                                                \
  void widget##_hide(widget *);                                                \
  void widget##_activate(widget *);                                            \
  void widget##_deactivate(widget *);                                          \
  void widget##_redraw_label(widget *);                                        \
  void widget##_resize(widget *, int x, int y, int width, int height);         \
  const char *widget##_tooltip(widget *);                                      \
  void widget##_set_tooltip(widget *, const char *txt);                        \
  int widget##_get_type(widget *);                                             \
  void widget##_set_type(widget *, int typ);                                   \
  int widget##_color(widget *);                                                \
  void widget##_set_color(widget *, int color);                                \
  int widget##_label_color(widget *);                                          \
  void widget##_set_label_color(widget *, int color);                          \
  int widget##_label_font(widget *);                                           \
  void widget##_set_label_font(widget *, int font);                            \
  int widget##_label_size(widget *);                                           \
  void widget##_set_label_size(widget *, int sz);                              \
  int widget##_label_type(widget *);                                           \
  void widget##_set_label_type(widget *, int typ);                             \
  int widget##_box(widget *);                                                  \
  void widget##_set_box(widget *, int typ);                                    \
  int widget##_changed(widget *);                                              \
  void widget##_set_changed(widget *);                                         \
  void widget##_clear_changed(widget *);                                       \
  int widget##_align(widget *);                                                \
  void widget##_set_align(widget *, int typ);

#define GROUP_DECLARE(widget)                                                  \
  void widget##_begin(widget *self);                                           \
  void widget##_end(widget *self);

#define WINDOW_DECLARE(widget)                                                 \
  void widget##_make_modal(widget *, unsigned int boolean);                    \
  void widget##_fullscreen(widget *, unsigned int boolean);                    \
  void widget##_make_current(widget *);

#define INPUT_DECLARE(widget)                                                  \
  int widget##_set_value(widget *, const char *);                              \
  const char *widget##_value(widget *);                                        \
  int widget##_maximum_size(widget *);                                         \
  void widget##_set_maximum_size(widget *, int m);                             \
  int widget##_position(widget *);                                             \
  int widget##_set_position(widget *, int p);                                  \
  int widget##_set_mark(widget *, int m);                                      \
  int widget##_mark(widget *);                                                 \
  int widget##_replace(widget *, int b, int e, const char *text, int ilen);    \
  int widget##_insert(widget *, const char *t, int l);                         \
  int widget##_append(widget *, const char *t, int l, char keep_selection);    \
  int widget##_copy(widget *, int clipboard);                                  \
  int widget##_undo(widget *);                                                 \
  int widget##_copy_cuts(widget *);                                            \
  int widget##_text_font(widget *);                                            \
  void widget##_set_text_font(widget *, int s);                                \
  int widget##_text_color(widget *);                                           \
  void widget##_set_text_color(widget *, int s);                               \
  int widget##_text_size(widget *);                                            \
  void widget##_set_text_size(widget *, int s);                                \
  int widget##_readonly(widget *);                                             \
  void widget##_set_readonly(widget *, int boolean);                           \
  int widget##_wrap(widget *);                                                 \
  void widget##_set_wrap(widget *, int boolean);

#define MENU_DECLARE(widget)                                                   \
  void widget##_add(widget *, const char *name, int shortcut, Fl_Callback *,   \
                    void *, int);                                              \
  Fl_Menu_Item *widget##_get_item(widget *, const char *name);                 \
  int widget##_text_font(widget *);                                            \
  void widget##_set_text_font(widget *, int c);                                \
  int widget##_text_size(widget *);                                            \
  void widget##_set_text_size(widget *, int c);                                \
  int widget##_text_color(widget *);                                           \
  void widget##_set_text_color(widget *, int c);

#define VALUATOR_DECLARE(widget)                                               \
  void widget##_set_bounds(widget *, double a, double b);                      \
  double widget##_minimum(widget *);                                           \
  void widget##_set_minimum(widget *, double a);                               \
  double widget##_maximum(widget *);                                           \
  void widget##_set_maximum(widget *, double a);                               \
  void widget##_set_range(widget *, double a, double b);                       \
  void widget##_set_step(widget *, double a, int b);                           \
  double widget##_step(widget *);                                              \
  void widget##_set_precision(widget *, int digits);                           \
  double widget##_value(widget *);                                             \
  int widget##_set_value(widget *, double);                                    \
  int widget##_format(widget *, char *);                                       \
  double widget##_round(widget *, double);                                     \
  double widget##_clamp(widget *, double);                                     \
  double widget##_increment(widget *, double, int);

#define WIDGET_DEFINE(widget)                                                  \
  widget *widget##_new(int x, int y, int width, int height,                    \
                       const char *title) {                                    \
    return new widget(x, y, width, height, title);                             \
  }                                                                            \
  void widget##_set_label(widget *self, const char *title) {                   \
    self->label(title);                                                        \
  }                                                                            \
  void widget##_redraw(widget *self) { self->redraw(); }                       \
  void widget##_show(widget *self) { self->show(); }                           \
  void widget##_hide(widget *self) { self->hide(); }                           \
  void widget##_activate(widget *self) { self->activate(); }                   \
  void widget##_deactivate(widget *self) { self->deactivate(); }               \
  void widget##_redraw_label(widget *self) { self->redraw_label(); }           \
  void widget##_resize(widget *self, int x, int y, int width, int height) {    \
    self->resize(x, y, width, height);                                         \
  }                                                                            \
  const char *widget##_tooltip(widget *self) { return self->tooltip(); }       \
  void widget##_set_tooltip(widget *self, const char *txt) {                   \
    self->tooltip(txt);                                                        \
  }                                                                            \
  int widget##_get_type(widget *self) { return self->type(); }                 \
  void widget##_set_type(widget *self, int typ) { self->type(typ); }           \
  int widget##_color(widget *self) { return self->color(); }                   \
  void widget##_set_color(widget *self, int color) { self->color(color); }     \
  int widget##_label_color(widget *self) { return self->labelcolor(); }        \
  void widget##_set_label_color(widget *self, int color) {                     \
    self->labelcolor(color);                                                   \
  }                                                                            \
  int widget##_label_font(widget *self) { return self->labelfont(); }          \
  void widget##_set_label_font(widget *self, int font) {                       \
    self->labelfont(font);                                                     \
  }                                                                            \
  int widget##_label_size(widget *self) { return self->labelsize(); }          \
  void widget##_set_label_size(widget *self, int sz) { self->labelsize(sz); }  \
  int widget##_label_type(widget *self) { return self->labeltype(); }          \
  void widget##_set_label_type(widget *self, int typ) {                        \
    self->labeltype(static_cast<Fl_Labeltype>(typ));                           \
  }                                                                            \
  int widget##_box(widget *self) { return self->box(); }                       \
  void widget##_set_box(widget *self, int typ) {                               \
    self->box(static_cast<Fl_Boxtype>(typ));                                   \
  }                                                                            \
  int widget##_changed(widget *self) { return self->changed(); }               \
  void widget##_set_changed(widget *self) { self->set_changed(); }             \
  void widget##_clear_changed(widget *self) { self->clear_changed(); }         \
  int widget##_align(widget *self) { return self->align(); }                   \
  void widget##_set_align(widget *self, int typ) { self->align(typ); }

#define GROUP_DEFINE(widget)                                                   \
  void widget##_begin(widget *self) { self->begin(); }                         \
  void widget##_end(widget *self) { self->end(); }

#define WINDOW_DEFINE(widget)                                                  \
  void widget##_make_modal(widget *self, unsigned int boolean) {               \
    if (boolean) {                                                             \
      self->set_modal();                                                       \
    } else {                                                                   \
      self->set_non_modal();                                                   \
    }                                                                          \
  }                                                                            \
  void widget##_fullscreen(widget *self, unsigned int boolean) {               \
    if (boolean) {                                                             \
      self->fullscreen();                                                      \
    } else {                                                                   \
      self->fullscreen_off();                                                  \
    }                                                                          \
  }                                                                            \
  void widget##_make_current(widget *self) { self->make_current(); }

#define INPUT_DEFINE(widget)                                                   \
  int widget##_set_value(widget *self, const char *t) { self->value(t); }      \
  const char *widget##_value(widget *self) { return self->value(); }           \
  int widget##_maximum_size(widget *self) { return self->maximum_size(); }     \
  void widget##_set_maximum_size(widget *self, int m) {                        \
    self->maximum_size(m);                                                     \
  }                                                                            \
  int widget##_position(widget *self) { self->position(); }                    \
  int widget##_set_position(widget *self, int p) { self->position(p); }        \
  int widget##_set_mark(widget *self, int m) { self->mark(m); }                \
  int widget##_mark(widget *self) { return self->mark(); }                     \
  int widget##_replace(widget *self, int b, int e, const char *text,           \
                       int ilen) {                                             \
    return self->replace(b, e, text, ilen);                                    \
  }                                                                            \
  int widget##_insert(widget *self, const char *t, int l) {                    \
    return self->insert(t, l);                                                 \
  }                                                                            \
  int widget##_append(widget *self, const char *t, int l,                      \
                      char keep_selection) {                                   \
    return self->append(t, l, keep_selection);                                 \
  }                                                                            \
  int widget##_copy(widget *self, int clipboard) {                             \
    return self->copy(clipboard);                                              \
  }                                                                            \
  int widget##_undo(widget *self) { return self->undo(); }                     \
  int widget##_copy_cuts(widget *self) { return self->cut(); }                 \
  int widget##_text_font(widget *self) { return self->textfont(); }            \
  void widget##_set_text_font(widget *self, int s) { self->textfont(s); }      \
  int widget##_text_color(widget *self) { return self->textcolor(); }          \
  void widget##_set_text_color(widget *self, int s) { self->textcolor(s); }    \
  int widget##_text_size(widget *self) { return self->textsize(); }            \
  void widget##_set_text_size(widget *self, int s) { self->textsize(s); }      \
  int widget##_readonly(widget *self) { return self->readonly(); }             \
  void widget##_set_readonly(widget *self, int boolean) {                      \
    self->readonly(boolean);                                                   \
  }                                                                            \
  int widget##_wrap(widget *self) { return self->wrap(); }                     \
  void widget##_set_wrap(widget *self, int boolean) { self->wrap(boolean); }

#define MENU_DEFINE(widget)                                                    \
  void widget##_add(widget *self, const char *name, int shortcut,              \
                    Fl_Callback *cb, void *data, int flag) {                   \
    self->add(name, shortcut, cb, data, flag);                                 \
  }                                                                            \
  Fl_Menu_Item *widget##_get_item(widget *self, const char *name) {            \
    return (Fl_Menu_Item *)self->find_item(name);                              \
  }                                                                            \
  int widget##_text_font(widget *self) { return self->textfont(); }            \
  void widget##_set_text_font(widget *self, int c) { self->textfont(c); }      \
  int widget##_text_size(widget *self) { return self->textsize(); }            \
  void widget##_set_text_size(widget *self, int c) { self->textsize(c); }      \
  int widget##_text_color(widget *self) { return self->textcolor(); }          \
  void widget##_set_text_color(widget *self, int c) { self->textcolor(c); }

#define VALUATOR_DEFINE(widget)                                                \
  void widget##_set_bounds(widget *self, double a, double b) {                 \
    self->bounds(a, b);                                                        \
  }                                                                            \
  double widget##_minimum(widget *self) { return self->minimum(); }            \
  void widget##_set_minimum(widget *self, double a) { self->minimum(a); }      \
  double widget##_maximum(widget *self) { return self->maximum(); }            \
  void widget##_set_maximum(widget *self, double a) { self->maximum(a); }      \
  void widget##_set_range(widget *self, double a, double b) {                  \
    self->range(a, b);                                                         \
  }                                                                            \
  void widget##_set_step(widget *self, double a, int b) { self->step(a, b); }  \
  double widget##_step(widget *self) { return self->step(); }                  \
  void widget##_set_precision(widget *self, int digits) {                      \
    self->precision(digits);                                                   \
  }                                                                            \
  double widget##_value(widget *self) { return self->value(); }                \
  int widget##_set_value(widget *self, double val) {                           \
    return self->value(val);                                                   \
  }                                                                            \
  int widget##_format(widget *self, char *chr) { return self->format(chr); }   \
  double widget##_round(widget *self, double val) { return self->round(val); } \
  double widget##_clamp(widget *self, double val) { return self->clamp(val); } \
  double widget##_increment(widget *self, double a, int b) {                   \
    return self->increment(a, b);                                              \
  }

#ifdef __cplusplus
}
#endif