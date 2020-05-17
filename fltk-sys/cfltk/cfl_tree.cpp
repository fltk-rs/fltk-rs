#include "cfl_tree.h"
#include <FL/Fl.H>
#include <FL/Fl_Image.H>
#include <FL/Fl_Tree.H>
#include <FL/Fl_Tree_Item.H>
#include <FL/Fl_Tree_Item_Array.H>
#include <FL/Fl_Widget.H>
#include <new>

WIDGET_DEFINE(Fl_Tree)

void show_self(Fl_Tree *self) { self->show_self(); }

void root_label(Fl_Tree *self, const char *new_label) {
    self->root_label(new_label);
}

Fl_Tree_Item *root(Fl_Tree *self) { return self->root(); }

void set_root(Fl_Tree *self, Fl_Tree_Item *newitem) { self->root(newitem); }

Fl_Tree_Item *add(Fl_Tree *self, Fl_Tree_Item *parent_item, const char *name) {
    return self->add(parent_item, name);
}

Fl_Tree_Item *insert_above(Fl_Tree *self, Fl_Tree_Item *above,
                           const char *name) {
    return self->insert_above(above, name);
}

Fl_Tree_Item *insert(Fl_Tree *self, Fl_Tree_Item *item, const char *name,
                     int pos) {
    return self->insert(item, name, pos);
}

int Fl_remove(Fl_Tree *self, Fl_Tree_Item *item) { return self->remove(item); }

void clear(Fl_Tree *self) { self->clear(); }

void clear_children(Fl_Tree *self, Fl_Tree_Item *item) {
    self->clear_children(item);
}

const Fl_Tree_Item *find_clicked(const Fl_Tree *self, int yonly) {
    return self->find_clicked(yonly);
}

Fl_Tree_Item *item_clicked(Fl_Tree *self) { return self->item_clicked(); }

Fl_Tree_Item *first(Fl_Tree *self) { return self->first(); }

Fl_Tree_Item *first_visible_item(Fl_Tree *self) {
    return self->first_visible_item();
}

Fl_Tree_Item *next(Fl_Tree *self, Fl_Tree_Item *item) {
    return self->next(item);
}

Fl_Tree_Item *prev(Fl_Tree *self, Fl_Tree_Item *item) {
    return self->prev(item);
}

Fl_Tree_Item *last(Fl_Tree *self) { return self->last(); }

Fl_Tree_Item *last_visible_item(Fl_Tree *self) {
    return self->last_visible_item();
}

Fl_Tree_Item *next_visible_item(Fl_Tree *self, Fl_Tree_Item *start, int dir) {
    return self->next_visible_item(start, dir);
}

Fl_Tree_Item *first_selected_item(Fl_Tree *self) {
    return self->first_selected_item();
}

Fl_Tree_Item *last_selected_item(Fl_Tree *self) {
    return self->last_selected_item();
}

Fl_Tree_Item *next_item(Fl_Tree *self, Fl_Tree_Item *item, int dir,
                        int visible) {
    return self->next_item(item, dir, visible);
}

Fl_Tree_Item *next_selected_item(Fl_Tree *self, Fl_Tree_Item *item, int dir) {
    return self->next_selected_item(item, dir);
}

int get_selected_items(Fl_Tree *self, Fl_Tree_Item **items, int *cnt) {
    int c = 0;
    for (Fl_Tree_Item *i = self->first_selected_item(); i; i = self->next_selected_item(i))
        c++;
    auto arr = new Fl_Tree_Item_Array(c);
    auto ret = self->get_selected_items(*arr);
    auto first = (*arr)[0];
    items = &first;
    *cnt = c;
    return ret;
}

int open(Fl_Tree *self, Fl_Tree_Item *item, int docallback) {
    return self->open(item, docallback);
}

void open_toggle(Fl_Tree *self, Fl_Tree_Item *item, int docallback) {
    self->open_toggle(item, docallback);
}

int close(Fl_Tree *self, Fl_Tree_Item *item, int docallback) {
    return self->close(item, docallback);
}

int is_open(const Fl_Tree *self, Fl_Tree_Item *item) {
    return self->is_open(item);
}

int is_close(const Fl_Tree *self, Fl_Tree_Item *item) {
    return self->is_close(item);
}

