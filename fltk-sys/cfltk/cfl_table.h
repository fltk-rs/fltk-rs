#pragma once

#include "cfl_group.h"

#ifdef __cplusplus
extern "C" {
#endif

#define TABLE_DECLARE(table)                                                                       \
    typedef struct table table;                                                                    \
    void table##_set_table_box(table *self, int val);                                              \
    int table##_table_box(table *self);                                                            \
    void table##_set_rows(table *self, int val);                                                   \
    int table##_rows(table *self);                                                                 \
    void table##_set_cols(table *self, int val);                                                   \
    int table##_cols(table *self);                                                                 \
    void table##_visible_cells(table *self, int *r1, int *r2, int *c1, int *c2);                   \
    int table##_is_interactive_resize(table *self);                                                \
    int table##_row_resize(table *self);                                                           \
    void table##_set_row_resize(table *self, int flag);                                            \
    int table##_col_resize(table *self);                                                           \
    void table##_set_col_resize(table *self, int flag);                                            \
    int table##_col_resize_min(table *self);                                                       \
    void table##_set_col_resize_min(table *self, int val);                                         \
    int table##_row_resize_min(table *self);                                                       \
    void table##_set_row_resize_min(table *self, int val);                                         \
    int table##_row_header(table *self);                                                           \
    void table##_set_row_header(table *self, int flag);                                            \
    int table##_col_header(table *self);                                                           \
    void table##_set_col_header(table *self, int flag);                                            \
    void table##_set_col_header_height(table *self, int height);                                   \
    int table##_col_header_height(table *self);                                                    \
    void table##_set_row_header_width(table *self, int width);                                     \
    int table##_row_header_width(table *self);                                                     \
    void table##_set_row_header_color(table *self, unsigned int val);                              \
    unsigned int table##_row_header_color(table *self);                                            \
    void table##_set_col_header_color(table *self, unsigned int val);                              \
    unsigned int table##_col_header_color(table *self);                                            \
    void table##_set_row_height(table *self, int row, int height);                                 \
    int table##_row_height(table *self, int row);                                                  \
    void table##_set_col_width(table *self, int col, int width);                                   \
    int table##_col_width(table *self, int col);                                                   \
    void table##_set_row_height_all(table *self, int height);                                      \
    void table##_set_col_width_all(table *self, int width);                                        \
    void table##_set_row_position(table *self, int row);                                           \
    void table##_set_col_position(table *self, int col);                                           \
    int table##_row_position(table *self);                                                         \
    int table##_col_position(table *self);                                                         \
    void table##_set_top_row(table *self, int row);                                                \
    int table##_top_row(table *self);                                                              \
    int table##_is_selected(table *self, int r, int c);                                            \
    void table##_get_selection(table *self, int *row_top, int *col_left, int *row_bot,             \
                               int *col_right);                                                    \
    void table##_set_selection(table *self, int row_top, int col_left, int row_bot,                \
                               int col_right);                                                     \
    int table##_move_cursor_with_shiftselect(table *self, int R, int C, int shiftselect);          \
    int table##_move_cursor(table *self, int R, int C);                                            \
    void table##_init_sizes(table *self);                                                          \
    int table##_scrollbar_size(const table *self);                                                 \
    void table##_set_scrollbar_size(table *self, int newSize);                                     \
    void table##_set_tab_cell_nav(table *self, int val);                                           \
    int table##_tab_cell_nav(const table *self);                                                   \
    void table##_set_draw_cell(table *self, void (*)(int, int, int, int, int, int, int, void *),   \
                               void *data);

WIDGET_DECLARE(Fl_Table)

GROUP_DECLARE(Fl_Table)

TABLE_DECLARE(Fl_Table)

WIDGET_DECLARE(Fl_Table_Row)

GROUP_DECLARE(Fl_Table_Row)

TABLE_DECLARE(Fl_Table_Row)

int Fl_Table_Row_row_selected(Fl_Table_Row *self, int row);

int Fl_Table_Row_select_row(Fl_Table_Row *self, int row);

void Fl_Table_Row_select_all_rows(Fl_Table_Row *self);

#ifdef __cplusplus
}
#endif
