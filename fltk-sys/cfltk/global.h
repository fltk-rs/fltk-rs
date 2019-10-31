#pragma once

#ifdef __cplusplus
extern "C" {
#endif

#define WIDGET_DECLARE(widget)\
typedef struct widget widget; \
widget* widget## _new(int x, int y, int width, int height, const char *title); \
void widget## _set_label(widget *, const char *title); \
void widget## _redraw(widget *); \
void widget## _show(widget *); \
void widget## _hide(widget *); \
void widget## _activate(widget *); \
void widget## _deactivate(widget *); \
void widget## _redraw_label(widget *); \
void widget## _resize(widget *, int x, int y, int width, int height); \
const char* widget## _tooltip(widget *); \
void widget## _set_tooltip(widget *, const char* txt); \
int widget## _get_type(widget *); \
void widget## _set_type(widget *, int typ); \
int widget## _color(widget *); \
void widget## _set_color(widget *, int color); \
int widget## _label_color(widget *); \
void widget## _set_label_color(widget *, int color); \
int widget## _label_font(widget *); \
void widget## _set_label_font(widget *, int font); \
int widget## _label_size(widget *); \
void widget## _set_label_size(widget *, int sz); \
int widget## _label_type(widget *); \
void widget## _set_label_type(widget *, int typ); \
int widget## _box(widget *); \
void widget## _set_box(widget *, int typ); \

#define GROUP_DECLARE(widget) \
void widget## _begin(widget *self); \
void widget## _end(widget *self); \

#define WIDGET_DEFINE(widget) \
widget* widget## _new(int x, int y, int width, int height, const char *title) {\
    return new widget(x, y, width, height, title);\
}\
void widget## _set_label(widget *self, const char *title) {\
    self->label(title);\
}\
void widget## _redraw(widget *self) {\
    self->redraw();\
}\
void widget## _show(widget *self) {\
    self->show();\
}\
void widget## _hide(widget *self) {\
    self->hide();\
}\
void widget## _activate(widget *self) {\
    self->activate();\
}\
void widget## _deactivate(widget *self) {\
    self->deactivate();\
}\
void widget## _redraw_label(widget *self) {\
    self->redraw_label();\
}\
void widget## _resize(widget *self, int x, int y, int width, int height) {\
    self->resize(x, y, width, height);\
}\
const char* widget## _tooltip(widget *self) {\
    return self->tooltip();\
}\
void widget## _set_tooltip(widget *self, const char* txt) {\
    self->tooltip(txt);\
}\
int widget## _get_type(widget *self) {\
    return self->type();\
}\
void widget## _set_type(widget *self, int typ) {\
    self->type(typ);\
}\
int widget## _color(widget *self) {\
    return self->color();\
}\
void widget## _set_color(widget *self, int color) {\
    self->color(color);\
}\
int widget## _label_color(widget *self) {\
    return self->labelcolor();\
}\
void widget## _set_label_color(widget *self, int color) {\
    self->labelcolor(color);\
}\
int widget## _label_font(widget *self) {\
    return self->labelfont();\
}\
void widget## _set_label_font(widget *self, int font) {\
    self->labelfont(font);\
}\
int widget## _label_size(widget *self) {\
    return self->labelsize();\
}\
void widget## _set_label_size(widget *self, int sz) {\
    self->labelsize(sz);\
}\
int widget## _label_type(widget *self) {\
    return self->labelsize();\
}\
void widget## _set_label_type(widget *self, int sz) {\
    self->labelsize(sz);\
}\
int widget## _box(widget *self) {\
    return self->box();\
}\
void widget## _set_box(widget *self, int typ) {\
    self->box(static_cast<Fl_Boxtype>(typ));\
}\

#define GROUP_DEFINE(widget)\
void widget## _begin(widget *self) {\
    self->begin();\
}\
void widget## _end(widget *self) {\
    self->end();\
}\

#ifdef __cplusplus
}
#endif