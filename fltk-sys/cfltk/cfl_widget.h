#pragma once

#ifdef __cplusplus
extern "C" {
#endif

typedef struct Fl_Widget Fl_Widget;
typedef void(Fl_Callback)(Fl_Widget *, void *);

void Fl_Widget_callback(Fl_Widget *, Fl_Callback *cb);
void Fl_Widget_callback_with_captures(Fl_Widget *, Fl_Callback *cb, void *);

#ifdef __cplusplus
}
#endif