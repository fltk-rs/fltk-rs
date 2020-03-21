#include "cfl_widget.h"
#include <FL/Fl_Widget.H>
#include <FL/Fl_Image.H>

void Fl_Widget_callback(Fl_Widget *self, Fl_Callback *cb) {
  self->callback(cb);
}

void Fl_Widget_callback_with_captures(Fl_Widget *self, Fl_Callback *cb,
                                      void *data) {
  self->callback(cb, data);
}
