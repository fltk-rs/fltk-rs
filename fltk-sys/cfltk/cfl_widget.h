#pragma once

#ifdef __cplusplus
extern "C" {
#endif

#ifndef LOCK
#define LOCK(x)                                                                \
    Fl::lock();                                                                \
    x;                                                                         \
    Fl::unlock();                                                              \
    Fl::awake();
#endif

typedef struct Fl_Widget Fl_Widget;
typedef void(Fl_Callback)(Fl_Widget *, void *);
typedef int (*custom_handler_callback)(int, void *);
typedef void (*custom_draw_callback)(void *);
void Fl_Widget_callback_with_captures(Fl_Widget *, Fl_Callback *cb, void *);

#define WIDGET_DECLARE(widget)                                                 \
    typedef struct widget widget;                                              \
    widget *widget##_new(int x, int y, int width, int height,                  \
                         const char *title);                                   \
    int widget##_x(widget *);                                                  \
    int widget##_y(widget *);                                                  \
    int widget##_width(widget *);                                              \
    int widget##_height(widget *);                                             \
    const char *widget##_label(widget *);                                      \
    void widget##_set_label(widget *, const char *title);                      \
    void widget##_redraw(widget *);                                            \
    void widget##_show(widget *);                                              \
    void widget##_hide(widget *);                                              \
    void widget##_activate(widget *);                                          \
    void widget##_deactivate(widget *);                                        \
    void widget##_redraw_label(widget *);                                      \
    void widget##_resize(widget *, int x, int y, int width, int height);       \
    const char *widget##_tooltip(widget *);                                    \
    void widget##_set_tooltip(widget *, const char *txt);                      \
    int widget##_get_type(widget *);                                           \
    void widget##_set_type(widget *, int typ);                                 \
    unsigned int widget##_color(widget *);                                     \
    void widget##_set_color(widget *, unsigned int color);                     \
    unsigned int widget##_label_color(widget *);                               \
    void widget##_set_label_color(widget *, unsigned int color);               \
    int widget##_label_font(widget *);                                         \
    void widget##_set_label_font(widget *, int font);                          \
    int widget##_label_size(widget *);                                         \
    void widget##_set_label_size(widget *, int sz);                            \
    int widget##_label_type(widget *);                                         \
    void widget##_set_label_type(widget *, int typ);                           \
    int widget##_box(widget *);                                                \
    void widget##_set_box(widget *, int typ);                                  \
    int widget##_changed(widget *);                                            \
    void widget##_set_changed(widget *);                                       \
    void widget##_clear_changed(widget *);                                     \
    int widget##_align(widget *);                                              \
    void widget##_set_align(widget *, int typ);                                \
    void widget##_delete(widget *);                                            \
    void widget##_set_image(widget *, void *);                                 \
    void widget##_set_image_with_size(widget *, void *, int, int);             \
    void widget##_set_handler(widget *self, custom_handler_callback cb,        \
                              void *data);                                     \
    void widget##_set_draw(widget *self, custom_draw_callback cb, void *data); \
    void widget##_set_trigger(widget *, int);                                  \
    void *widget##_image(const widget *);                                      \
    void *widget##_parent(const widget *self);                                 \
    unsigned int widget##_selection_color(widget *);                           \
    void widget##_set_selection_color(widget *, unsigned int color);           \
    void widget##_do_callback(widget *);                                       \
    int widget##_inside(const widget *self, void *);                           \
    void *widget##_window(const widget *);                                     \
    void *widget##_top_window(const widget *);                                 \
    int widget##_takes_events(const widget *);

