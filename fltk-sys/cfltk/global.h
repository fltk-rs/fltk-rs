#pragma once

#ifdef __cplusplus
extern "C" {
#endif

typedef struct Fl_Widget Fl_Widget;
typedef void(Fl_Callback)(Fl_Widget *, void *);
typedef int (*custom_handler_callback)(int, void *);
void Fl_Widget_callback_with_captures(Fl_Widget *, Fl_Callback *cb, void *);

#define WIDGET_DECLARE(widget)                                                 \
  typedef struct widget widget;                                                \
  widget *widget##_new(int x, int y, int width, int height,                    \
                       const char *title);                                     \
  int widget##_x(widget *);                                                    \
  int widget##_y(widget *);                                                    \
  int widget##_width(widget *);                                                \
  int widget##_height(widget *);                                               \
  const char *widget##_label(widget *);                                        \
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
  unsigned int widget##_color(widget *);                                       \
  void widget##_set_color(widget *, unsigned int color);                       \
  unsigned int widget##_label_color(widget *);                                 \
  void widget##_set_label_color(widget *, unsigned int color);                 \
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
  void widget##_set_align(widget *, int typ);                                  \
  void widget##_delete(widget *);                                              \
  void widget##_set_image(widget *, void *);                                   \
  void widget##_set_handler(widget **self, custom_handler_callback cb,         \
                            void *data);                                       \
  void widget##_set_trigger(widget *, int);

#define GROUP_DECLARE(widget)                                                  \
  void widget##_begin(widget *self);                                           \
  void widget##_end(widget *self);                                             \
  int widget##_find(widget *self, const void *);                               \
  void widget##_add(widget *self, void *);                                     \
  void widget##_insert(widget *self, void *, int pos);                         \
  void widget##_remove(widget *self, int index);                               \
  void widget##_clear(widget *self);                                           \
  int widget##_children(widget *self);

#define WINDOW_DECLARE(widget)                                                 \
  void widget##_make_modal(widget *, unsigned int boolean);                    \
  void widget##_fullscreen(widget *, unsigned int boolean);                    \
  void widget##_make_current(widget *);                                        \
  void widget##_set_icon(widget *, const void *);                              \
  void widget##_make_resizable(widget *self, void *);

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
  unsigned int widget##_text_color(widget *);                                  \
  void widget##_set_text_color(widget *, unsigned int s);                      \
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
  unsigned int widget##_text_color(widget *);                                  \
  void widget##_set_text_color(widget *, unsigned int c);                      \
  void widget##_add_choice(widget *, const char *);                            \
  const char *widget##_get_choice(widget *);

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

#define DISPLAY_DECLARE(widget)                                                \
  int widget##_text_font(const widget *);                                      \
  void widget##_set_text_font(widget *, int s);                                \
  int widget##_text_size(const widget *);                                      \
  void widget##_set_text_size(widget *, int s);                                \
  unsigned int widget##_text_color(const widget *);                            \
  void widget##_set_text_color(widget *, unsigned int n);                      \
  const char *widget##_text(widget *);                                         \
  void widget##_set_text(widget *, const char *);                              \
  void widget##_append(widget *, const char *);                                \
  int widget##_buffer_length(const widget *);                                  \
  void widget##_scroll(widget *, int topLineNum, int horizOffset);             \
  void widget##_insert(widget *, const char *text);                            \
  void widget##_set_insert_position(widget *, int newPos);                     \
  int widget##_insert_position(const widget *);                                \
  int widget##_count_lines(const widget *, int start, int end,                 \
                           int start_pos_is_line_start);                       \
  int widget##_move_right(widget *);                                           \
  int widget##_move_left(widget *);                                            \
  int widget##_move_up(widget *);                                              \
  int widget##_move_down(widget *);                                            \
  void widget##_remove(widget *self, int start, int end);                      \
  void widget##_show_cursor(widget *, int boolean);                            \
  void widget##_set_style_table_entry(widget *self, unsigned int *color,       \
                                      int *font, int *fontsz, int sz);         \
  void widget##_set_cursor_style(widget *, int style);                         \
  void widget##_set_cursor_color(widget *, unsigned int color);                \
  void widget##_set_scrollbar_width(widget *, int width);                      \
  void widget##_set_scrollbar_size(widget *, int newSize);                     \
  void widget##_set_scrollbar_align(widget *, int align);                      \
  int widget##_cursor_style(widget *);                                         \
  unsigned int widget##_cursor_color(widget *);                                \
  int widget##_scrollbar_width(widget *);                                      \
  int widget##_scrollbar_size(widget *);                                       \
  int widget##_scrollbar_align(widget *);

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
  void widget##_set_text(widget *, int line, const char *newtext);

