#pragma once

#ifdef __cplusplus
extern "C" {
#endif

typedef struct Fl_Box Fl_Box;

Fl_Box *Fl_Box_new(int x, int y, int width, int height, const char *title);
void Fl_Box_set_label(Fl_Box *, const char *title);
void Fl_Box_redraw(Fl_Box *);
void Fl_Box_show(Fl_Box *);
void Fl_Box_hide(Fl_Box *);
void Fl_Box_activate(Fl_Box *);
void Fl_Box_deactivate(Fl_Box *); 
void Fl_Box_redraw_label(Fl_Box *);
void Fl_Box_resize(Fl_Box *, int x, int y, int width, int height);
const char* Fl_Box_tooltip(Fl_Box *);
void Fl_Box_set_tooltip(Fl_Box *, const char* txt);
int Fl_Box_get_type(Fl_Box *);
void Fl_Box_set_type(Fl_Box *, int typ);
int Fl_Box_color(Fl_Box *);
void Fl_Box_set_color(Fl_Box *, int color);
int Fl_Box_label_color(Fl_Box *);
void Fl_Box_set_label_color(Fl_Box *, int color);
int Fl_Box_label_font(Fl_Box *);
void Fl_Box_set_label_font(Fl_Box *, int font);
int Fl_Box_label_size(Fl_Box *);
void Fl_Box_set_label_size(Fl_Box *, int sz);
int Fl_Box_label_type(Fl_Box *);
void Fl_Box_set_label_type(Fl_Box *, int typ);
int Fl_Box_box(Fl_Box *);
void Fl_Box_set_box(Fl_Box *, int typ);


#ifdef __cplusplus
}
#endif