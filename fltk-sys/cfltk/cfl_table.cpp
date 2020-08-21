#include "cfl_table.h"
#include "cfl_group.h"
#include <FL/Fl.H>

#include <FL/Fl_Image.H>
#include <FL/Fl_Table.H>
#include <FL/Fl_Table_Row.H>
 #include "cfl_new.hpp"

#define TABLE_DEFINE(table)                                                                        \
    void table##_set_table_box(table *self, int val) {                                             \
        LOCK(self->table_box((Fl_Boxtype)val);)                                                    \
    }                                                                                              \
    int table##_table_box(table *self) {                                                           \
        return self->table_box();                                                                  \
    }                                                                                              \
    void table##_set_rows(table *self, int val) {                                                  \
        LOCK(self->rows(val);)                                                                     \
    }                                                                                              \
    int table##_rows(table *self) {                                                                \
        return self->rows();                                                                       \
    }                                                                                              \
    void table##_set_cols(table *self, int val) {                                                  \
        LOCK(self->cols(val);)                                                                     \
    }                                                                                              \
    int table##_cols(table *self) {                                                                \
        return self->cols();                                                                       \
    }                                                                                              \
    void table##_visible_cells(table *self, int *r1, int *r2, int *c1, int *c2) {                  \
        LOCK(self->visible_cells(*r1, *r2, *c1, *c2);)                                             \
    }                                                                                              \
    int table##_is_interactive_resize(table *self) {                                               \
        return self->is_interactive_resize();                                                      \
    }                                                                                              \
    int table##_row_resize(table *self) {                                                          \
        return self->row_resize();                                                                 \
    }                                                                                              \
    void table##_set_row_resize(table *self, int flag) {                                           \
        LOCK(self->row_resize(flag);)                                                              \
    }                                                                                              \
    int table##_col_resize(table *self) {                                                          \
        return self->col_resize();                                                                 \
    }                                                                                              \
    void table##_set_col_resize(table *self, int flag) {                                           \
        LOCK(self->col_resize(flag);)                                                              \
    }                                                                                              \
    int table##_col_resize_min(table *self) {                                                      \
        return self->col_resize_min();                                                             \
    }                                                                                              \
    void table##_set_col_resize_min(table *self, int val) {                                        \
        LOCK(self->col_resize_min(val);)                                                           \
    }                                                                                              \
    int table##_row_resize_min(table *self) {                                                      \
        return self->row_resize_min();                                                             \
    }                                                                                              \
    void table##_set_row_resize_min(table *self, int val) {                                        \
        LOCK(self->row_resize_min(val);)                                                           \
    }                                                                                              \
    int table##_row_header(table *self) {                                                          \
        return self->row_header();                                                                 \
    }                                                                                              \
    void table##_set_row_header(table *self, int flag) {                                           \
        LOCK(self->row_header(flag);)                                                              \
    }                                                                                              \
    int table##_col_header(table *self) {                                                          \
        return self->col_header();                                                                 \
    }                                                                                              \
    void table##_set_col_header(table *self, int flag) {                                           \
        LOCK(self->col_header(flag);)                                                              \
    }                                                                                              \
    void table##_set_col_header_height(table *self, int height) {                                  \
        LOCK(self->col_header_height(height);)                                                     \
    }                                                                                              \
    int table##_col_header_height(table *self) {                                                   \
        return self->col_header_height();                                                          \
    }                                                                                              \
    void table##_set_row_header_width(table *self, int width) {                                    \
        LOCK(self->row_header_width(width);)                                                       \
    }                                                                                              \
    int table##_row_header_width(table *self) {                                                    \
        return self->row_header_width();                                                           \
    }                                                                                              \
    void table##_set_row_header_color(table *self, unsigned int val) {                             \
        LOCK(self->row_header_color(val);)                                                         \
    }                                                                                              \
    unsigned int table##_row_header_color(table *self) {                                           \
        return self->row_header_color();                                                           \
    }                                                                                              \
    void table##_set_col_header_color(table *self, unsigned int val) {                             \
        LOCK(self->col_header_color(val);)                                                         \
    }                                                                                              \
    unsigned int table##_col_header_color(table *self) {                                           \
        return self->col_header_color();                                                           \
    }                                                                                              \
    void table##_set_row_height(table *self, int row, int height) {                                \
        LOCK(self->row_height(row, height);)                                                       \
    }                                                                                              \
    int table##_row_height(table *self, int row) {                                                 \
        return self->row_height(row);                                                              \
    }                                                                                              \
    void table##_set_col_width(table *self, int col, int width) {                                  \
        LOCK(self->col_width(col, width);)                                                         \
    }                                                                                              \
    int table##_col_width(table *self, int col) {                                                  \
        return self->col_width(col);                                                               \
    }                                                                                              \
    void table##_set_row_height_all(table *self, int height) {                                     \
        LOCK(self->row_height_all(height);)                                                        \
    }                                                                                              \
    void table##_set_col_width_all(table *self, int width) {                                       \
        LOCK(self->col_width_all(width);)                                                          \
    }                                                                                              \
    void table##_set_row_position(table *self, int row) {                                          \
        LOCK(self->row_position(row);)                                                             \
    }                                                                                              \
    void table##_set_col_position(table *self, int col) {                                          \
        LOCK(self->col_position(col);)                                                             \
    }                                                                                              \
    int table##_row_position(table *self) {                                                        \
        return self->row_position();                                                               \
    }                                                                                              \
    int table##_col_position(table *self) {                                                        \
        return self->col_position();                                                               \
    }                                                                                              \
    void table##_set_top_row(table *self, int row) {                                               \
        LOCK(self->top_row(row);)                                                                  \
    }                                                                                              \
    int table##_top_row(table *self) {                                                             \
        return self->top_row();                                                                    \
    }                                                                                              \
    int table##_is_selected(table *self, int r, int c) {                                           \
        return self->is_selected(r, c);                                                            \
    }                                                                                              \
    void table##_get_selection(table *self, int *row_top, int *col_left, int *row_bot,             \
                               int *col_right) {                                                   \
        LOCK(self->get_selection(*row_top, *col_left, *row_bot, *col_right);)                      \
    }                                                                                              \
    void table##_set_selection(table *self, int row_top, int col_left, int row_bot,                \
                               int col_right) {                                                    \
        LOCK(self->set_selection(row_top, col_left, row_bot, col_right);)                          \
    }                                                                                              \
    int table##_move_cursor_with_shiftselect(table *self, int R, int C, int shiftselect) {         \
        return self->move_cursor(R, C, shiftselect);                                               \
    }                                                                                              \
    int table##_move_cursor(table *self, int R, int C) {                                           \
        return self->move_cursor(R, C);                                                            \
    }                                                                                              \
    void table##_init_sizes(table *self) {                                                         \
        LOCK(self->init_sizes();)                                                                  \
    }                                                                                              \
    int table##_scrollbar_size(const table *self) {                                                \
        return self->scrollbar_size();                                                             \
    }                                                                                              \
    void table##_set_scrollbar_size(table *self, int newSize) {                                    \
        LOCK(self->scrollbar_size(newSize);)                                                       \
    }                                                                                              \
    void table##_set_tab_cell_nav(table *self, int val) {                                          \
        LOCK(self->tab_cell_nav(val);)                                                             \
    }                                                                                              \
    int table##_tab_cell_nav(const table *self) {                                                  \
        return self->tab_cell_nav();                                                               \
    }                                                                                              \
    void table##_set_draw_cell(table *self, void (*cb)(int, int, int, int, int, int, int, void *), \
                               void *data) {                                                       \
        LOCK(((table##_Derived *)self)->set_cell_drawer_data(data);                                \
             ((table##_Derived *)self)->set_cell_drawer(cb);)                                      \
    }                                                                                              \
    void *table##_draw_cell_data(const table *self) {                                              \
        return ((table##_Derived *)self)->draw_cell_data_;                                         \
    }                                                                                              \
    void table##_set_draw_cell_data(table *self, void *data) {                                     \
        LOCK(((table##_Derived *)self)->draw_cell_data_ = data)                                    \
    }

struct Fl_Table_Derived : public Fl_Table {
    void *ev_data_ = NULL;
    void *draw_data_ = NULL;
    void *draw_cell_data_ = NULL;

    typedef int (*handler)(int, void *data);
    handler inner_handler = NULL;
    typedef void (*drawer)(void *data);
    typedef void (*cell_drawer)(int, int, int, int, int, int, int, void *data);
    drawer inner_drawer = NULL;
    cell_drawer inner_cell_drawer = NULL;
    Fl_Table_Derived(int x, int y, int w, int h, const char *title = 0)
        : Fl_Table(x, y, w, h, title) {
    }

    operator Fl_Table *() {
        return (Fl_Table *)this;
    }

    void set_handler(handler h) {
        inner_handler = h;
    }

    void set_handler_data(void *data) {
        ev_data_ = data;
    }

    int handle(int event) override {
        int ret = Fl_Table::handle(event);
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

    void set_cell_drawer(cell_drawer h) {
        inner_cell_drawer = h;
    }

    void set_cell_drawer_data(void *data) {
        draw_cell_data_ = data;
    }

    void draw() override {
        Fl_Table::draw();

        if (draw_data_ && inner_drawer)
            inner_drawer(draw_data_);
    }

    void draw_cell(TableContext context, int R, int C, int X, int Y, int W, int H) override {
        Fl_Table::draw_cell(context, R, C, X, Y, W, H);

        if (draw_cell_data_ && inner_cell_drawer)
            inner_cell_drawer(context, R, C, X, Y, W, H, draw_cell_data_);
    }
};
Fl_Table *Fl_Table_new(int x, int y, int width, int height, const char *title) {
    return new Fl_Table_Derived(x, y, width, height, title);
}

int Fl_Table_x(Fl_Table *self) {
    return self->x();
}

int Fl_Table_y(Fl_Table *self) {
    return self->y();
}

int Fl_Table_width(Fl_Table *self) {
    return self->w();
}

int Fl_Table_height(Fl_Table *self) {
    return self->h();
}

const char *Fl_Table_label(Fl_Table *self) {
    return self->label();
}

void Fl_Table_set_label(Fl_Table *self, const char *title) {
    LOCK(self->copy_label(title);)
}

void Fl_Table_redraw(Fl_Table *self) {
    LOCK(self->redraw();)
}

void Fl_Table_show(Fl_Table *self) {
    LOCK(self->show();)
}

void Fl_Table_hide(Fl_Table *self) {
    LOCK(self->hide();)
}

void Fl_Table_activate(Fl_Table *self) {
    LOCK(self->activate();)
}

void Fl_Table_deactivate(Fl_Table *self) {
    LOCK(self->deactivate();)
}

void Fl_Table_redraw_label(Fl_Table *self) {
    LOCK(self->redraw_label();)
}

void Fl_Table_resize(Fl_Table *self, int x, int y, int width, int height) {
    LOCK(self->resize(x, y, width, height);)
}

const char *Fl_Table_tooltip(Fl_Table *self) {
    return self->tooltip();
}

void Fl_Table_set_tooltip(Fl_Table *self, const char *txt) {
    LOCK(self->copy_tooltip(txt);)
}

int Fl_Table_get_type(Fl_Table *self) {
    return self->type();
}

void Fl_Table_set_type(Fl_Table *self, int typ) {
    LOCK(auto val = self->type(); self->type((decltype(val))typ);)
}

unsigned int Fl_Table_color(Fl_Table *self) {
    return self->color();
}

void Fl_Table_set_color(Fl_Table *self, unsigned int color) {
    LOCK(self->color(color);)
}

unsigned int Fl_Table_label_color(Fl_Table *self) {
    return self->labelcolor();
}

void Fl_Table_set_label_color(Fl_Table *self, unsigned int color) {
    LOCK(self->labelcolor(color);)
}

int Fl_Table_label_font(Fl_Table *self) {
    return self->labelfont();
}

void Fl_Table_set_label_font(Fl_Table *self, int font) {
    LOCK(self->labelfont(font);)
}

int Fl_Table_label_size(Fl_Table *self) {
    return self->labelsize();
}

void Fl_Table_set_label_size(Fl_Table *self, int sz) {
    LOCK(self->labelsize(sz);)
}

int Fl_Table_label_type(Fl_Table *self) {
    return self->labeltype();
}

void Fl_Table_set_label_type(Fl_Table *self, int typ) {
    LOCK(self->labeltype(static_cast<Fl_Labeltype>(typ));)
}

int Fl_Table_box(Fl_Table *self) {
    return self->box();
}

void Fl_Table_set_box(Fl_Table *self, int typ) {
    LOCK(self->box(static_cast<Fl_Boxtype>(typ));)
}

int Fl_Table_changed(Fl_Table *self) {
    return self->changed();
}

void Fl_Table_set_changed(Fl_Table *self) {
    LOCK(self->set_changed();)
}

void Fl_Table_clear_changed(Fl_Table *self) {
    LOCK(self->clear_changed();)
}

int Fl_Table_align(Fl_Table *self) {
    return self->align();
}

void Fl_Table_set_align(Fl_Table *self, int typ) {
    LOCK(self->align(typ);)
}

void Fl_Table_delete(Fl_Table *self) {
    delete self;
}

void Fl_Table_set_image(Fl_Table *self, void *image) {
    LOCK(self->image(((Fl_Image *)image)))
}

void Fl_Table_set_handler(Fl_Table *self, custom_handler_callback cb, void *data) {
    LOCK(((Fl_Table_Derived *)self)->set_handler_data(data);
         ((Fl_Table_Derived *)self)->set_handler(cb);)
}

void Fl_Table_set_trigger(Fl_Table *self, int val) {
    LOCK(self->when(val);)
}

void *Fl_Table_image(const Fl_Table *self) {
    return (Fl_Image *)self->image();
}

void Fl_Table_set_draw(Fl_Table *self, custom_draw_callback cb, void *data) {
    LOCK(((Fl_Table_Derived *)self)->set_drawer_data(data);
         ((Fl_Table_Derived *)self)->set_drawer(cb);)
}

void *Fl_Table_parent(const Fl_Table *self) {
    return (Fl_Group *)self->parent();
}

unsigned int Fl_Table_selection_color(Fl_Table *self) {
    return self->selection_color();
}

void Fl_Table_set_selection_color(Fl_Table *self, unsigned int color) {
    LOCK(self->selection_color(color);)
}

void Fl_Table_do_callback(Fl_Table *self) {
    LOCK(((Fl_Widget *)self)->do_callback();)
}

int Fl_Table_inside(const Fl_Table *self, void *wid) {
    return self->inside((Fl_Table *)wid);
}

void *Fl_Table_window(const Fl_Table *self) {
    return (void *)self->window();
}

void *Fl_Table_top_window(const Fl_Table *self) {
    return (void *)self->top_window();
}

int Fl_Table_takes_events(const Fl_Table *self) {
    return self->takesevents();
}

void *Fl_Table_user_data(const Fl_Table *self) {
    return self->user_data();
}

int Fl_Table_take_focus(Fl_Table *self) {
    int ret = 0;
    LOCK(ret = self->take_focus());
    return ret;
}

void Fl_Table_set_visible_focus(Fl_Table *self) {
    LOCK(self->set_visible_focus();)
}

void Fl_Table_clear_visible_focus(Fl_Table *self) {
    LOCK(self->clear_visible_focus();)
}

void Fl_Table_visible_focus(Fl_Table *self, int v) {
    LOCK(self->visible_focus(v);)
}

unsigned int Fl_Table_has_visible_focus(Fl_Table *self) {
    return self->visible_focus();
}

void Fl_Table_set_user_data(Fl_Table *self, void *data) {
    LOCK(self->user_data(data);)
}

void *Fl_Table_draw_data(const Fl_Table *self) {
    return ((Fl_Table_Derived *)self)->draw_data_;
}

void *Fl_Table_handle_data(const Fl_Table *self) {
    return ((Fl_Table_Derived *)self)->ev_data_;
}

void Fl_Table_set_draw_data(Fl_Table *self, void *data) {
    LOCK(((Fl_Table_Derived *)self)->draw_data_ = data;)
}

void Fl_Table_set_handle_data(Fl_Table *self, void *data) {
    LOCK(((Fl_Table_Derived *)self)->ev_data_ = data;)
}

unsigned char Fl_Table_damage(const Fl_Table *self) {
    return self->damage();
}

void Fl_Table_set_damage(Fl_Table *self, unsigned char flag) {
    LOCK(self->damage(flag);)
}

void Fl_Table_clear_damage(Fl_Table *self) {
    LOCK(self->clear_damage();)
}

void *Fl_Table_as_window(Fl_Table *self) {
    return self->as_window();
}

void *Fl_Table_as_group(Fl_Table *self) {
    return self->as_group();
}

void Fl_Table_set_deimage(Fl_Table *self, void *image) {
    LOCK(self->deimage(((Fl_Image *)image)))
}

void *Fl_Table_deimage(const Fl_Table *self) {
    return (Fl_Image *)self->deimage();
}

GROUP_DEFINE(Fl_Table)

TABLE_DEFINE(Fl_Table)

struct Fl_Table_Row_Derived : public Fl_Table_Row {
    void *ev_data_ = NULL;
    void *draw_data_ = NULL;
    void *draw_cell_data_ = NULL;

    typedef int (*handler)(int, void *data);
    handler inner_handler = NULL;
    typedef void (*drawer)(void *data);
    typedef void (*cell_drawer)(int, int, int, int, int, int, int, void *data);
    drawer inner_drawer = NULL;
    cell_drawer inner_cell_drawer = NULL;
    Fl_Table_Row_Derived(int x, int y, int w, int h, const char *title = 0)
        : Fl_Table_Row(x, y, w, h, title) {
    }

    operator Fl_Table_Row *() {
        return (Fl_Table_Row *)this;
    }

    void set_handler(handler h) {
        inner_handler = h;
    }

    void set_handler_data(void *data) {
        ev_data_ = data;
    }

    int handle(int event) override {
        int ret = Fl_Table_Row::handle(event);
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

    void set_cell_drawer(cell_drawer h) {
        inner_cell_drawer = h;
    }

    void set_cell_drawer_data(void *data) {
        draw_cell_data_ = data;
    }

    void draw() override {
        Fl_Table_Row::draw();

        if (draw_data_ && inner_drawer)
            inner_drawer(draw_data_);
    }

    void draw_cell(TableContext context, int R, int C, int X, int Y, int W, int H) override {
        Fl_Table::draw_cell(context, R, C, X, Y, W, H);

        if (draw_cell_data_ && inner_cell_drawer)
            inner_cell_drawer(context, R, C, X, Y, W, H, draw_cell_data_);
    }
};
Fl_Table_Row *Fl_Table_Row_new(int x, int y, int width, int height, const char *title) {
    return new Fl_Table_Row_Derived(x, y, width, height, title);
}

int Fl_Table_Row_x(Fl_Table_Row *self) {
    return self->x();
}

int Fl_Table_Row_y(Fl_Table_Row *self) {
    return self->y();
}

int Fl_Table_Row_width(Fl_Table_Row *self) {
    return self->w();
}

int Fl_Table_Row_height(Fl_Table_Row *self) {
    return self->h();
}

const char *Fl_Table_Row_label(Fl_Table_Row *self) {
    return self->label();
}

void Fl_Table_Row_set_label(Fl_Table_Row *self, const char *title) {
    LOCK(self->copy_label(title);)
}

void Fl_Table_Row_redraw(Fl_Table_Row *self) {
    LOCK(self->redraw();)
}

void Fl_Table_Row_show(Fl_Table_Row *self) {
    LOCK(self->show();)
}

void Fl_Table_Row_hide(Fl_Table_Row *self) {
    LOCK(self->hide();)
}

void Fl_Table_Row_activate(Fl_Table_Row *self) {
    LOCK(self->activate();)
}

void Fl_Table_Row_deactivate(Fl_Table_Row *self) {
    LOCK(self->deactivate();)
}

void Fl_Table_Row_redraw_label(Fl_Table_Row *self) {
    LOCK(self->redraw_label();)
}

void Fl_Table_Row_resize(Fl_Table_Row *self, int x, int y, int width, int height) {
    LOCK(self->resize(x, y, width, height);)
}

const char *Fl_Table_Row_tooltip(Fl_Table_Row *self) {
    return self->tooltip();
}

void Fl_Table_Row_set_tooltip(Fl_Table_Row *self, const char *txt) {
    LOCK(self->copy_tooltip(txt);)
}

int Fl_Table_Row_get_type(Fl_Table_Row *self) {
    return self->type();
}

void Fl_Table_Row_set_type(Fl_Table_Row *self, int typ) {
    LOCK(auto val = self->type(); self->type((decltype(val))typ);)
}

unsigned int Fl_Table_Row_color(Fl_Table_Row *self) {
    return self->color();
}

void Fl_Table_Row_set_color(Fl_Table_Row *self, unsigned int color) {
    LOCK(self->color(color);)
}

unsigned int Fl_Table_Row_label_color(Fl_Table_Row *self) {
    return self->labelcolor();
}

void Fl_Table_Row_set_label_color(Fl_Table_Row *self, unsigned int color) {
    LOCK(self->labelcolor(color);)
}

int Fl_Table_Row_label_font(Fl_Table_Row *self) {
    return self->labelfont();
}

void Fl_Table_Row_set_label_font(Fl_Table_Row *self, int font) {
    LOCK(self->labelfont(font);)
}

int Fl_Table_Row_label_size(Fl_Table_Row *self) {
    return self->labelsize();
}

void Fl_Table_Row_set_label_size(Fl_Table_Row *self, int sz) {
    LOCK(self->labelsize(sz);)
}

int Fl_Table_Row_label_type(Fl_Table_Row *self) {
    return self->labeltype();
}

void Fl_Table_Row_set_label_type(Fl_Table_Row *self, int typ) {
    LOCK(self->labeltype(static_cast<Fl_Labeltype>(typ));)
}

int Fl_Table_Row_box(Fl_Table_Row *self) {
    return self->box();
}

void Fl_Table_Row_set_box(Fl_Table_Row *self, int typ) {
    LOCK(self->box(static_cast<Fl_Boxtype>(typ));)
}

int Fl_Table_Row_changed(Fl_Table_Row *self) {
    return self->changed();
}

void Fl_Table_Row_set_changed(Fl_Table_Row *self) {
    LOCK(self->set_changed();)
}

void Fl_Table_Row_clear_changed(Fl_Table_Row *self) {
    LOCK(self->clear_changed();)
}

int Fl_Table_Row_align(Fl_Table_Row *self) {
    return self->align();
}

void Fl_Table_Row_set_align(Fl_Table_Row *self, int typ) {
    LOCK(self->align(typ);)
}

void Fl_Table_Row_delete(Fl_Table_Row *self) {
    delete self;
}

void Fl_Table_Row_set_image(Fl_Table_Row *self, void *image) {
    LOCK(self->image(((Fl_Image *)image)))
}

void Fl_Table_Row_set_handler(Fl_Table_Row *self, custom_handler_callback cb, void *data) {
    LOCK(((Fl_Table_Row_Derived *)self)->set_handler_data(data);
         ((Fl_Table_Row_Derived *)self)->set_handler(cb);)
}

void Fl_Table_Row_set_trigger(Fl_Table_Row *self, int val) {
    LOCK(self->when(val);)
}

void *Fl_Table_Row_image(const Fl_Table_Row *self) {
    return (Fl_Image *)self->image();
}

void Fl_Table_Row_set_draw(Fl_Table_Row *self, custom_draw_callback cb, void *data) {
    LOCK(((Fl_Table_Row_Derived *)self)->set_drawer_data(data);
         ((Fl_Table_Row_Derived *)self)->set_drawer(cb);)
}

void *Fl_Table_Row_parent(const Fl_Table_Row *self) {
    return (Fl_Group *)self->parent();
}

unsigned int Fl_Table_Row_selection_color(Fl_Table_Row *self) {
    return self->selection_color();
}

void Fl_Table_Row_set_selection_color(Fl_Table_Row *self, unsigned int color) {
    LOCK(self->selection_color(color);)
}

void Fl_Table_Row_do_callback(Fl_Table_Row *self) {
    LOCK(((Fl_Widget *)self)->do_callback();)
}

int Fl_Table_Row_inside(const Fl_Table_Row *self, void *wid) {
    return self->inside((Fl_Table_Row *)wid);
}

void *Fl_Table_Row_window(const Fl_Table_Row *self) {
    return (void *)self->window();
}

void *Fl_Table_Row_top_window(const Fl_Table_Row *self) {
    return (void *)self->top_window();
}

int Fl_Table_Row_takes_events(const Fl_Table_Row *self) {
    return self->takesevents();
}

void *Fl_Table_Row_user_data(const Fl_Table_Row *self) {
    return self->user_data();
}

int Fl_Table_Row_take_focus(Fl_Table_Row *self) {
    int ret = 0;
    LOCK(ret = self->take_focus());
    return ret;
}

void Fl_Table_Row_set_visible_focus(Fl_Table_Row *self) {
    LOCK(self->set_visible_focus();)
}

void Fl_Table_Row_clear_visible_focus(Fl_Table_Row *self) {
    LOCK(self->clear_visible_focus();)
}

void Fl_Table_Row_visible_focus(Fl_Table_Row *self, int v) {
    LOCK(self->visible_focus(v);)
}

unsigned int Fl_Table_Row_has_visible_focus(Fl_Table_Row *self) {
    return self->visible_focus();
}

void Fl_Table_Row_set_user_data(Fl_Table_Row *self, void *data) {
    LOCK(self->user_data(data);)
}

void *Fl_Table_Row_draw_data(const Fl_Table_Row *self) {
    return ((Fl_Table_Row_Derived *)self)->draw_data_;
}

void *Fl_Table_Row_handle_data(const Fl_Table_Row *self) {
    return ((Fl_Table_Row_Derived *)self)->ev_data_;
}

void Fl_Table_Row_set_draw_data(Fl_Table_Row *self, void *data) {
    LOCK(((Fl_Table_Row_Derived *)self)->draw_data_ = data;)
}

void Fl_Table_Row_set_handle_data(Fl_Table_Row *self, void *data) {
    LOCK(((Fl_Table_Row_Derived *)self)->ev_data_ = data;)
}

unsigned char Fl_Table_Row_damage(const Fl_Table_Row *self) {
    return self->damage();
}

void Fl_Table_Row_set_damage(Fl_Table_Row *self, unsigned char flag) {
    LOCK(self->damage(flag);)
}

void Fl_Table_Row_clear_damage(Fl_Table_Row *self) {
    LOCK(self->clear_damage();)
}

void *Fl_Table_Row_as_window(Fl_Table_Row *self) {
    return self->as_window();
}

void *Fl_Table_Row_as_group(Fl_Table_Row *self) {
    return self->as_group();
}

void Fl_Table_Row_set_deimage(Fl_Table_Row *self, void *image) {
    LOCK(self->deimage(((Fl_Image *)image)))
}

void *Fl_Table_Row_deimage(const Fl_Table_Row *self) {
    return (Fl_Image *)self->deimage();
}

GROUP_DEFINE(Fl_Table_Row)

TABLE_DEFINE(Fl_Table_Row)

int Fl_Table_Row_row_selected(Fl_Table_Row *self, int row) {
    return self->row_selected(row);
}

int Fl_Table_Row_select_row(Fl_Table_Row *self, int row) {
    int ret = 0;
    LOCK(ret = self->select_row(row);)
    return ret;
}

void Fl_Table_Row_select_all_rows(Fl_Table_Row *self) {
    LOCK(self->select_all_rows();)
}