#define IMAGE_DECLARE(image)                                                   \
  typedef struct image image;                                                  \
  image *image##_new(const char *filename);                                    \
  void image##_draw(image *, int X, int Y, int W, int H);                      \
  int image##_width(image *);                                                  \
  int image##_height(image *);

#define WIDGET_DEFINE(widget)                                                  \
  class widget##_Derived : public widget {                                     \
    void *data_;                                                               \
                                                                               \
  public:                                                                      \
    typedef int (*handler)(int, void *data);                                   \
    handler inner_handler;                                                     \
    widget##_Derived(int x, int y, int w, int h, const char *title = 0)        \
        : widget(x, y, w, h, title) {}                                         \
    widget##_Derived(widget *ptr)                                              \
        : widget(ptr->x(), ptr->y(), ptr->w(), ptr->h(), ptr->label()) {}      \
    operator widget *() { return (widget *)*this; }                            \
    void set_handler(handler h) { inner_handler = h; }                         \
    void set_handler_data(void *data) { data_ = data; }                        \
    int handle(int event) {                                                    \
      int ret = widget::handle(event);                                         \
      int local = inner_handler(event, data_);                                 \
      if (local == 0)                                                          \
        return ret;                                                            \
      else                                                                     \
        return local;                                                          \
    }                                                                          \
  };                                                                           \
                                                                               \
  widget *widget##_new(int x, int y, int width, int height,                    \
                       const char *title) {                                    \
    return new (std::nothrow) widget(x, y, width, height, title);              \
  }                                                                            \
  int widget##_x(widget *self) { return self->x(); }                           \
  int widget##_y(widget *self) { return self->y(); }                           \
  int widget##_width(widget *self) { return self->w(); }                       \
  int widget##_height(widget *self) { return self->h(); }                      \
  const char *widget##_label(widget *self) { return self->label(); }           \
  void widget##_set_label(widget *self, const char *title) {                   \
    self->copy_label(title);                                                   \
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
    self->copy_tooltip(txt);                                                   \
  }                                                                            \
  int widget##_get_type(widget *self) { return self->type(); }                 \
  void widget##_set_type(widget *self, int typ) { self->type(typ); }           \
  unsigned int widget##_color(widget *self) { return self->color(); }          \
  void widget##_set_color(widget *self, unsigned int color) {                  \
    self->color(color);                                                        \
  }                                                                            \
  unsigned int widget##_label_color(widget *self) {                            \
    return self->labelcolor();                                                 \
  }                                                                            \
  void widget##_set_label_color(widget *self, unsigned int color) {            \
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
  void widget##_set_align(widget *self, int typ) { self->align(typ); }         \
  void widget##_delete(widget *self) { delete self; }                          \
  void widget##_set_image(widget *self, void *image) {                         \
    self->image((Fl_Image *)image);                                            \
  }                                                                            \
  void widget##_set_handler(widget **self, custom_handler_callback cb,         \
                            void *data) {                                      \
    widget##_Derived *temp = new widget##_Derived(*self);                      \
    temp->set_handler_data(data);                                              \
    temp->set_handler(cb);                                                     \
    *self = temp;                                                              \
  }                                                                            \
  void widget##_set_trigger(widget *self, int val) { self->when(val); }

