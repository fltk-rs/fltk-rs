#pragma once

#include "cfl_widget.h"

#ifdef __cplusplus
extern "C" {
#endif

#define BROWSER_DECLARE(widget)                                                                    \
    void widget##_remove(widget *, int line);                                                      \
    void widget##_add(widget *, const char *newtext);                                              \
    void widget##_insert(widget *, int line, const char *newtext);                                 \
    void widget##_move(widget *, int to, int from);                                                \
    void widget##_swap(widget *, int a, int b);                                                    \
    void widget##_clear(widget *);                                                                 \
    int widget##_size(const widget *);                                                             \
    void widget##_set_size(widget *, int W, int H);                                                \
    int widget##_select(widget *, int line);                                                       \
    int widget##_selected(const widget *, int line);                                               \
    const char *widget##_text(const widget *, int line);                                           \
    void widget##_set_text(widget *, int line, const char *newtext);                               \
    void widget##_load_file(widget *, const char *file);                                           \
    int widget##_text_size(widget *);                                                              \
    void widget##_set_text_size(widget *, int s);                                                  \
    void widget##_set_icon(widget *, int line, void *icon);                                        \
    void *widget##_icon(const widget *, int line);                                                 \
    void widget##_remove_icon(widget *, int line);                                                 \
    void widget##_topline(widget *self, int line);                                                 \
    void widget##_bottomline(widget *self, int line);                                              \
    void widget##_middleline(widget *self, int line);                                              \
    char widget##_format_char(const widget *self);                                                 \
    void widget##_set_format_char(widget *self, char c);                                           \
    char widget##_column_char(const widget *self);                                                 \
    void widget##_set_column_char(widget *self, char c);                                           \
    const int *widget##_column_widths(const widget *self);                                         \
    void widget##_set_column_widths(widget *self, const int *arr);                                 \
    int widget##_displayed(const widget *self, int line);                                          \
    void widget##_make_visible(widget *self, int line);                                            \
    int widget##_position(const widget *self);                                                     \
    void widget##_set_position(widget *self, int pos);                                             \
    int widget##_hposition(const widget *self);                                                    \
    void widget##_set_hposition(widget *self, int);                                                \
    unsigned char widget##_has_scrollbar(const widget *self);                                      \
    void widget##_set_has_scrollbar(widget *self, unsigned char mode);                             \
    int widget##_scrollbar_size(const widget *self);                                               \
    void widget##_set_scrollbar_size(widget *self, int newSize);                                   \
    int widget##_scrollbar_width(const widget *self);                                              \
    void widget##_set_scrollbar_width(widget *self, int width);                                    \
    void widget##_sort(widget *self);                                                              \
    void *widget##_scrollbar(widget *self);                                                        \
    void *widget##_hscrollbar(widget *self);

WIDGET_DECLARE(Fl_Browser)

BROWSER_DECLARE(Fl_Browser)

WIDGET_DECLARE(Fl_Hold_Browser)

BROWSER_DECLARE(Fl_Hold_Browser)

WIDGET_DECLARE(Fl_Select_Browser)

BROWSER_DECLARE(Fl_Select_Browser)

WIDGET_DECLARE(Fl_Multi_Browser)

BROWSER_DECLARE(Fl_Multi_Browser)

WIDGET_DECLARE(Fl_File_Browser)

unsigned Fl_File_Browser_iconsize(const Fl_File_Browser *self);

void Fl_File_Browser_set_iconsize(Fl_File_Browser *self, unsigned s);

void Fl_File_Browser_set_filter(Fl_File_Browser *self, const char *pattern);

const char *Fl_File_Browser_filter(const Fl_File_Browser *self);

int Fl_File_Browser_filetype(const Fl_File_Browser *self);

void Fl_File_Browser_set_filetype(Fl_File_Browser *self, int t);

BROWSER_DECLARE(Fl_File_Browser)

#ifdef __cplusplus
}
#endif
