#pragma once

#ifdef __cplusplus
extern "C" {
#endif

typedef struct Fl_Button Fl_Button;

Fl_Button *Fl_Button_new(int x, int y, int width, int height, const char *title);
void Fl_Button_set_label(Fl_Button *, const char *title);
void Fl_Button_redraw(Fl_Button *);
void Fl_Button_show(Fl_Button *);
void Fl_Button_hide(Fl_Button *);
void Fl_Button_activate(Fl_Button *);
void Fl_Button_deactivate(Fl_Button *); 
void Fl_Button_redraw_label(Fl_Button *);
void Fl_Button_resize(Fl_Button *, int x, int y, int width, int height);
const char* Fl_Button_tooltip(Fl_Button *);
void Fl_Button_set_tooltip(Fl_Button *, const char* txt);
int Fl_Button_get_type(Fl_Button *);
void Fl_Button_set_type(Fl_Button *, int typ);
int Fl_Button_color(Fl_Button *);
void Fl_Button_set_color(Fl_Button *, int color);
int Fl_Button_label_color(Fl_Button *);
void Fl_Button_set_label_color(Fl_Button *, int color);
int Fl_Button_label_font(Fl_Button *);
void Fl_Button_set_label_font(Fl_Button *, int font);
int Fl_Button_label_size(Fl_Button *);
void Fl_Button_set_label_size(Fl_Button *, int sz);
int Fl_Button_label_type(Fl_Button *);
void Fl_Button_set_label_type(Fl_Button *, int typ);
int Fl_Button_box(Fl_Button *);
void Fl_Button_set_box(Fl_Button *, int typ);


#ifdef __cplusplus
}
#endif