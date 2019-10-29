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
void Fl_Group_set_tooltip(Fl_Group *, const char* txt);

#ifdef __cplusplus
}
#endif