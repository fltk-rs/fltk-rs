#include "cfl_menu.h"
#include <FL/Fl.H>
#include <FL/Fl_Choice.H>
#include <FL/Fl_Image.H>
#include <FL/Fl_Menu_Bar.H>
#include <FL/Fl_Menu_Button.H>
#include <FL/Fl_Menu_Item.H>
#include <new>

#define MENU_DEFINE(widget)                                                                        \
    void widget##_add(widget *self, const char *name, int shortcut, Fl_Callback *cb, void *data,   \
                      int flag) {                                                                  \
        if (!cb || !data)                                                                          \
            return;                                                                                \
        LOCK(self->add(name, shortcut, cb, data, flag);)                                           \
    }                                                                                              \
    void widget##_insert(widget *self, int index, const char *name, int shortcut, Fl_Callback *cb, \
                         void *data, int flag) {                                                   \
        if (!cb || !data)                                                                          \
            return;                                                                                \
        LOCK(self->insert(index, name, shortcut, cb, data, flag);)                                 \
    }                                                                                              \
    Fl_Menu_Item *widget##_get_item(widget *self, const char *name) {                              \
        return (Fl_Menu_Item *)self->find_item(name);                                              \
    }                                                                                              \
    int widget##_set_item(widget *self, Fl_Menu_Item *item) {                                      \
        int ret = 0;                                                                               \
        LOCK(ret = self->value(item);)                                                             \
        return ret;                                                                                \
    }                                                                                              \
    int widget##_text_font(widget *self) {                                                         \
        return self->textfont();                                                                   \
    }                                                                                              \
    void widget##_set_text_font(widget *self, int c) {                                             \
        LOCK(self->textfont(c);)                                                                   \
    }                                                                                              \
    int widget##_text_size(widget *self) {                                                         \
        return self->textsize();                                                                   \
    }                                                                                              \
    void widget##_set_text_size(widget *self, int c) {                                             \
        LOCK(self->textsize(c);)                                                                   \
    }                                                                                              \
    unsigned int widget##_text_color(widget *self) {                                               \
        return self->textcolor();                                                                  \
    }                                                                                              \
    void widget##_set_text_color(widget *self, unsigned int c) {                                   \
        LOCK(self->textcolor(c);)                                                                  \
    }                                                                                              \
    void widget##_add_choice(widget *self, const char *str) {                                      \
        LOCK(self->add(str);)                                                                      \
    }                                                                                              \
    const char *widget##_get_choice(widget *self) {                                                \
        return self->text();                                                                       \
    }                                                                                              \
    int widget##_value(widget *self) {                                                             \
        int ret = 0;                                                                               \
        LOCK(ret = self->value();)                                                                 \
        return ret;                                                                                \
    }                                                                                              \
    int widget##_set_value(widget *self, int v) {                                                  \
        int ret = 0;                                                                               \
        LOCK(ret = self->value(v);)                                                                \
        return ret;                                                                                \
    }                                                                                              \
    void widget##_clear(widget *self) {                                                            \
        LOCK(self->clear();)                                                                       \
    }                                                                                              \
    int widget##_clear_submenu(widget *self, int index) {                                          \
        int ret = 0;                                                                               \
        LOCK(ret = self->clear_submenu(index));                                                    \
        return ret;                                                                                \
    }                                                                                              \
    int widget##_size(const widget *self) {                                                        \
        return self->size();                                                                       \
    }                                                                                              \
    const char *widget##_text(const widget *self, int idx) {                                       \
        return self->text(idx);                                                                    \
    }                                                                                              \
    const Fl_Menu_Item *widget##_at(const widget *self, int idx) {                                 \
        return &self->menu()[idx];                                                                 \
    }                                                                                              \
    void widget##_set_mode(widget *self, int i, int fl) {                                          \
        LOCK(self->mode(fl);)                                                                      \
    }                                                                                              \
    int widget##_mode(const widget *self, int i) {                                                 \
        return self->mode(i);                                                                      \
    }                                                                                              \
    int widget##_find_index(const widget *self, const char *label) {                               \
        return self->find_index(label);                                                            \
    }                                                                                              \
    const Fl_Menu_Item *widget##_menu(const widget *self) {                                        \
        return self->menu();                                                                       \
    }                                                                                              \
    void widget##_remove(widget *self, int idx) {                                                  \
        LOCK(self->remove(idx);)                                                                   \
    }