int Fl_Tree_select(Fl_Tree *self, Fl_Tree_Item *item, int docallback) {
    return self->select(item, docallback);
}

void select_toggle(Fl_Tree *self, Fl_Tree_Item *item, int docallback) {
    self->select_toggle(item, docallback);
}

int deselect(Fl_Tree *self, Fl_Tree_Item *item, int docallback) {
    return self->deselect(item, docallback);
}

int deselect_all(Fl_Tree *self, Fl_Tree_Item *item, int docallback) {
    return self->deselect_all(item, docallback);
}

int select_only(Fl_Tree *self, Fl_Tree_Item *selitem, int docallback) {
    return self->select_only(selitem, docallback);
}

int select_all(Fl_Tree *self, Fl_Tree_Item *item, int docallback) {
    return self->select_all(item, docallback);
}

int extend_selection_dir(Fl_Tree *self, Fl_Tree_Item *from, Fl_Tree_Item *to,
                         int dir, int val, int visible) {
    return self->extend_selection_dir(from, to, dir, val, visible);
}

int extend_selection(Fl_Tree *self, Fl_Tree_Item *from, Fl_Tree_Item *to,
                     int val, int visible) {
    return self->extend_selection(from, to, val, visible);
}

void set_item_focus(Fl_Tree *self, Fl_Tree_Item *item) {
    self->set_item_focus(item);
}

Fl_Tree_Item *get_item_focus(const Fl_Tree *self) {
    return self->get_item_focus();
}

int is_selected(const Fl_Tree *self, Fl_Tree_Item *item) {
    return self->is_selected(item);
}

int item_labelfont(const Fl_Tree *self) { return self->item_labelfont(); }

void set_item_labelfont(Fl_Tree *self, int val) { self->item_labelfont(val); }

int item_labelsize(const Fl_Tree *self) { return self->item_labelsize(); }

void set_item_labelsize(Fl_Tree *self, int val) { self->item_labelsize(val); }

unsigned int item_labelfgcolor(const Fl_Tree *self) {
    return self->item_labelfgcolor();
}

void set_item_labelfgcolor(Fl_Tree *self, unsigned int val) {
    self->item_labelfgcolor(val);
}

unsigned int item_labelbgcolor(const Fl_Tree *self) {
    return self->item_labelbgcolor();
}

void set_item_labelbgcolor(Fl_Tree *self, unsigned int val) {
    self->item_labelbgcolor(val);
}

unsigned int connectorcolor(const Fl_Tree *self) {
    return self->connectorcolor();
}

void set_connectorcolor(Fl_Tree *self, unsigned int val) {
    self->connectorcolor(val);
}

int marginleft(const Fl_Tree *self) { return self->marginleft(); }

void set_marginleft(Fl_Tree *self, int val) { self->marginleft(val); }

int margintop(const Fl_Tree *self) { return self->margintop(); }

void set_margintop(Fl_Tree *self, int val) { self->margintop(val); }

int marginbottom(const Fl_Tree *self) { return self->marginbottom(); }

void set_marginbottom(Fl_Tree *self, int val) { self->marginbottom(val); }

int linespacing(const Fl_Tree *self) { return self->linespacing(); }

void set_linespacing(Fl_Tree *self, int val) { self->linespacing(val); }

int openchild_marginbottom(const Fl_Tree *self) {
    return self->openchild_marginbottom();
}

void set_openchild_marginbottom(Fl_Tree *self, int val) {
    self->openchild_marginbottom(val);
}

int usericonmarginleft(const Fl_Tree *self) {
    return self->usericonmarginleft();
}

void set_usericonmarginleft(Fl_Tree *self, int val) {
    self->usericonmarginleft(val);
}

int labelmarginleft(const Fl_Tree *self) { return self->labelmarginleft(); }

void set_labelmarginleft(Fl_Tree *self, int val) { self->labelmarginleft(val); }

int widgetmarginleft(const Fl_Tree *self) { return self->widgetmarginleft(); }

void set_widgetmarginleft(Fl_Tree *self, int val) {
    self->widgetmarginleft(val);
}

int connectorwidth(const Fl_Tree *self) { return self->connectorwidth(); }

void set_connectorwidth(Fl_Tree *self, int val) { self->connectorwidth(val); }

