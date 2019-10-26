#pragma once

#ifdef __cplusplus
extern "C" {
#endif

typedef struct Fl_Widget Fl_Widget;
typedef void (cfl_callback)(Fl_Widget*, void*);

void Fl_Widget_callback(Fl_Widget*, cfl_callback* cb);

#ifdef __cplusplus
}
#endif