WIDGET_DEFINE(Fl_Menu_Bar)

MENU_DEFINE(Fl_Menu_Bar)

WIDGET_DEFINE(Fl_Menu_Button)

MENU_DEFINE(Fl_Menu_Button)

WIDGET_DEFINE(Fl_Choice)

MENU_DEFINE(Fl_Choice)

Fl_Menu_Item *Fl_Menu_Item_new(char **args, int sz) {
    Fl_Menu_Item *items = new (std::nothrow) Fl_Menu_Item[sz + 1];
    if (!items)
        return NULL;
    for (int i = 0; i < sz; i++) {
        items[i] = {args[i]};
    }
    items[sz] = {NULL};
    return items;
}

void Fl_Menu_Item_delete(Fl_Menu_Item *self) {
    delete[] self;
}

const Fl_Menu_Item *Fl_Menu_Item_popup(Fl_Menu_Item *self, int x, int y) {
    return self->popup(x, y);
}

const char *Fl_Menu_Item_label(Fl_Menu_Item *self) {
    return self->label();
}

void Fl_Menu_Item_set_label(Fl_Menu_Item *self, const char *a) {
    LOCK(self->label(a);)
}

int Fl_Menu_Item_label_type(Fl_Menu_Item *self) {
    return self->labeltype();
}

void Fl_Menu_Item_set_label_type(Fl_Menu_Item *self, int a) {
    LOCK(self->labeltype(static_cast<Fl_Labeltype>(a));)
}

unsigned int Fl_Menu_Item_label_color(Fl_Menu_Item *self) {
    return self->labelcolor();
}

void Fl_Menu_Item_set_label_color(Fl_Menu_Item *self, unsigned int a) {
    LOCK(self->labelcolor(a);)
}

int Fl_Menu_Item_label_font(Fl_Menu_Item *self) {
    return self->labelfont();
}

void Fl_Menu_Item_set_label_font(Fl_Menu_Item *self, int a) {
    LOCK(self->labelfont(a);)
}

int Fl_Menu_Item_label_size(Fl_Menu_Item *self) {
    return self->labelsize();
}

void Fl_Menu_Item_set_label_size(Fl_Menu_Item *self, int a) {
    LOCK(self->labelsize(a);)
}

int Fl_Menu_Item_value(Fl_Menu_Item *self) {
    return self->value();
}

void Fl_Menu_Item_set(Fl_Menu_Item *self) {
    LOCK(self->set();)
}

void Fl_Menu_Item_clear(Fl_Menu_Item *self) {
    LOCK(self->clear();)
}

int Fl_Menu_Item_visible(Fl_Menu_Item *self) {
    return self->visible();
}

void Fl_Menu_Item_show(Fl_Menu_Item *self) {
    LOCK(self->show();)
}

void Fl_Menu_Item_hide(Fl_Menu_Item *self) {
    LOCK(self->hide();)
}

int Fl_Menu_Item_active(Fl_Menu_Item *self) {
    return self->active();
}

void Fl_Menu_Item_activate(Fl_Menu_Item *self) {
    LOCK(self->activate();)
}

void Fl_Menu_Item_deactivate(Fl_Menu_Item *self) {
    LOCK(self->deactivate();)
}

int Fl_Menu_Item_submenu(const Fl_Menu_Item *self) {
    return self->submenu();
}

Fl_Menu_Item *Fl_Menu_Item_next(Fl_Menu_Item *self, int idx) {
    return self->next(idx);
}

void Fl_Menu_Item_callback(Fl_Menu_Item *self, Fl_Callback *c, void *p) {
    LOCK(self->callback(c, p);)
}

void *Fl_Menu_Item_user_data(const Fl_Menu_Item *self) {
    return self->user_data();
}

void Fl_Menu_Item_set_user_data(Fl_Menu_Item *self, void *data) {
    self->user_data(data);
}