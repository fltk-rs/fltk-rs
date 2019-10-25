#include <FL/Fl_Widget.H>
#include "cfl_widget.h"

void Fl_Widget_callback(Fl_Widget* widget, Fl_Callback* cb, void* data) {
    cb(widget, data);
}