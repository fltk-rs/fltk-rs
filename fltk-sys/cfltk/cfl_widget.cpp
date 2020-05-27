#include "cfl_widget.h"
#include <FL/Fl.H>
#include <FL/Fl_Image.H>
#include <FL/Fl_Widget.H>
#include <new>

void Fl_Widget_callback(Fl_Widget *self, Fl_Callback *cb) {
    self->callback(cb);
}

void Fl_Widget_callback_with_captures(Fl_Widget *self, Fl_Callback *cb,
                                      void *data) {
    if (!cb || !data)
        return;
    LOCK(self->callback(cb, data);)
}

class Fl_Widget_Derived : public Fl_Widget {
    void *ev_data_ = NULL;
    void *draw_data_ = NULL;

  public:
    typedef int (*handler)(int, void *data);
    handler inner_handler = NULL;
    typedef void (*drawer)(void *data);
    drawer inner_drawer = NULL;
    Fl_Widget_Derived(int x, int y, int w, int h, const char *title = 0)
        : Fl_Widget(x, y, w, h, title) {}

    operator Fl_Widget *() { return (Fl_Widget *)this; }

    void set_handler(handler h) { inner_handler = h; }

    void set_handler_data(void *data) { ev_data_ = data; }

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

    void set_drawer(drawer h) { inner_drawer = h; }

    void set_drawer_data(void *data) { draw_data_ = data; }

    void draw() override {
        if (draw_data_ && inner_drawer)
            inner_drawer(draw_data_);
    };
    ~Fl_Widget_Derived() {
        void *user_data_ = Fl_Widget::user_data();
        free(user_data_);
        free(ev_data_);
        free(draw_data_);
    }
};
Fl_Widget *Fl_Widget_new(int x, int y, int width, int height,
                         const char *title) {
    return new (std::nothrow) Fl_Widget_Derived(x, y, width, height, title);
}

int Fl_Widget_x(Fl_Widget *self) { return self->x(); }

int Fl_Widget_y(Fl_Widget *self) { return self->y(); }

int Fl_Widget_width(Fl_Widget *self) { return self->w(); }

int Fl_Widget_height(Fl_Widget *self) { return self->h(); }

const char *Fl_Widget_label(Fl_Widget *self) { return self->label(); }

void Fl_Widget_set_label(Fl_Widget *self, const char *title) {
    LOCK(self->copy_label(title);)
}

void Fl_Widget_redraw(Fl_Widget *self) { LOCK(self->redraw();) }

void Fl_Widget_show(Fl_Widget *self) { LOCK(self->show();) }

void Fl_Widget_hide(Fl_Widget *self) { LOCK(self->hide();) }

void Fl_Widget_activate(Fl_Widget *self) { LOCK(self->activate();) }

void Fl_Widget_deactivate(Fl_Widget *self) { LOCK(self->deactivate();) }

void Fl_Widget_redraw_label(Fl_Widget *self) { LOCK(self->redraw_label();) }

void Fl_Widget_resize(Fl_Widget *self, int x, int y, int width, int height) {
    LOCK(self->resize(x, y, width, height);)
}

const char *Fl_Widget_tooltip(Fl_Widget *self) { return self->tooltip(); }

void Fl_Widget_set_tooltip(Fl_Widget *self, const char *txt) {
    LOCK(self->copy_tooltip(txt);)
}

int Fl_Widget_get_type(Fl_Widget *self) { return self->type(); }

void Fl_Widget_set_type(Fl_Widget *self, int typ) {
    LOCK(auto val = self->type(); self->type((decltype(val))typ);)
}

unsigned int Fl_Widget_color(Fl_Widget *self) { return self->color(); }

void Fl_Widget_set_color(Fl_Widget *self, unsigned int color) {
    LOCK(self->color(color);)
}

unsigned int Fl_Widget_label_color(Fl_Widget *self) {
    return self->labelcolor();
}

void Fl_Widget_set_label_color(Fl_Widget *self, unsigned int color) {
    LOCK(self->labelcolor(color);)
}

int Fl_Widget_label_font(Fl_Widget *self) { return self->labelfont(); }

void Fl_Widget_set_label_font(Fl_Widget *self, int font) {
    LOCK(self->labelfont(font);)
}

int Fl_Widget_label_size(Fl_Widget *self) { return self->labelsize(); }

void Fl_Widget_set_label_size(Fl_Widget *self, int sz) {
    LOCK(self->labelsize(sz);)
}

int Fl_Widget_label_type(Fl_Widget *self) { return self->labeltype(); }

void Fl_Widget_set_label_type(Fl_Widget *self, int typ) {
    LOCK(self->labeltype(static_cast<Fl_Labeltype>(typ));)
}

int Fl_Widget_box(Fl_Widget *self) { return self->box(); }

void Fl_Widget_set_box(Fl_Widget *self, int typ) {
    LOCK(self->box(static_cast<Fl_Boxtype>(typ));)
}

int Fl_Widget_changed(Fl_Widget *self) { return self->changed(); }

void Fl_Widget_set_changed(Fl_Widget *self) { LOCK(self->set_changed();) }

void Fl_Widget_clear_changed(Fl_Widget *self) { LOCK(self->clear_changed();) }

int Fl_Widget_align(Fl_Widget *self) { return self->align(); }

void Fl_Widget_set_align(Fl_Widget *self, int typ) { LOCK(self->align(typ);) }

void Fl_Widget_delete(Fl_Widget *self) { delete self; }

void Fl_Widget_set_image_with_size(Fl_Widget *self, void *image, int w, int h) {
    LOCK(self->image(((Fl_Image *)image)->copy(w, h)); self->redraw();)
}

void Fl_Widget_set_image(Fl_Widget *self, void *image) {
    LOCK(self->image(((Fl_Image *)image)->copy()); self->redraw();)
}

void Fl_Widget_set_handler(Fl_Widget *self, custom_handler_callback cb,
                           void *data) {
    LOCK(((Fl_Widget_Derived *)self)->set_handler_data(data);
         ((Fl_Widget_Derived *)self)->set_handler(cb);)
}

void Fl_Widget_set_trigger(Fl_Widget *self, int val) { LOCK(self->when(val);) }

void *Fl_Widget_image(const Fl_Widget *self) {
    return (Fl_Image *)self->image();
}

void Fl_Widget_set_draw(Fl_Widget *self, custom_draw_callback cb, void *data) {
    LOCK(((Fl_Widget_Derived *)self)->set_drawer_data(data);
         ((Fl_Widget_Derived *)self)->set_drawer(cb);)
}

void *Fl_Widget_parent(const Fl_Widget *self) {
    return (Fl_Group *)self->parent();
}

unsigned int Fl_Widget_selection_color(Fl_Widget *self) {
    return self->selection_color();
}

void Fl_Widget_set_selection_color(Fl_Widget *self, unsigned int color) {
    LOCK(self->selection_color(color);)
}

void Fl_Widget_do_callback(Fl_Widget *self) {
    LOCK(((Fl_Widget *)self)->do_callback();)
}

int Fl_Widget_inside(const Fl_Widget *self, void *wid) {
    return self->inside((Fl_Widget *)wid);
}

void *Fl_Widget_window(const Fl_Widget *self) { return (void *)self->window(); }

void *Fl_Widget_top_window(const Fl_Widget *self) {
    return (void *)self->top_window();
}

int Fl_Widget_takes_events(const Fl_Widget *self) {
    return self->takesevents();
}

void *Fl_Widget_user_data(const Fl_Widget *self) { return self->user_data(); }

