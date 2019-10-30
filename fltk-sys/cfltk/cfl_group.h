#pragma once

#ifdef __cplusplus
extern "C" {
#endif

typedef struct Fl_Group Fl_Group;

Fl_Group *Fl_Group_new(int x, int y, int width, int height, const char *title);
void Fl_Group_begin(Fl_Group *self);
void Fl_Group_end(Fl_Group *self);
void Fl_Group_show(Fl_Group *self);
void Fl_Group_hide(Fl_Group *self);
void Fl_Group_set_label(Fl_Group *self, const char *title);
void Fl_Group_redraw(Fl_Group *self);
void Fl_Group_activate(Fl_Group *);
void Fl_Group_deactivate(Fl_Group *); 
void Fl_Group_redraw_label(Fl_Group *);
void Fl_Group_resize(Fl_Group *, int x, int y, int width, int height);
const char* Fl_Group_tooltip(Fl_Group *);
void Fl_Group_set_tooltip(Fl_Group *, const char* txt);
int Fl_Group_get_type(Fl_Group *);
void Fl_Group_set_type(Fl_Group *, int typ);
int Fl_Group_color(Fl_Group *);
void Fl_Group_set_color(Fl_Group *, int color);
int Fl_Group_label_color(Fl_Group *);
void Fl_Group_set_label_color(Fl_Group *, int color);
int Fl_Group_label_font(Fl_Group *);
void Fl_Group_set_label_font(Fl_Group *, int font);
int Fl_Group_label_size(Fl_Group *);
void Fl_Group_set_label_size(Fl_Group *, int sz);
int Fl_Group_label_type(Fl_Group *);
void Fl_Group_set_label_type(Fl_Group *, int typ);
int Fl_Group_box(Fl_Group *);
void Fl_Group_set_box(Fl_Group *, int typ);

#ifdef __cplusplus
}
#endif