void *usericon(const Fl_Tree *self) { return self->usericon(); }

void set_usericon(Fl_Tree *self, void *val) { self->usericon((Fl_Image *)val); }

void *openicon(const Fl_Tree *self) { return self->openicon(); }

void set_openicon(Fl_Tree *self, void *val) { self->openicon((Fl_Image *)val); }

void *closeicon(const Fl_Tree *self) { return self->closeicon(); }

void set_closeicon(Fl_Tree *self, void *val) {
    self->closeicon((Fl_Image *)val);
}

int showcollapse(const Fl_Tree *self) { return self->showcollapse(); }

void set_showcollapse(Fl_Tree *self, int val) { self->showcollapse(val); }

int showroot(const Fl_Tree *self) { return self->showroot(); }

void set_showroot(Fl_Tree *self, int val) { self->showroot(val); }

int connectorstyle(const Fl_Tree *self) { return self->connectorstyle(); }

void set_connectorstyle(Fl_Tree *self, int val) {
    self->connectorstyle((Fl_Tree_Connector)val);
}

int sortorder(const Fl_Tree *self) { return self->sortorder(); }

void set_sortorder(Fl_Tree *self, int val) {
    self->sortorder((Fl_Tree_Sort)val);
}

int selectbox(const Fl_Tree *self) { return self->selectbox(); }

void set_selectbox(Fl_Tree *self, int val) { self->selectbox((Fl_Boxtype)val); }

int selectmode(const Fl_Tree *self) { return self->selectmode(); }

void set_selectmode(Fl_Tree *self, int val) {
    self->selectmode((Fl_Tree_Select)val);
}

int item_reselect_mode(const Fl_Tree *self) {
    return self->item_reselect_mode();
}

void set_item_reselect_mode(Fl_Tree *self, int mode) {
    self->item_reselect_mode((Fl_Tree_Item_Reselect_Mode)mode);
}

int item_draw_mode(const Fl_Tree *self) { return self->item_draw_mode(); }

void set_item_draw_mode(Fl_Tree *self, int mode) { self->item_draw_mode(mode); }

void calc_dimensions(Fl_Tree *self) { self->calc_dimensions(); }

void calc_tree(Fl_Tree *self) { self->calc_tree(); }

void recalc_tree(Fl_Tree *self) { self->recalc_tree(); }

int displayed(Fl_Tree *self, Fl_Tree_Item *item) {
    return self->displayed(item);
}

void show_item(Fl_Tree *self, Fl_Tree_Item *item, int yoff) {
    self->show_item(item, yoff);
}

void show_item_top(Fl_Tree *self, Fl_Tree_Item *item) {
    self->show_item_top(item);
}

void show_item_middle(Fl_Tree *self, Fl_Tree_Item *item) {
    self->show_item_middle(item);
}

void show_item_bottom(Fl_Tree *self, Fl_Tree_Item *item) {
    self->show_item_bottom(item);
}

void display(Fl_Tree *self, Fl_Tree_Item *item) { self->display(item); }

int vposition(const Fl_Tree *self) { return self->vposition(); }

void set_vposition(Fl_Tree *self, int pos) { self->vposition(pos); }

int hposition(const Fl_Tree *self) { return self->hposition(); }

void set_hposition(Fl_Tree *self, int pos) { self->hposition(pos); }

int is_scrollbar(Fl_Tree *self, Fl_Widget *w) { return self->is_scrollbar(w); }

int scrollbar_size(const Fl_Tree *self) { return self->scrollbar_size(); }

void set_scrollbar_size(Fl_Tree *self, int size) { self->scrollbar_size(size); }

int is_vscroll_visible(const Fl_Tree *self) {
    return self->is_vscroll_visible();
}

int is_hscroll_visible(const Fl_Tree *self) {
    return self->is_hscroll_visible();
}

void set_callback_item(Fl_Tree *self, Fl_Tree_Item *item) {
    self->callback_item(item);
}

Fl_Tree_Item *callback_item(Fl_Tree *self) { return self->callback_item(); }

void set_callback_reason(Fl_Tree *self, int reason) {
    self->callback_reason((Fl_Tree_Reason)reason);
}

int callback_reason(const Fl_Tree *self) { return self->callback_reason(); }
