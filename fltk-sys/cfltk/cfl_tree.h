#pragma once

#include "cfl_widget.h"

#ifdef __cplusplus
extern "C" {
#endif

WIDGET_DECLARE(Fl_Tree)

typedef struct Fl_Tree_Item Fl_Tree_Item;

typedef struct Fl_Tree_Item_Array Fl_Tree_Item_Array;

void show_self(Fl_Tree *self);

void root_label(Fl_Tree *self, const char *new_label);

Fl_Tree_Item *root(Fl_Tree *self);

void set_root(Fl_Tree *self, Fl_Tree_Item *newitem);

Fl_Tree_Item *add(Fl_Tree *self, Fl_Tree_Item *parent_item, const char *name);

Fl_Tree_Item *insert_above(Fl_Tree *self, Fl_Tree_Item *above,
                           const char *name);

Fl_Tree_Item *insert(Fl_Tree *self, Fl_Tree_Item *item, const char *name,
                     int pos);

int Fl_Tree_remove(Fl_Tree *self, Fl_Tree_Item *item);

void clear(Fl_Tree *self);

void clear_children(Fl_Tree *self, Fl_Tree_Item *item);

const Fl_Tree_Item *find_clicked(const Fl_Tree *self, int yonly);

Fl_Tree_Item *item_clicked(Fl_Tree *self);

Fl_Tree_Item *first(Fl_Tree *self);

Fl_Tree_Item *first_visible_item(Fl_Tree *self);

Fl_Tree_Item *next(Fl_Tree *self, Fl_Tree_Item *item);

Fl_Tree_Item *prev(Fl_Tree *self, Fl_Tree_Item *item);

Fl_Tree_Item *last(Fl_Tree *self);

Fl_Tree_Item *last_visible_item(Fl_Tree *self);

Fl_Tree_Item *next_visible_item(Fl_Tree *self, Fl_Tree_Item *start, int dir);

Fl_Tree_Item *first_selected_item(Fl_Tree *self);

Fl_Tree_Item *last_selected_item(Fl_Tree *self);

Fl_Tree_Item *next_item(Fl_Tree *self, Fl_Tree_Item *item, int dir,
                        int visible);

Fl_Tree_Item *next_selected_item(Fl_Tree *self, Fl_Tree_Item *item, int dir);

int get_selected_items(Fl_Tree *self, Fl_Tree_Item **items, int *cnt);

int open(Fl_Tree *self, Fl_Tree_Item *item, int docallback);

void open_toggle(Fl_Tree *self, Fl_Tree_Item *item, int docallback);

int close(Fl_Tree *self, Fl_Tree_Item *item, int docallback);

int is_open(const Fl_Tree *self, Fl_Tree_Item *item);

int is_close(const Fl_Tree *self, Fl_Tree_Item *item);

int Fl_Tree_select(Fl_Tree *self, Fl_Tree_Item *item, int docallback);

void select_toggle(Fl_Tree *self, Fl_Tree_Item *item, int docallback);

int deselect(Fl_Tree *self, Fl_Tree_Item *item, int docallback);

int deselect_all(Fl_Tree *self, Fl_Tree_Item *item, int docallback);

int select_only(Fl_Tree *self, Fl_Tree_Item *selitem, int docallback);

int select_all(Fl_Tree *self, Fl_Tree_Item *item, int docallback);

int extend_selection_dir(Fl_Tree *self, Fl_Tree_Item *from, Fl_Tree_Item *to,
                         int dir, int val, int visible);

int extend_selection(Fl_Tree *self, Fl_Tree_Item *from, Fl_Tree_Item *to,
                     int val, int visible);

void set_item_focus(Fl_Tree *self, Fl_Tree_Item *item);

Fl_Tree_Item *get_item_focus(const Fl_Tree *self);

int is_selected(const Fl_Tree *self, Fl_Tree_Item *item);

int item_labelfont(const Fl_Tree *self);

void set_item_labelfont(Fl_Tree *self, int val);

int item_labelsize(const Fl_Tree *self);

void set_item_labelsize(Fl_Tree *self, int val);

unsigned int item_labelfgcolor(const Fl_Tree *self);

void set_item_labelfgcolor(Fl_Tree *self, unsigned int val);

unsigned int item_labelbgcolor(const Fl_Tree *self);

void set_item_labelbgcolor(Fl_Tree *self, unsigned int val);

unsigned int connectorcolor(const Fl_Tree *self);

void set_connectorcolor(Fl_Tree *self, unsigned int val);

int marginleft(const Fl_Tree *self);

void set_marginleft(Fl_Tree *self, int val);

int margintop(const Fl_Tree *self);

void set_margintop(Fl_Tree *self, int val);

int marginbottom(const Fl_Tree *self);

void set_marginbottom(Fl_Tree *self, int val);

int linespacing(const Fl_Tree *self);

void set_linespacing(Fl_Tree *self, int val);

int openchild_marginbottom(const Fl_Tree *self);

void set_openchild_marginbottom(Fl_Tree *self, int val);

int usericonmarginleft(const Fl_Tree *self);

void set_usericonmarginleft(Fl_Tree *self, int val);

int labelmarginleft(const Fl_Tree *self);

void set_labelmarginleft(Fl_Tree *self, int val);

int widgetmarginleft(const Fl_Tree *self);

void set_widgetmarginleft(Fl_Tree *self, int val);

int connectorwidth(const Fl_Tree *self);

void set_connectorwidth(Fl_Tree *self, int val);

void *usericon(const Fl_Tree *self);

void set_usericon(Fl_Tree *self, void *val);

void *openicon(const Fl_Tree *self);

void set_openicon(Fl_Tree *self, void *val);

void *closeicon(const Fl_Tree *self);

void set_closeicon(Fl_Tree *self, void *val);

int showcollapse(const Fl_Tree *self);

void set_showcollapse(Fl_Tree *self, int val);

int showroot(const Fl_Tree *self);

void set_showroot(Fl_Tree *self, int val);

int connectorstyle(const Fl_Tree *self);

void set_connectorstyle(Fl_Tree *self, int val);

int sortorder(const Fl_Tree *self);

void set_sortorder(Fl_Tree *self, int val);

int selectbox(const Fl_Tree *self);

void set_selectbox(Fl_Tree *self, int val);

int selectmode(const Fl_Tree *self);

void set_selectmode(Fl_Tree *self, int val);

int item_reselect_mode(const Fl_Tree *self);

void set_item_reselect_mode(Fl_Tree *self, int mode);

int item_draw_mode(const Fl_Tree *self);

void set_item_draw_mode(Fl_Tree *self, int mode);

void calc_dimensions(Fl_Tree *self);

void calc_tree(Fl_Tree *self);

void recalc_tree(Fl_Tree *self);

int displayed(Fl_Tree *self, Fl_Tree_Item *item);

void show_item(Fl_Tree *self, Fl_Tree_Item *item, int yoff);

void show_item_top(Fl_Tree *self, Fl_Tree_Item *item);

void show_item_middle(Fl_Tree *self, Fl_Tree_Item *item);

void show_item_bottom(Fl_Tree *self, Fl_Tree_Item *item);

void display(Fl_Tree *self, Fl_Tree_Item *item);

int vposition(const Fl_Tree *self);

void set_vposition(Fl_Tree *self, int pos);

int hposition(const Fl_Tree *self);

void set_hposition(Fl_Tree *self, int pos);

int is_scrollbar(Fl_Tree *self, Fl_Widget *w);

int scrollbar_size(const Fl_Tree *self);

void set_scrollbar_size(Fl_Tree *self, int size);

int is_vscroll_visible(const Fl_Tree *self);

int is_hscroll_visible(const Fl_Tree *self);

void set_callback_item(Fl_Tree *self, Fl_Tree_Item *item);

Fl_Tree_Item *callback_item(Fl_Tree *self);

void set_callback_reason(Fl_Tree *self, int reason);

int callback_reason(const Fl_Tree *self);

#ifdef __cplusplus
}
#endif