#define GROUP_DEFINE(widget)                                                   \
  void widget##_begin(widget *self) { self->begin(); }                         \
  void widget##_end(widget *self) { self->end(); }                             \
  int widget##_find(widget *self, const void *wid) {                           \
    return self->find((const Fl_Widget *)wid);                                 \
  }                                                                            \
  void widget##_add(widget *self, void *wid) { self->add((Fl_Widget *)wid); }  \
  void widget##_insert(widget *self, void *wid, int pos) {                     \
    self->insert(*(Fl_Widget *)wid, pos);                                      \
  }                                                                            \
  void widget##_remove(widget *self, int index) { self->remove(index); }       \
  void widget##_clear(widget *self) { self->clear(); }                         \
  int widget##_children(widget *self) { return self->children(); }

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
  void widget##_make_current(widget *self) {                                   \
    ((Fl_Window *)self)->make_current();                                       \
  }                                                                            \
  void widget##_set_icon(widget *self, const void *image) {                    \
    self->icon((const Fl_RGB_Image *)image);                                   \
  }                                                                            \
  void widget##_make_resizable(widget *self, void *wid) {                      \
    self->resizable((Fl_Widget *)wid);                                         \
  }

#define INPUT_DEFINE(widget)                                                   \
  int widget##_set_value(widget *self, const char *t) {                        \
    return self->value(t);                                                     \
  }                                                                            \
  const char *widget##_value(widget *self) {                                   \
    std::string temp = self->value();                                          \
    char *ptr = new char[temp.size() + 1];                                     \
    strcpy(ptr, temp.c_str());                                                 \
    return ptr;                                                                \
  }                                                                            \
  int widget##_maximum_size(widget *self) { return self->maximum_size(); }     \
  void widget##_set_maximum_size(widget *self, int m) {                        \
    self->maximum_size(m);                                                     \
  }                                                                            \
  int widget##_position(widget *self) { return self->position(); }             \
  int widget##_set_position(widget *self, int p) { return self->position(p); } \
  int widget##_set_mark(widget *self, int m) { return self->mark(m); }         \
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
  unsigned int widget##_text_color(widget *self) { return self->textcolor(); } \
  void widget##_set_text_color(widget *self, unsigned int s) {                 \
    self->textcolor(s);                                                        \
  }                                                                            \
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
  unsigned int widget##_text_color(widget *self) { return self->textcolor(); } \
  void widget##_set_text_color(widget *self, unsigned int c) {                 \
    self->textcolor(c);                                                        \
  }                                                                            \
  void widget##_add_choice(widget *self, const char *str) { self->add(str); }  \
  const char *widget##_get_choice(widget *self) { return self->text(); }

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

