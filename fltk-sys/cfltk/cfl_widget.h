#pragma once

#ifdef __cplusplus
extern "C" {
#endif

typedef struct Fl_Widget Fl_Widget;
typedef void (Fl_Callback)(Fl_Widget* widget, void* data);
// void Fl_Widget_callback(Fl_Widget* widget, Fl_Callback* cb, void* data);

#ifdef __cplusplus
}
#endif