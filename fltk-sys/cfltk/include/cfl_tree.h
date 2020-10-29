#pragma once

#include "cfl_widget.h"

#ifdef __cplusplus
extern "C" {
#endif

WIDGET_DECLARE(Fl_Tree)

typedef struct Fl_Tree_Item Fl_Tree_Item;

typedef struct Fl_Tree_Item_Array Fl_Tree_Item_Array;

void Fl_Tree_begin(Fl_Tree *self);

void Fl_Tree_end(Fl_Tree *self);

void Fl_Tree_show_self(Fl_Tree *self);

void Fl_Tree_root_label(Fl_Tree *self, const char *new_label);

Fl_Tree_Item *Fl_Tree_root(Fl_Tree *self);

void Fl_Tree_set_root(Fl_Tree *self, Fl_Tree_Item *newitem);

Fl_Tree_Item *Fl_Tree_add(Fl_Tree *self, const char *name);

Fl_Tree_Item *Fl_Tree_insert_above(Fl_Tree *self, Fl_Tree_Item *above, const char *name);

Fl_Tree_Item *Fl_Tree_insert(Fl_Tree *self, Fl_Tree_Item *item, const char *name, int pos);

const Fl_Tree_Item *Fl_Tree_find_item(const Fl_Tree *self, const char *path);

Fl_Tree_Item *Fl_Tree_find_item_mut(Fl_Tree *self, const char *path);

int Fl_Tree_remove(Fl_Tree *self, Fl_Tree_Item *item);

void Fl_Tree_clear(Fl_Tree *self);

void Fl_Tree_clear_children(Fl_Tree *self, Fl_Tree_Item *item);

const Fl_Tree_Item *Fl_Tree_find_clicked(const Fl_Tree *self, int yonly);

Fl_Tree_Item *Fl_Tree_item_clicked(Fl_Tree *self);

Fl_Tree_Item *Fl_Tree_first(Fl_Tree *self);

Fl_Tree_Item *Fl_Tree_first_visible_item(Fl_Tree *self);

Fl_Tree_Item *Fl_Tree_next(Fl_Tree *self, Fl_Tree_Item *item);

Fl_Tree_Item *Fl_Tree_prev(Fl_Tree *self, Fl_Tree_Item *item);

Fl_Tree_Item *Fl_Tree_last(Fl_Tree *self);

Fl_Tree_Item *Fl_Tree_last_visible_item(Fl_Tree *self);

Fl_Tree_Item *Fl_Tree_next_visible_item(Fl_Tree *self, Fl_Tree_Item *start, int dir);

Fl_Tree_Item *Fl_Tree_first_selected_item(Fl_Tree *self);

Fl_Tree_Item *Fl_Tree_last_selected_item(Fl_Tree *self);

Fl_Tree_Item *Fl_Tree_next_item(Fl_Tree *self, Fl_Tree_Item *item, int dir, int visible);

Fl_Tree_Item *Fl_Tree_next_selected_item(Fl_Tree *self, Fl_Tree_Item *item, int dir);

int Fl_Tree_get_selected_items(Fl_Tree *self, Fl_Tree_Item_Array **arr);

int Fl_Tree_get_items(Fl_Tree *self, Fl_Tree_Item_Array **arr);

int Fl_Tree_open(Fl_Tree *self, const char *path, int docallback);

void Fl_Tree_open_toggle(Fl_Tree *self, Fl_Tree_Item *item, int docallback);

int Fl_Tree_close(Fl_Tree *self, const char *path, int docallback);

int Fl_Tree_is_open(const Fl_Tree *self, const char *path);

int Fl_Tree_is_close(const Fl_Tree *self, const char *path);

int Fl_Tree_select(Fl_Tree *self, const char *path, int docallback);

void Fl_Tree_select_toggle(Fl_Tree *self, Fl_Tree_Item *item, int docallback);

int Fl_Tree_deselect(Fl_Tree *self, const char *path, int docallback);

int Fl_Tree_deselect_all(Fl_Tree *self, Fl_Tree_Item *item, int docallback);

int Fl_Tree_select_only(Fl_Tree *self, Fl_Tree_Item *selitem, int docallback);

int Fl_Tree_select_all(Fl_Tree *self, Fl_Tree_Item *item, int docallback);

int Fl_Tree_extend_selection_dir(Fl_Tree *self, Fl_Tree_Item *from, Fl_Tree_Item *to, int dir,
                                 int val, int visible);

int Fl_Tree_extend_selection(Fl_Tree *self, Fl_Tree_Item *from, Fl_Tree_Item *to, int val,
                             int visible);

void Fl_Tree_set_item_focus(Fl_Tree *self, Fl_Tree_Item *item);

Fl_Tree_Item *Fl_Tree_get_item_focus(const Fl_Tree *self);

int Fl_Tree_is_selected(Fl_Tree *self, const char *path);

int Fl_Tree_item_labelfont(const Fl_Tree *self);

void Fl_Tree_set_item_labelfont(Fl_Tree *self, int val);

int Fl_Tree_item_labelsize(const Fl_Tree *self);

void Fl_Tree_set_item_labelsize(Fl_Tree *self, int val);

unsigned int Fl_Tree_item_labelfgcolor(const Fl_Tree *self);

void Fl_Tree_set_item_labelfgcolor(Fl_Tree *self, unsigned int val);

unsigned int Fl_Tree_item_labelbgcolor(const Fl_Tree *self);

void Fl_Tree_set_item_labelbgcolor(Fl_Tree *self, unsigned int val);

unsigned int Fl_Tree_connectorcolor(const Fl_Tree *self);

void Fl_Tree_set_connectorcolor(Fl_Tree *self, unsigned int val);

int Fl_Tree_marginleft(const Fl_Tree *self);

void Fl_Tree_set_marginleft(Fl_Tree *self, int val);

int Fl_Tree_margintop(const Fl_Tree *self);

void Fl_Tree_set_margintop(Fl_Tree *self, int val);

int Fl_Tree_marginbottom(const Fl_Tree *self);

void Fl_Tree_set_marginbottom(Fl_Tree *self, int val);

int Fl_Tree_linespacing(const Fl_Tree *self);

void Fl_Tree_set_linespacing(Fl_Tree *self, int val);

int Fl_Tree_openchild_marginbottom(const Fl_Tree *self);

void Fl_Tree_set_openchild_marginbottom(Fl_Tree *self, int val);

int Fl_Tree_usericonmarginleft(const Fl_Tree *self);

void Fl_Tree_set_usericonmarginleft(Fl_Tree *self, int val);

int Fl_Tree_labelmarginleft(const Fl_Tree *self);

void Fl_Tree_set_labelmarginleft(Fl_Tree *self, int val);

int Fl_Tree_widgetmarginleft(const Fl_Tree *self);

void Fl_Tree_set_widgetmarginleft(Fl_Tree *self, int val);

int Fl_Tree_connectorwidth(const Fl_Tree *self);

void Fl_Tree_set_connectorwidth(Fl_Tree *self, int val);

void *Fl_Tree_usericon(const Fl_Tree *self);

void Fl_Tree_set_usericon(Fl_Tree *self, void *val);

void *Fl_Tree_openicon(const Fl_Tree *self);

void Fl_Tree_set_openicon(Fl_Tree *self, void *val);

void *Fl_Tree_closeicon(const Fl_Tree *self);

void Fl_Tree_set_closeicon(Fl_Tree *self, void *val);

int Fl_Tree_showcollapse(const Fl_Tree *self);

void Fl_Tree_set_showcollapse(Fl_Tree *self, int val);

int Fl_Tree_showroot(const Fl_Tree *self);

void Fl_Tree_set_showroot(Fl_Tree *self, int val);

int Fl_Tree_connectorstyle(const Fl_Tree *self);

void Fl_Tree_set_connectorstyle(Fl_Tree *self, int val);

int Fl_Tree_sortorder(const Fl_Tree *self);

void Fl_Tree_set_sortorder(Fl_Tree *self, int val);

int Fl_Tree_selectbox(const Fl_Tree *self);

void Fl_Tree_set_selectbox(Fl_Tree *self, int val);

int Fl_Tree_selectmode(const Fl_Tree *self);

void Fl_Tree_set_selectmode(Fl_Tree *self, int val);

int Fl_Tree_item_reselect_mode(const Fl_Tree *self);

void Fl_Tree_set_item_reselect_mode(Fl_Tree *self, int mode);

int Fl_Tree_item_draw_mode(const Fl_Tree *self);

void Fl_Tree_set_item_draw_mode(Fl_Tree *self, int mode);

void Fl_Tree_calc_dimensions(Fl_Tree *self);

void Fl_Tree_calc_tree(Fl_Tree *self);

void Fl_Tree_recalc_tree(Fl_Tree *self);

int Fl_Tree_displayed(Fl_Tree *self, Fl_Tree_Item *item);

void Fl_Tree_show_item(Fl_Tree *self, Fl_Tree_Item *item, int yoff);

void Fl_Tree_show_item_top(Fl_Tree *self, Fl_Tree_Item *item);

void Fl_Tree_show_item_middle(Fl_Tree *self, Fl_Tree_Item *item);

void Fl_Tree_show_item_bottom(Fl_Tree *self, Fl_Tree_Item *item);

void Fl_Tree_display(Fl_Tree *self, Fl_Tree_Item *item);

int Fl_Tree_vposition(const Fl_Tree *self);

void Fl_Tree_set_vposition(Fl_Tree *self, int pos);

int Fl_Tree_hposition(const Fl_Tree *self);

void Fl_Tree_set_hposition(Fl_Tree *self, int pos);

int Fl_Tree_is_scrollbar(Fl_Tree *self, Fl_Widget *w);

int Fl_Tree_scrollbar_size(const Fl_Tree *self);

void Fl_Tree_set_scrollbar_size(Fl_Tree *self, int size);

int Fl_Tree_is_vscroll_visible(const Fl_Tree *self);

int Fl_Tree_is_hscroll_visible(const Fl_Tree *self);

void Fl_Tree_set_callback_item(Fl_Tree *self, Fl_Tree_Item *item);

Fl_Tree_Item *Fl_Tree_callback_item(Fl_Tree *self);

void Fl_Tree_set_callback_reason(Fl_Tree *self, int reason);

int Fl_Tree_callback_reason(const Fl_Tree *self);

/* TreeItems */

int Fl_Tree_Item_x(const Fl_Tree_Item *self);

int Fl_Tree_Item_y(const Fl_Tree_Item *self);

int Fl_Tree_Item_w(const Fl_Tree_Item *self);

int Fl_Tree_Item_h(const Fl_Tree_Item *self);

int Fl_Tree_Item_label_x(const Fl_Tree_Item *self);

int Fl_Tree_Item_label_y(const Fl_Tree_Item *self);

int Fl_Tree_Item_label_w(const Fl_Tree_Item *self);

int Fl_Tree_Item_label_h(const Fl_Tree_Item *self);

void Fl_Tree_Item_show_self(const Fl_Tree_Item *self, const char *indent);

void Fl_Tree_set_Item_label(Fl_Tree_Item *self, const char *val);

const char *Fl_Tree_Item_label(const Fl_Tree_Item *self);

void Fl_Tree_Item_set_labelfont(Fl_Tree_Item *self, int val);

int Fl_Tree_Item_labelfont(const Fl_Tree_Item *self);

void Fl_Tree_Item_set_labelsize(Fl_Tree_Item *self, int val);

int Fl_Tree_Item_labelsize(const Fl_Tree_Item *self);

void Fl_Tree_Item_set_labelfgcolor(Fl_Tree_Item *self, unsigned int val);

unsigned int Fl_Tree_Item_labelfgcolor(const Fl_Tree_Item *self);

void Fl_Tree_Item_set_labelcolor(Fl_Tree_Item *self, unsigned int val);

unsigned int Fl_Tree_Item_labelcolor(const Fl_Tree_Item *self);

void Fl_Tree_Item_set_labelbgcolor(Fl_Tree_Item *self, unsigned int val);

unsigned int Fl_Tree_Item_labelbgcolor(const Fl_Tree_Item *self);

void Fl_Tree_Item_set_widget(Fl_Tree_Item *self, Fl_Widget *val);

Fl_Widget *Fl_Tree_Item_widget(const Fl_Tree_Item *self);

int Fl_Tree_Item_children(const Fl_Tree_Item *self);

const Fl_Tree_Item *Fl_Tree_Item_child(const Fl_Tree_Item *self, int t);

int Fl_Tree_Item_has_children(const Fl_Tree_Item *self);

int Fl_Tree_Item_find_child(Fl_Tree_Item *self, const char *name);

int Fl_Tree_Item_remove_child(Fl_Tree_Item *self, const char *new_label);

void Fl_Tree_Item_clear_children(Fl_Tree_Item *self);

int Fl_Tree_Item_swap_children(Fl_Tree_Item *self, Fl_Tree_Item *a, Fl_Tree_Item *b);

const Fl_Tree_Item *Fl_Tree_Item_find_child_item(const Fl_Tree_Item *self, const char *name);

Fl_Tree_Item *Fl_Tree_Item_replace(Fl_Tree_Item *self, Fl_Tree_Item *new_item);

Fl_Tree_Item *Fl_Tree_Item_replace_child(Fl_Tree_Item *self, Fl_Tree_Item *olditem,
                                         Fl_Tree_Item *newitem);

Fl_Tree_Item *Fl_Tree_Item_deparent(Fl_Tree_Item *self, int index);

int Fl_Tree_Item_reparent(Fl_Tree_Item *self, Fl_Tree_Item *newchild, int index);

int Fl_Tree_Item_move(Fl_Tree_Item *self, int to, int from);

int Fl_Tree_Item_move_above(Fl_Tree_Item *self, Fl_Tree_Item *item);

int Fl_Tree_Item_move_below(Fl_Tree_Item *self, Fl_Tree_Item *item);

int Fl_Tree_Item_move_into(Fl_Tree_Item *self, Fl_Tree_Item *item, int pos);

int Fl_Tree_Item_depth(const Fl_Tree_Item *self);

Fl_Tree_Item *Fl_Tree_Item_prev(Fl_Tree_Item *self);

Fl_Tree_Item *Fl_Tree_Item_next(Fl_Tree_Item *self);

Fl_Tree_Item *Fl_Tree_Item_next_sibling(Fl_Tree_Item *self);

Fl_Tree_Item *Fl_Tree_Item_prev_sibling(Fl_Tree_Item *self);

void Fl_Tree_Item_update_prev_next(Fl_Tree_Item *self, int index);

const Fl_Tree_Item *Fl_Tree_Item_parent(const Fl_Tree_Item *self);

void Fl_Tree_Item_set_parent(Fl_Tree_Item *self, Fl_Tree_Item *val);

const Fl_Tree *Fl_Tree_Item_tree(const Fl_Tree_Item *self);

void Fl_Tree_Item_open(Fl_Tree_Item *self);

void Fl_Tree_Item_close(Fl_Tree_Item *self);

int Fl_Tree_Item_is_open(const Fl_Tree_Item *self);

int Fl_Tree_Item_is_close(const Fl_Tree_Item *self);

void Fl_Tree_Item_open_toggle(Fl_Tree_Item *self);

void Fl_Tree_Item_select(Fl_Tree_Item *self, int val);

void Fl_Tree_Item_select_toggle(Fl_Tree_Item *self);

int Fl_Tree_Item_select_all(Fl_Tree_Item *self);

void Fl_Tree_Item_deselect(Fl_Tree_Item *self);

int Fl_Tree_Item_deselect_all(Fl_Tree_Item *self);

int Fl_Tree_Item_is_root(const Fl_Tree_Item *self);

int Fl_Tree_Item_is_visible(const Fl_Tree_Item *self);

char Fl_Tree_Item_is_active(const Fl_Tree_Item *self);

char Fl_Tree_Item_is_activated(const Fl_Tree_Item *self);

void Fl_Tree_Item_deactivate(Fl_Tree_Item *self);

void Fl_Tree_Item_activate(Fl_Tree_Item *self, int val);

char Fl_Tree_Item_is_selected(const Fl_Tree_Item *self);

/* TreeItemArray */

int Fl_Tree_Item_Array_total(const Fl_Tree_Item_Array *self);

void Fl_Tree_Item_Array_swap(Fl_Tree_Item_Array *self, int ax, int bx);

int Fl_Tree_Item_Array_move(Fl_Tree_Item_Array *self, int to, int from);

int Fl_Tree_Item_Array_deparent(Fl_Tree_Item_Array *self, int pos);

int Fl_Tree_Item_Array_reparent(Fl_Tree_Item_Array *self, Fl_Tree_Item *item,
                                Fl_Tree_Item *newparent, int pos);

void Fl_Tree_Item_Array_clear(Fl_Tree_Item_Array *self);

void Fl_Tree_Item_Array_add(Fl_Tree_Item_Array *self, Fl_Tree_Item *val);

void Fl_Tree_Item_Array_insert(Fl_Tree_Item_Array *self, int pos, Fl_Tree_Item *new_item);

void Fl_Tree_Item_Array_replace(Fl_Tree_Item_Array *self, int pos, Fl_Tree_Item *new_item);

void Fl_Tree_Item_Array_remove(Fl_Tree_Item_Array *self, int index);

int Fl_Tree_Item_Array_remove_item(Fl_Tree_Item_Array *self, Fl_Tree_Item *item);

Fl_Tree_Item *Fl_Tree_Item_Array_at(Fl_Tree_Item_Array *self, int index);

void Fl_Tree_Item_Array_delete(Fl_Tree_Item_Array *self);

#ifdef __cplusplus
}
#endif
