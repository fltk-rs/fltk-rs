#pragma once

#ifdef __cplusplus
extern "C" {
#endif

typedef struct Fl_Input Fl_Input;

Fl_Input *Fl_Input_new(int x, int y, int width, int height, const char *title);
void Fl_Input_set_label(Fl_Input *, const char *title);
void Fl_Input_redraw(Fl_Input *);
void Fl_Input_show(Fl_Input *);
void Fl_Input_hide(Fl_Input *);
void Fl_Input_activate(Fl_Input *);
void Fl_Input_deactivate(Fl_Input *); 
void Fl_Input_redraw_label(Fl_Input *);
void Fl_Input_resize(Fl_Input *, int x, int y, int width, int height);
const char* Fl_Input_tooltip(Fl_Input *);
void Fl_Input_set_tooltip(Fl_Input *, const char* txt);
int Fl_Input_get_type(Fl_Input *);
void Fl_Input_set_type(Fl_Input *, int typ);
int Fl_Input_color(Fl_Input *);
void Fl_Input_set_color(Fl_Input *, int color);
int Fl_Input_label_color(Fl_Input *);
void Fl_Input_set_label_color(Fl_Input *, int color);
int Fl_Input_label_font(Fl_Input *);
void Fl_Input_set_label_font(Fl_Input *, int font);
int Fl_Input_label_size(Fl_Input *);
void Fl_Input_set_label_size(Fl_Input *, int sz);
int Fl_Input_label_type(Fl_Input *);
void Fl_Input_set_label_type(Fl_Input *, int typ);
int Fl_Input_box(Fl_Input *);
void Fl_Input_set_box(Fl_Input *, int typ);


#ifdef __cplusplus
}
#endif