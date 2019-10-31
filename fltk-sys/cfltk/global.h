#pragma once

#ifdef __cplusplus
extern "C" {
#endif

#define WIDGET_DECLARE(widget) \
typedef struct Fl_##widget Fl_##widget;\
Fl_##widget *Fl_##widget_new(int x, int y, int width, int height, const char *title);\
void Fl_##widget_set_label(Fl_##widget *, const char *title);\
void Fl_##widget_redraw(Fl_##widget *);\
void Fl_##widget_show(Fl_##widget *);\
void Fl_##widget_hide(Fl_##widget *);\
void Fl_##widget_activate(Fl_##widget *);\
void Fl_##widget_deactivate(Fl_##widget *);\
void Fl_##widget_redraw_label(Fl_##widget *);\
void Fl_##widget_resize(Fl_##widget *, int x, int y, int width, int height);\
const char* Fl_##widget_tooltip(Fl_##widget *);\
void Fl_##widget_set_tooltip(Fl_##widget *, const char* txt);\
int Fl_##widget_get_type(Fl_##widget *);\
void Fl_##widget_set_type(Fl_##widget *, int typ);\
int Fl_##widget_color(Fl_##widget *);\
void Fl_##widget_set_color(Fl_##widget *, int color);\
int Fl_##widget_label_color(Fl_##widget *);\
void Fl_##widget_set_label_color(Fl_##widget *, int color);\
int Fl_##widget_label_font(Fl_##widget *);\
void Fl_##widget_set_label_font(Fl_##widget *, int font);\
int Fl_##widget_label_size(Fl_##widget *);\
void Fl_##widget_set_label_size(Fl_##widget *, int sz);\
int Fl_##widget_label_type(Fl_##widget *);\
void Fl_##widget_set_label_type(Fl_##widget *, int typ);\
int Fl_##widget_box(Fl_##widget *);\
void Fl_##widget_set_box(Fl_##widget *, int typ);

#define GROUP_DECLARE(widget) \
void Fl_##widget_begin(Fl_##widget *self);\
void Fl_##widget_end(Fl_##widget *self);

#ifdef __cplusplus
}
#endif