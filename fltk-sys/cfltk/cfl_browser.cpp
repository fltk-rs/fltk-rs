#include "cfl_browser.h"
#include <FL/Fl.H>
#include <FL/Fl_Browser.H>
#include <FL/Fl_File_Browser.H>
#include <FL/Fl_Hold_Browser.H>
#include <FL/Fl_Image.H>
#include <FL/Fl_Multi_Browser.H>
#include <FL/Fl_Select_Browser.H>
#include <new>

#define BROWSER_DEFINE(widget)                                                                     \
    void widget##_remove(widget *self, int line) {                                                 \
        LOCK(self->remove(line);)                                                                  \
    }                                                                                              \
    void widget##_add(widget *self, const char *newtext) {                                         \
        LOCK(self->add(newtext);)                                                                  \
    }                                                                                              \
    void widget##_insert(widget *self, int line, const char *newtext) {                            \
        LOCK(self->insert(line, newtext);)                                                         \
    }                                                                                              \
    void widget##_move(widget *self, int to, int from) {                                           \
        LOCK(self->move(to, from);)                                                                \
    }                                                                                              \
    void widget##_swap(widget *self, int a, int b) {                                               \
        LOCK(self->swap(a, b);)                                                                    \
    }                                                                                              \
    void widget##_clear(widget *self) {                                                            \
        LOCK(self->clear();)                                                                       \
    }                                                                                              \
    int widget##_size(const widget *self) {                                                        \
        return self->size();                                                                       \
    }                                                                                              \
    void widget##_set_size(widget *self, int W, int H) {                                           \
        LOCK(self->size(W, H);)                                                                    \
    }                                                                                              \
    int widget##_select(widget *self, int line) {                                                  \
        int ret = 0;                                                                               \
        LOCK(ret = self->select(line));                                                            \
        return ret;                                                                                \
    }                                                                                              \
    int widget##_selected(const widget *self, int line) {                                          \
        return self->selected(line);                                                               \
    }                                                                                              \
    const char *widget##_text(const widget *self, int line) {                                      \
        return self->text(line);                                                                   \
    }                                                                                              \
    void widget##_set_text(widget *self, int line, const char *newtext) {                          \
        LOCK(self->text(line, newtext);)                                                           \
    }                                                                                              \
    void widget##_load_file(widget *self, const char *file) {                                      \
        LOCK(self->load(file);)                                                                    \
    }                                                                                              \
    int widget##_text_size(widget *self) {                                                         \
        return self->textsize();                                                                   \
    }                                                                                              \
    void widget##_set_text_size(widget *self, int s) {                                             \
        LOCK(self->textsize(s);)                                                                   \
    }                                                                                              \
    void widget##_set_icon(widget *self, int line, void *icon) {                                   \
        LOCK(self->icon(line, ((Fl_Image *)icon));)                                                \
    }                                                                                              \
    void *widget##_icon(const widget *self, int line) {                                            \
        return (Fl_Image *)self->icon(line);                                                       \
    }                                                                                              \
    void widget##_remove_icon(widget *self, int l) {                                               \
        LOCK(self->remove_icon(l);)                                                                \
    }                                                                                              \
    void widget##_topline(widget *self, int line) {                                                \
        LOCK(self->topline(line);)                                                                 \
    }                                                                                              \
    void widget##_bottomline(widget *self, int line) {                                             \
        LOCK(self->bottomline(line);)                                                              \
    }                                                                                              \
    void widget##_middleline(widget *self, int line) {                                             \
        LOCK(self->middleline(line);)                                                              \
    }                                                                                              \
    char widget##_format_char(const widget *self) {                                                \
        return self->format_char();                                                                \
    }                                                                                              \
    void widget##_set_format_char(widget *self, char c) {                                          \
        LOCK(self->format_char(c);)                                                                \
    }                                                                                              \
    char widget##_column_char(const widget *self) {                                                \
        return self->column_char();                                                                \
    }                                                                                              \
    void widget##_set_column_char(widget *self, char c) {                                          \
        LOCK(self->column_char(c);)                                                                \
    }                                                                                              \
    const int *widget##_column_widths(const widget *self) {                                        \
        return self->column_widths();                                                              \
    }                                                                                              \
    void widget##_set_column_widths(widget *self, const int *arr) {                                \
        LOCK(self->column_widths(arr);)                                                            \
    }                                                                                              \
    int widget##_displayed(const widget *self, int line) {                                         \
        return self->displayed(line);                                                              \
    }                                                                                              \
    void widget##_make_visible(widget *self, int line) {                                           \
        LOCK(self->make_visible(line);)                                                            \
    }                                                                                              \
    int widget##_position(const widget *self) {                                                    \
        return self->position();                                                                   \
    }                                                                                              \
    void widget##_set_position(widget *self, int pos) {                                            \
        LOCK(self->position(pos);)                                                                 \
    }                                                                                              \
    int widget##_hposition(const widget *self) {                                                   \
        return self->hposition();                                                                  \
    }                                                                                              \
    void widget##_set_hposition(widget *self, int pos) {                                           \
        LOCK(self->hposition(pos);)                                                                \
    }                                                                                              \
    unsigned char widget##_has_scrollbar(const widget *self) {                                     \
        return self->has_scrollbar();                                                              \
    }                                                                                              \
    void widget##_set_has_scrollbar(widget *self, unsigned char mode) {                            \
        LOCK(self->has_scrollbar(mode);)                                                           \
    }                                                                                              \
    int widget##_scrollbar_size(const widget *self) {                                              \
        return self->scrollbar_size();                                                             \
    }                                                                                              \
    void widget##_set_scrollbar_size(widget *self, int newSize) {                                  \
        LOCK(self->scrollbar_size(newSize);)                                                       \
    }                                                                                              \
    int widget##_scrollbar_width(const widget *self) {                                             \
        return self->scrollbar_width();                                                            \
    }                                                                                              \
    void widget##_set_scrollbar_width(widget *self, int width) {                                   \
        LOCK(self->scrollbar_width(width);)                                                        \
    }                                                                                              \
    void widget##_sort(widget *self) {                                                             \
        LOCK(self->sort();)                                                                        \
    }

WIDGET_DEFINE(Fl_Browser)

BROWSER_DEFINE(Fl_Browser)

WIDGET_DEFINE(Fl_Hold_Browser)

BROWSER_DEFINE(Fl_Hold_Browser)

WIDGET_DEFINE(Fl_Select_Browser)

BROWSER_DEFINE(Fl_Select_Browser)

WIDGET_DEFINE(Fl_Multi_Browser)

BROWSER_DEFINE(Fl_Multi_Browser)

WIDGET_DEFINE(Fl_File_Browser)

BROWSER_DEFINE(Fl_File_Browser)