#define DISPLAY_DEFINE(widget)                                                 \
  const char *widget##_text(widget *self) { return self->buffer()->text(); }   \
  void widget##_set_text(widget *self, const char *txt) {                      \
    self->buffer()->text(txt);                                                 \
  }                                                                            \
  int widget##_text_font(const widget *self) { return self->textfont(); }      \
  void widget##_set_text_font(widget *self, int s) { self->textfont(s); }      \
  int widget##_text_size(const widget *self) { return self->textsize(); }      \
  void widget##_set_text_size(widget *self, int s) { self->textsize(s); }      \
  unsigned int widget##_text_color(const widget *self) {                       \
    return self->textcolor();                                                  \
  }                                                                            \
  void widget##_set_text_color(widget *self, unsigned int n) {                 \
    self->textcolor(n);                                                        \
  }                                                                            \
  void widget##_append(widget *self, const char *txt) {                        \
    auto buff = self->buffer();                                                \
    buff->append(txt);                                                         \
    self->buffer(buff);                                                        \
    self->insert_position(self->buffer()->length());                           \
    self->scroll(self->count_lines(0, self->buffer()->length(), 1), 0);        \
  }                                                                            \
  int widget##_buffer_length(const widget *self) {                             \
    return self->buffer()->length();                                           \
  }                                                                            \
  void widget##_scroll(widget *self, int topLineNum, int horizOffset) {        \
    self->scroll(topLineNum, horizOffset);                                     \
  }                                                                            \
  void widget##_insert(widget *self, const char *text) {                       \
    ((Fl_Text_Display *)self)->insert(text);                                   \
  }                                                                            \
  void widget##_set_insert_position(widget *self, int newPos) {                \
    self->insert_position(newPos);                                             \
  }                                                                            \
  int widget##_insert_position(const widget *self) {                           \
    return self->insert_position();                                            \
  }                                                                            \
  int widget##_count_lines(const widget *self, int start, int end,             \
                           int start_pos_is_line_start) {                      \
    return self->count_lines(start, end, start_pos_is_line_start);             \
  }                                                                            \
  int widget##_move_right(widget *self) { return self->move_right(); }         \
  int widget##_move_left(widget *self) { return self->move_left(); }           \
  int widget##_move_up(widget *self) { return self->move_up(); }               \
  int widget##_move_down(widget *self) { return self->move_down(); }           \
  void widget##_remove(widget *self, int start, int end) {                     \
    auto buff = self->buffer();                                                \
    buff->remove(start, end);                                                  \
    self->buffer(buff);                                                        \
  }                                                                            \
  void widget##_show_cursor(widget *self, int boolean) {                       \
    if (boolean)                                                               \
      self->show_cursor();                                                     \
    else                                                                       \
      self->hide_cursor();                                                     \
  }                                                                            \
  void widget##_set_style_table_entry(widget *self, unsigned int *color,       \
                                      int *font, int *fontsz, int sz) {        \
    Fl_Text_Display::Style_Table_Entry *stable =                               \
        new Fl_Text_Display::Style_Table_Entry[sz];                            \
    for (int i = 0; i < sz; ++i) {                                             \
      stable[i] = {color[i], font[i], fontsz[i]};                              \
    }                                                                          \
    Fl_Text_Buffer *sbuff = new Fl_Text_Buffer();                              \
    self->highlight_data(sbuff, stable, sz, 'A', 0, 0);                        \
  }                                                                            \
  void widget##_set_cursor_style(widget *self, int style) {                    \
    self->cursor_style(style);                                                 \
  }                                                                            \
  void widget##_set_cursor_color(widget *self, unsigned int color) {           \
    self->cursor_color(color);                                                 \
  }                                                                            \
  void widget##_set_scrollbar_width(widget *self, int width) {                 \
    self->scrollbar_width(width);                                              \
  }                                                                            \
  void widget##_set_scrollbar_size(widget *self, int newSize) {                \
    self->scrollbar_size(newSize);                                             \
  }                                                                            \
  void widget##_set_scrollbar_align(widget *self, int align) {                 \
    self->scrollbar_align(align);                                              \
  }                                                                            \
  int widget##_cursor_style(widget *self) { return self->cursor_style(); }     \
  unsigned int widget##_cursor_color(widget *self) {                           \
    return self->cursor_color();                                               \
  }                                                                            \
  int widget##_scrollbar_width(widget *self) {                                 \
    return self->scrollbar_width();                                            \
  }                                                                            \
  int widget##_scrollbar_size(widget *self) { return self->scrollbar_size(); } \
  int widget##_scrollbar_align(widget *self) { return self->scrollbar_align(); }

#define BROWSER_DEFINE(widget)                                                 \
  void widget##_remove(widget *self, int line) { self->remove(line); }         \
  void widget##_add(widget *self, const char *newtext) { self->add(newtext); } \
  void widget##_insert(widget *self, int line, const char *newtext) {          \
    self->insert(line, newtext);                                               \
  }                                                                            \
  void widget##_move(widget *self, int to, int from) { self->move(to, from); } \
  void widget##_swap(widget *self, int a, int b) { self->swap(a, b); }         \
  void widget##_clear(widget *self) { self->clear(); }                         \
  int widget##_size(const widget *self) { return self->size(); }               \
  void widget##_set_size(widget *self, int W, int H) { self->size(W, H); }     \
  int widget##_select(widget *self, int line) { return self->select(line); }   \
  int widget##_selected(const widget *self, int line) {                        \
    return self->selected(line);                                               \
  }                                                                            \
  const char *widget##_text(const widget *self, int line) {                    \
    return self->text(line);                                                   \
  }                                                                            \
  void widget##_set_text(widget *self, int line, const char *newtext) {        \
    self->text(line, newtext);                                                 \
  }

#define IMAGE_DEFINE(image)                                                    \
  image *image##_new(const char *filename) {                                   \
    return new (std::nothrow) image(filename);                                 \
  }                                                                            \
  void image##_draw(image *self, int X, int Y, int W, int H) {                 \
    self->draw(X, Y, W, H);                                                    \
  }                                                                            \
  int image##_width(image *self) { return self->w(); }                         \
  int image##_height(image *self) { return self->h(); }

#ifdef __cplusplus
}
#endif