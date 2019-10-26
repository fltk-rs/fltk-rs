#include <Fl/Fl_Widget.H>
#include "cfl_widget.h"

void Fl_Widget_callback(Fl_Widget *self, cfl_callback* cb) {
    self->callback(cb);
}