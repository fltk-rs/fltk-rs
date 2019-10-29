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
void Fl_Button_set_tooltip(Fl_Button *, const char* txt);
void Fl_Button_set_type(Fl_Button *, int typ);


#ifdef __cplusplus
}
#endif