#include "cfl_widget.h"
#include <Fl/Fl_Widget.H>
#include <cstring>

void Fl_Widget_callback(Fl_Widget *self, Fl_Callback *cb) {
  self->callback(cb);
}

void Fl_Widget_callback_with_captures(Fl_Widget *self, Fl_Callback *cb,
                                      void *data) {
  self->callback(cb, data);
}