#define WIDGET_DEFINE(widget)                                                  \
    class widget##_Derived : public widget {                                   \
        void *ev_data_ = NULL;                                                 \
        void *draw_data_ = NULL;                                               \
                                                                               \
      public:                                                                  \
        typedef int (*handler)(int, void *data);                               \
        handler inner_handler = NULL;                                          \
        typedef void (*drawer)(void *data);                                    \
        drawer inner_drawer = NULL;                                            \
        widget##_Derived(int x, int y, int w, int h, const char *title = 0)    \
            : widget(x, y, w, h, title) {}                                     \
        operator widget *() { return (widget *)*this; }                        \
        void set_handler(handler h) { inner_handler = h; }                     \
        void set_handler_data(void *data) { ev_data_ = data; }                 \
        int handle(int event) override {                                       \
            int ret = widget::handle(event);                                   \
            int local = 0;                                                     \
            if (ev_data_ && inner_handler) {                                   \
                local = inner_handler(event, ev_data_);                        \
                if (local == 0)                                                \
                    return ret;                                                \
                else                                                           \
                    return local;                                              \
            } else {                                                           \
                return ret;                                                    \
            }                                                                  \
        }                                                                      \
        void set_drawer(drawer h) { inner_drawer = h; }                        \
        void set_drawer_data(void *data) { draw_data_ = data; }                \
        void draw() override {                                                 \
            widget::draw();                                                    \
                                                                               \
            if (draw_data_ && inner_drawer)                                    \
                inner_drawer(draw_data_);                                      \
        };                                                                     \
    };                                                                         \
    widget *widget##_new(int x, int y, int width, int height,                  \
                         const char *title) {                                  \
        return new (std::nothrow)                                              \
            widget##_Derived(x, y, width, height, title);                      \
    }                                                                          \
    int widget##_x(widget *self) { return self->x(); }                         \
    int widget##_y(widget *self) { return self->y(); }                         \
    int widget##_width(widget *self) { return self->w(); }                     \
    int widget##_height(widget *self) { return self->h(); }                    \
    const char *widget##_label(widget *self) { return self->label(); }         \
    void widget##_set_label(widget *self, const char *title) {                 \
        LOCK(self->copy_label(title);)                                         \
    }                                                                          \
    void widget##_redraw(widget *self) { LOCK(self->redraw();) }               \
    void widget##_show(widget *self) { LOCK(self->show();) }                   \
    void widget##_hide(widget *self) { LOCK(self->hide();) }                   \
    void widget##_activate(widget *self) { LOCK(self->activate();) }           \
    void widget##_deactivate(widget *self) { LOCK(self->deactivate();) }       \
    void widget##_redraw_label(widget *self) { LOCK(self->redraw_label();) }   \
    void widget##_resize(widget *self, int x, int y, int width, int height) {  \
        LOCK(self->resize(x, y, width, height);)                               \
    }                                                                          \
    const char *widget##_tooltip(widget *self) { return self->tooltip(); }     \
    void widget##_set_tooltip(widget *self, const char *txt) {                 \
        LOCK(self->copy_tooltip(txt);)                                         \
    }                                                                          \
    int widget##_get_type(widget *self) { return self->type(); }               \
    void widget##_set_type(widget *self, int typ) {                            \
        LOCK(auto val = self->type(); self->type((decltype(val))typ);)         \
    }                                                                          \
    unsigned int widget##_color(widget *self) { return self->color(); }        \
    void widget##_set_color(widget *self, unsigned int color) {                \
        LOCK(self->color(color);)                                              \
    }                                                                          \
    unsigned int widget##_label_color(widget *self) {                          \
        return self->labelcolor();                                             \
    }                                                                          \
    void widget##_set_label_color(widget *self, unsigned int color) {          \
        LOCK(self->labelcolor(color);)                                         \
    }                                                                          \
    int widget##_label_font(widget *self) { return self->labelfont(); }        \
    void widget##_set_label_font(widget *self, int font) {                     \
        LOCK(self->labelfont(font);)                                           \
    }                                                                          \
    int widget##_label_size(widget *self) { return self->labelsize(); }        \
    void widget##_set_label_size(widget *self, int sz) {                       \
        LOCK(self->labelsize(sz);)                                             \
    }                                                                          \
    int widget##_label_type(widget *self) { return self->labeltype(); }        \
    void widget##_set_label_type(widget *self, int typ) {                      \
        LOCK(self->labeltype(static_cast<Fl_Labeltype>(typ));)                 \
    }                                                                          \
    int widget##_box(widget *self) { return self->box(); }                     \
    void widget##_set_box(widget *self, int typ) {                             \
        LOCK(self->box(static_cast<Fl_Boxtype>(typ));)                         \
    }                                                                          \
    int widget##_changed(widget *self) { return self->changed(); }             \
    void widget##_set_changed(widget *self) { LOCK(self->set_changed();) }     \
    void widget##_clear_changed(widget *self) { LOCK(self->clear_changed();) } \
    int widget##_align(widget *self) { return self->align(); }                 \
    void widget##_set_align(widget *self, int typ) { LOCK(self->align(typ);) } \
    void widget##_delete(widget *self) { delete self; }                        \
    void widget##_set_image_with_size(widget *self, void *image, int w,        \
                                      int h) {                                 \
        LOCK(self->image(((Fl_Image *)image)->copy(w, h)); self->redraw();)    \
    }                                                                          \
    void widget##_set_image(widget *self, void *image) {                       \
        LOCK(self->image(((Fl_Image *)image)->copy()); self->redraw();)        \
    }                                                                          \
    void widget##_set_handler(widget *self, custom_handler_callback cb,        \
                              void *data) {                                    \
        LOCK(((widget##_Derived *)self)->set_handler_data(data);               \
             ((widget##_Derived *)self)->set_handler(cb);)                     \
    }                                                                          \
    void widget##_set_trigger(widget *self, int val) {                         \
        LOCK(self->when(val);)                                                 \
    }                                                                          \
    void *widget##_image(const widget *self) {                                 \
        return (Fl_Image *)self->image();                                      \
    }                                                                          \
    void widget##_set_draw(widget *self, custom_draw_callback cb,              \
                           void *data) {                                       \
        LOCK(((widget##_Derived *)self)->set_drawer_data(data);                \
             ((widget##_Derived *)self)->set_drawer(cb);)                      \
    }                                                                          \
    void *widget##_parent(const widget *self) {                                \
        return (Fl_Group *)self->parent();                                     \
    }                                                                          \
    unsigned int widget##_selection_color(widget *self) {                      \
        return self->selection_color();                                        \
    }                                                                          \
    void widget##_set_selection_color(widget *self, unsigned int color) {      \
        LOCK(self->selection_color(color);)                                    \
    }                                                                          \
    void widget##_do_callback(widget *self) {                                  \
        LOCK(((Fl_Widget *)self)->do_callback();)                              \
    }                                                                          \
    int widget##_inside(const widget *self, void *wid) {                       \
        return self->inside((Fl_Widget *)wid);                                 \
    }                                                                          \
    void *widget##_window(const widget *self) {                                \
        return (void *)self->window();                                         \
    }                                                                          \
    void *widget##_top_window(const widget *self) {                            \
        return (void *)self->top_window();                                     \
    }                                                                          \
    int widget##_takes_events(const widget *self) {                            \
        return self->takesevents();                                            \
    }

#ifdef __cplusplus
}
#endif