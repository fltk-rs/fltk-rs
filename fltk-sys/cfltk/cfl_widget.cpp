#include "cfl_widget.h"
#include <FL/Fl.H>
#include <FL/Fl_Image.H>
#include <FL/Fl_Widget.H>

void Fl_Widget_callback(Fl_Widget *self, Fl_Callback *cb) {
    self->callback(cb);
}

void Fl_Widget_callback_with_captures(Fl_Widget *self, Fl_Callback *cb,
                                      void *data) {
    if (!cb || !data)
        return;
    LOCK(self->callback(cb, data);)
}
