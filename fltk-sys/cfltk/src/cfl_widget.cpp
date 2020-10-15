#include "cfl_widget.h"
#include <FL/Fl.H>

#include "cfl_new.hpp"
#include <FL/Fl_Image.H>
#include <FL/Fl_Widget.H>

struct Fl_Widget_Derived : public Fl_Widget {
    void *ev_data_ = NULL;
    void *draw_data_ = NULL;

    typedef int (*handler)(int, void *data);
    handler inner_handler = NULL;
    typedef void (*drawer)(void *data);
    drawer inner_drawer = NULL;
    Fl_Widget_Derived(int x, int y, int w, int h, const char *title = 0)
        : Fl_Widget(x, y, w, h, title) {
    }

    operator Fl_Widget *() {
        return (Fl_Widget *)this;
    }

    void set_handler(handler h) {
        inner_handler = h;
    }

    void set_handler_data(void *data) {
        ev_data_ = data;
    }

    int handle(int event) override {
        int ret = Fl_Widget::handle(event);
        int local = 0;
        if (ev_data_ && inner_handler) {
            local = inner_handler(event, ev_data_);
            if (local == 0)
                return ret;
            else
                return local;
        } else {
            return ret;
        }
    }

    void set_drawer(drawer h) {
        inner_drawer = h;
    }

    void set_drawer_data(void *data) {
        draw_data_ = data;
    }

    void draw() override {
        if (draw_data_ && inner_drawer)
            inner_drawer(draw_data_);
    }
};

WIDGET_DEFINE(Fl_Widget)