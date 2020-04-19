#include "cfl_table.h"
#include "cfl_group.h"
#include <FL/Fl_Image.H>
#include <FL/Fl_Table.H>
#include <FL/Fl_Table_Row.H>
#include <new>


#define TABLE_DEFINE(table)                                                    \
  void table##_set_table_box(table *self, int val) {                           \
    LOCK(self->table_box((Fl_Boxtype)val);)                                    \
  }                                                                            \
  int table##_table_box(table *self) { return self->table_box(); }             \
  void table##_set_rows(table *self, int val) { LOCK(self->rows(val);) }       \
  int table##_rows(table *self) { return self->rows(); }                       \
  void table##_set_cols(table *self, int val) { LOCK(self->cols(val);) }       \
  int table##_cols(table *self) { return self->cols(); }                       \
  void table##_visible_cells(table *self, int *r1, int *r2, int *c1,           \
                             int *c2) {                                        \
    LOCK(self->visible_cells(*r1, *r2, *c1, *c2);)                             \
  }                                                                            \
  int table##_is_interactive_resize(table *self) {                             \
    return self->is_interactive_resize();                                      \
  }                                                                            \
  int table##_row_resize(table *self) { return self->row_resize(); }           \
  void table##_set_row_resize(table *self, int flag) {                         \
    LOCK(self->row_resize(flag);)                                              \
  }                                                                            \
  int table##_col_resize(table *self) { return self->col_resize(); }           \
  void table##_set_col_resize(table *self, int flag) {                         \
    LOCK(self->col_resize(flag);)                                              \
  }                                                                            \
  int table##_col_resize_min(table *self) { return self->col_resize_min(); }   \
  void table##_set_col_resize_min(table *self, int val) {                      \
    LOCK(self->col_resize_min(val);)                                           \
  }                                                                            \
  int table##_row_resize_min(table *self) { return self->row_resize_min(); }   \
  void table##_set_row_resize_min(table *self, int val) {                      \
    LOCK(self->row_resize_min(val);)                                           \
  }                                                                            \
  int table##_row_header(table *self) { return self->row_header(); }           \
  void table##_set_row_header(table *self, int flag) {                         \
    LOCK(self->row_header(flag);)                                              \
  }                                                                            \
  int table##_col_header(table *self) { return self->col_header(); }           \
  void table##_set_col_header(table *self, int flag) {                         \
    LOCK(self->col_header(flag);)                                              \
  }                                                                            \
  void table##_set_col_header_height(table *self, int height) {                \
    LOCK(self->col_header_height(height);)                                     \
  }                                                                            \
  int table##_col_header_height(table *self) {                                 \
    return self->col_header_height();                                          \
  }                                                                            \
  void table##_set_row_header_width(table *self, int width) {                  \
    LOCK(self->row_header_width(width);)                                       \
  }                                                                            \
  int table##_row_header_width(table *self) {                                  \
    return self->row_header_width();                                           \
  }                                                                            \
  void table##_set_row_header_color(table *self, unsigned int val) {           \
    LOCK(self->row_header_color(val);)                                         \
  }                                                                            \
  unsigned int table##_row_header_color(table *self) {                         \
    return self->row_header_color();                                           \
  }                                                                            \
  void table##_set_col_header_color(table *self, unsigned int val) {           \
    LOCK(self->col_header_color(val);)                                         \
  }                                                                            \
  unsigned int table##_col_header_color(table *self) {                         \
    return self->col_header_color();                                           \
  }                                                                            \
  void table##_set_row_height(table *self, int row, int height) {              \
    LOCK(self->row_height(row, height);)                                       \
  }                                                                            \
  int table##_row_height(table *self, int row) {                               \
    return self->row_height(row);                                              \
  }                                                                            \
  void table##_set_col_width(table *self, int col, int width) {                \
    LOCK(self->col_width(col, width);)                                         \
  }                                                                            \
  int table##_col_width(table *self, int col) { return self->col_width(col); } \
  void table##_set_row_height_all(table *self, int height) {                   \
    LOCK(self->row_height_all(height);)                                        \
  }                                                                            \
  void table##_set_col_width_all(table *self, int width) {                     \
    LOCK(self->col_width_all(width);)                                          \
  }                                                                            \
  void table##_set_row_position(table *self, int row) {                        \
    LOCK(self->row_position(row);)                                             \
  }                                                                            \
  void table##_set_col_position(table *self, int col) {                        \
    LOCK(self->col_position(col);)                                             \
  }                                                                            \
  int table##_row_position(table *self) { return self->row_position(); }       \
  int table##_col_position(table *self) { return self->col_position(); }       \
  void table##_set_top_row(table *self, int row) { LOCK(self->top_row(row);) } \
  int table##_top_row(table *self) { return self->top_row(); }                 \
  int table##_is_selected(table *self, int r, int c) {                         \
    return self->is_selected(r, c);                                            \
  }                                                                            \
  void table##_get_selection(table *self, int *row_top, int *col_left,         \
                             int *row_bot, int *col_right) {                   \
    LOCK(self->get_selection(*row_top, *col_left, *row_bot, *col_right);)      \
  }                                                                            \
  void table##_set_selection(table *self, int row_top, int col_left,           \
                             int row_bot, int col_right) {                     \
    LOCK(self->set_selection(row_top, col_left, row_bot, col_right);)          \
  }                                                                            \
  int table##_move_cursor_with_shiftselect(table *self, int R, int C,          \
                                           int shiftselect) {                  \
    return self->move_cursor(R, C, shiftselect);                               \
  }                                                                            \
  int table##_move_cursor(table *self, int R, int C) {                         \
    return self->move_cursor(R, C);                                            \
  }                                                                            \
  void table##_init_sizes(table *self) { LOCK(self->init_sizes();) }           \
  int table##_scrollbar_size(const table *self) {                              \
    return self->scrollbar_size();                                             \
  }                                                                            \
  void table##_set_scrollbar_size(table *self, int newSize) {                  \
    LOCK(self->scrollbar_size(newSize);)                                       \
  }                                                                            \
  void table##_set_tab_cell_nav(table *self, int val) {                        \
    LOCK(self->tab_cell_nav(val);)                                             \
  }                                                                            \
  int table##_tab_cell_nav(const table *self) { return self->tab_cell_nav(); }

WIDGET_DEFINE(Fl_Table)

GROUP_DEFINE(Fl_Table)

TABLE_DEFINE(Fl_Table)

WIDGET_DEFINE(Fl_Table_Row)

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