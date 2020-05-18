#include "cfl_tree.h"
#include <FL/Fl.H>
#include <FL/Fl_Image.H>
#include <FL/Fl_Tree.H>
#include <FL/Fl_Tree_Item.H>
#include <FL/Fl_Tree_Item_Array.H>
#include <FL/Fl_Widget.H>
#include <new>

WIDGET_DEFINE(Fl_Tree)

void Fl_Tree_begin(Fl_Tree *self) { LOCK(self->begin();) }

void Fl_Tree_end(Fl_Tree *self) { LOCK(self->end();) }

void Fl_Tree_show_self(Fl_Tree *self) { LOCK(self->show_self();) }

void Fl_Tree_root_label(Fl_Tree *self, const char *new_label) {
    LOCK(self->root_label(new_label);)
}

Fl_Tree_Item *Fl_Tree_root(Fl_Tree *self) { return self->root(); }

void Fl_Tree_set_root(Fl_Tree *self, Fl_Tree_Item *newitem) {
    LOCK(self->root(newitem);)
}

Fl_Tree_Item *Fl_Tree_add(Fl_Tree *self, const char *name) {
    Fl_Tree_Item *ret;
    LOCK(ret = self->add(name));
    return ret;
}

Fl_Tree_Item *Fl_Tree_insert_above(Fl_Tree *self, Fl_Tree_Item *above,
                                   const char *name) {
    Fl_Tree_Item *ret;
    LOCK(self->insert_above(above, name));
    return ret;
}

Fl_Tree_Item *Fl_Tree_insert(Fl_Tree *self, Fl_Tree_Item *item,
                             const char *name, int pos) {
    Fl_Tree_Item *ret;
    LOCK(self->insert(item, name, pos));
    return ret;
}

const Fl_Tree_Item *Fl_Tree_find_item(const Fl_Tree *self, const char *path) {
    if (!path || strlen(path) == 0)
        return NULL;
    const Fl_Tree_Item *item = self->find_item(path);
    return item;
}

Fl_Tree_Item *Fl_Tree_find_item_mut(Fl_Tree *self, const char *path) {
    if (!path || strlen(path) == 0)
        return NULL;
    Fl_Tree_Item *item = self->find_item(path);
    return item;
}

int Fl_Tree_remove(Fl_Tree *self, Fl_Tree_Item *item) {
    int ret;
    LOCK(ret = self->remove(item));
    return ret;
}

void Fl_Tree_clear(Fl_Tree *self) { self->clear(); }

void Fl_Tree_clear_children(Fl_Tree *self, Fl_Tree_Item *item) {
    self->clear_children(item);
}

const Fl_Tree_Item *Fl_Tree_find_clicked(const Fl_Tree *self, int yonly) {
    return self->find_clicked(yonly);
}

Fl_Tree_Item *Fl_Tree_item_clicked(Fl_Tree *self) {
    return self->item_clicked();
}

Fl_Tree_Item *Fl_Tree_first(Fl_Tree *self) { return self->first(); }

Fl_Tree_Item *Fl_Tree_first_visible_item(Fl_Tree *self) {
    return self->first_visible_item();
}

Fl_Tree_Item *Fl_Tree_next(Fl_Tree *self, Fl_Tree_Item *item) {
    return self->next(item);
}

Fl_Tree_Item *Fl_Tree_prev(Fl_Tree *self, Fl_Tree_Item *item) {
    return self->prev(item);
}

Fl_Tree_Item *Fl_Tree_last(Fl_Tree *self) { return self->last(); }

Fl_Tree_Item *Fl_Tree_last_visible_item(Fl_Tree *self) {
    return self->last_visible_item();
}

Fl_Tree_Item *Fl_Tree_next_visible_item(Fl_Tree *self, Fl_Tree_Item *start,
                                        int dir) {
    return self->next_visible_item(start, dir);
}

Fl_Tree_Item *Fl_Tree_first_selected_item(Fl_Tree *self) {
    return self->first_selected_item();
}

Fl_Tree_Item *Fl_Tree_last_selected_item(Fl_Tree *self) {
    return self->last_selected_item();
}

Fl_Tree_Item *Fl_Tree_next_item(Fl_Tree *self, Fl_Tree_Item *item, int dir,
                                int visible) {
    return self->next_item(item, dir, visible);
}

Fl_Tree_Item *Fl_Tree_next_selected_item(Fl_Tree *self, Fl_Tree_Item *item,
                                         int dir) {
    return self->next_selected_item(item, dir);
}

int Fl_Tree_get_selected_items(Fl_Tree *self, Fl_Tree_Item_Array **arr) {
    int c = 0;
    for (Fl_Tree_Item *i = self->first_selected_item(); i;
         i = self->next_selected_item(i))
        c++;
    if (c == 0)
        return 0;
    *arr = new (std::nothrow) Fl_Tree_Item_Array(c);
    auto ret = self->get_selected_items(**arr);
    return ret;
}

int Fl_Tree_open(Fl_Tree *self, const char *path, int docallback) {
    if (!path)
        return 0;
    int ret;
    LOCK(ret = self->open(path, docallback));
    return ret;
}

void Fl_Tree_open_toggle(Fl_Tree *self, Fl_Tree_Item *item, int docallback) {
    LOCK(self->open_toggle(item, docallback);)
}

int Fl_Tree_close(Fl_Tree *self, const char *path, int docallback) {
    int ret;
    LOCK(self->close(path, 1));
    return ret;
}

int Fl_Tree_is_open(const Fl_Tree *self, const char *path) {
    return self->is_open(path);
}

int Fl_Tree_is_close(const Fl_Tree *self, const char *path) {
    return self->is_close(path);
}

int Fl_Tree_select(Fl_Tree *self, const char *path, int docallback) {
    int ret;
    LOCK(ret = self->select(path, docallback));
    return ret;
}

void Fl_Tree_select_toggle(Fl_Tree *self, Fl_Tree_Item *item, int docallback) {
    LOCK(self->select_toggle(item, docallback);)
}

int Fl_Tree_deselect(Fl_Tree *self, const char *path, int docallback) {
    int ret;
    LOCK(ret = self->deselect(path, docallback));
    return ret;
}

int Fl_Tree_deselect_all(Fl_Tree *self, Fl_Tree_Item *item, int docallback) {
    int ret;
    LOCK(ret = self->deselect_all(item, docallback));
    return ret;
}

int Fl_Tree_select_only(Fl_Tree *self, Fl_Tree_Item *selitem, int docallback) {
    int ret;
    LOCK(ret = self->select_only(selitem, docallback));
    return ret;
}

int Fl_Tree_select_all(Fl_Tree *self, Fl_Tree_Item *item, int docallback) {
    int ret;
    LOCK(ret = self->select_all(item, docallback));
    return ret;
}

int Fl_Tree_extend_selection_dir(Fl_Tree *self, Fl_Tree_Item *from,
                                 Fl_Tree_Item *to, int dir, int val,
                                 int visible) {
    int ret;
    LOCK(ret = self->extend_selection_dir(from, to, dir, val, visible));
    return ret;
}

int Fl_Tree_extend_selection(Fl_Tree *self, Fl_Tree_Item *from,
                             Fl_Tree_Item *to, int val, int visible) {
    int ret;
    LOCK(ret = self->extend_selection(from, to, val, visible));
    return ret;
}

void Fl_Tree_set_item_focus(Fl_Tree *self, Fl_Tree_Item *item) {
    LOCK(self->set_item_focus(item);)
}

Fl_Tree_Item *Fl_Tree_get_item_focus(const Fl_Tree *self) {
    return self->get_item_focus();
}

int Fl_Tree_is_selected(Fl_Tree *self, const char *path) {
    return self->is_selected(path);
}

int Fl_Tree_item_labelfont(const Fl_Tree *self) {
    return self->item_labelfont();
}

void Fl_Tree_set_item_labelfont(Fl_Tree *self, int val) {
    LOCK(self->item_labelfont(val);)
}

int Fl_Tree_item_labelsize(const Fl_Tree *self) {
    return self->item_labelsize();
}

void Fl_Tree_set_item_labelsize(Fl_Tree *self, int val) {
    LOCK(self->item_labelsize(val);)
}

unsigned int Fl_Tree_item_labelfgcolor(const Fl_Tree *self) {
    return self->item_labelfgcolor();
}

void Fl_Tree_set_item_labelfgcolor(Fl_Tree *self, unsigned int val) {
    LOCK(self->item_labelfgcolor(val);)
}

unsigned int Fl_Tree_item_labelbgcolor(const Fl_Tree *self) {
    return self->item_labelbgcolor();
}

void Fl_Tree_set_item_labelbgcolor(Fl_Tree *self, unsigned int val) {
    LOCK(self->item_labelbgcolor(val);)
}

unsigned int Fl_Tree_connectorcolor(const Fl_Tree *self) {
    return self->connectorcolor();
}

void Fl_Tree_set_connectorcolor(Fl_Tree *self, unsigned int val) {
    LOCK(self->connectorcolor(val);)
}

int Fl_Tree_marginleft(const Fl_Tree *self) { return self->marginleft(); }

void Fl_Tree_set_marginleft(Fl_Tree *self, int val) {
    LOCK(self->marginleft(val);)
}

int Fl_Tree_margintop(const Fl_Tree *self) { return self->margintop(); }

void Fl_Tree_set_margintop(Fl_Tree *self, int val) {
    LOCK(self->margintop(val);)
}

int Fl_Tree_marginbottom(const Fl_Tree *self) { return self->marginbottom(); }

void Fl_Tree_set_marginbottom(Fl_Tree *self, int val) {
    LOCK(self->marginbottom(val);)
}

int Fl_Tree_linespacing(const Fl_Tree *self) { return self->linespacing(); }

void Fl_Tree_set_linespacing(Fl_Tree *self, int val) {
    LOCK(self->linespacing(val);)
}

int Fl_Tree_openchild_marginbottom(const Fl_Tree *self) {
    return self->openchild_marginbottom();
}

void Fl_Tree_set_openchild_marginbottom(Fl_Tree *self, int val) {
    LOCK(self->openchild_marginbottom(val);)
}

int Fl_Tree_usericonmarginleft(const Fl_Tree *self) {
    return self->usericonmarginleft();
}

void Fl_Tree_set_usericonmarginleft(Fl_Tree *self, int val) {
    LOCK(self->usericonmarginleft(val);)
}

int Fl_Tree_labelmarginleft(const Fl_Tree *self) {
    return self->labelmarginleft();
}

void Fl_Tree_set_labelmarginleft(Fl_Tree *self, int val) {
    LOCK(self->labelmarginleft(val);)
}

int Fl_Tree_widgetmarginleft(const Fl_Tree *self) {
    return self->widgetmarginleft();
}

void Fl_Tree_set_widgetmarginleft(Fl_Tree *self, int val) {
    LOCK(self->widgetmarginleft(val);)
}

int Fl_Tree_connectorwidth(const Fl_Tree *self) {
    return self->connectorwidth();
}

void Fl_Tree_set_connectorwidth(Fl_Tree *self, int val) {
    LOCK(self->connectorwidth(val);)
}

void *Fl_Tree_usericon(const Fl_Tree *self) { return self->usericon(); }

void Fl_Tree_set_usericon(Fl_Tree *self, void *val) {
    LOCK(self->usericon((Fl_Image *)val);)
}

void *Fl_Tree_openicon(const Fl_Tree *self) { return self->openicon(); }

void Fl_Tree_set_openicon(Fl_Tree *self, void *val) {
    LOCK(self->openicon((Fl_Image *)val);)
}

void *Fl_Tree_closeicon(const Fl_Tree *self) { return self->closeicon(); }

void Fl_Tree_set_closeicon(Fl_Tree *self, void *val) {
    LOCK(self->closeicon((Fl_Image *)val);)
}

int Fl_Tree_showcollapse(const Fl_Tree *self) { return self->showcollapse(); }

void Fl_Tree_set_showcollapse(Fl_Tree *self, int val) {
    LOCK(self->showcollapse(val);)
}

int Fl_Tree_showroot(const Fl_Tree *self) { return self->showroot(); }

void Fl_Tree_set_showroot(Fl_Tree *self, int val) { LOCK(self->showroot(val);) }

int Fl_Tree_connectorstyle(const Fl_Tree *self) {
    return self->connectorstyle();
}

void Fl_Tree_set_connectorstyle(Fl_Tree *self, int val) {
    LOCK(self->connectorstyle((Fl_Tree_Connector)val);)
}

int Fl_Tree_sortorder(const Fl_Tree *self) { return self->sortorder(); }

void Fl_Tree_set_sortorder(Fl_Tree *self, int val) {
    LOCK(self->sortorder((Fl_Tree_Sort)val);)
}

int Fl_Tree_selectbox(const Fl_Tree *self) { return self->selectbox(); }

void Fl_Tree_set_selectbox(Fl_Tree *self, int val) {
    LOCK(self->selectbox((Fl_Boxtype)val);)
}

int Fl_Tree_selectmode(const Fl_Tree *self) { return self->selectmode(); }

void Fl_Tree_set_selectmode(Fl_Tree *self, int val) {
    LOCK(self->selectmode((Fl_Tree_Select)val);)
}

int Fl_Tree_item_reselect_mode(const Fl_Tree *self) {
    return self->item_reselect_mode();
}

void Fl_Tree_set_item_reselect_mode(Fl_Tree *self, int mode) {
    LOCK(self->item_reselect_mode((Fl_Tree_Item_Reselect_Mode)mode);)
}

int Fl_Tree_item_draw_mode(const Fl_Tree *self) {
    return self->item_draw_mode();
}

void Fl_Tree_set_item_draw_mode(Fl_Tree *self, int mode) {
    LOCK(self->item_draw_mode(mode);)
}

void Fl_Tree_calc_dimensions(Fl_Tree *self) { LOCK(self->calc_dimensions();) }

void Fl_Tree_calc_tree(Fl_Tree *self) { LOCK(self->calc_tree();) }

void Fl_Tree_recalc_tree(Fl_Tree *self) { LOCK(self->recalc_tree();) }

int Fl_Tree_displayed(Fl_Tree *self, Fl_Tree_Item *item) {
    int ret;
    LOCK(ret = self->displayed(item));
    return ret;
}

void Fl_Tree_show_item(Fl_Tree *self, Fl_Tree_Item *item, int yoff) {
    LOCK(self->show_item(item, yoff);)
}

void Fl_Tree_show_item_top(Fl_Tree *self, Fl_Tree_Item *item) {
    LOCK(self->show_item_top(item);)
}

void Fl_Tree_show_item_middle(Fl_Tree *self, Fl_Tree_Item *item) {
    LOCK(self->show_item_middle(item);)
}

void Fl_Tree_show_item_bottom(Fl_Tree *self, Fl_Tree_Item *item) {
    LOCK(self->show_item_bottom(item);)
}

void Fl_Tree_display(Fl_Tree *self, Fl_Tree_Item *item) {
    LOCK(self->display(item);)
}

int Fl_Tree_vposition(const Fl_Tree *self) { return self->vposition(); }

void Fl_Tree_set_vposition(Fl_Tree *self, int pos) {
    LOCK(self->vposition(pos);)
}

int Fl_Tree_hposition(const Fl_Tree *self) { return self->hposition(); }

void Fl_Tree_set_hposition(Fl_Tree *self, int pos) {
    LOCK(self->hposition(pos);)
}

int Fl_Tree_is_scrollbar(Fl_Tree *self, Fl_Widget *w) {
    return self->is_scrollbar(w);
}

int Fl_Tree_scrollbar_size(const Fl_Tree *self) {
    return self->scrollbar_size();
}

void Fl_Tree_set_scrollbar_size(Fl_Tree *self, int size) {
    LOCK(self->scrollbar_size(size);)
}

int Fl_Tree_is_vscroll_visible(const Fl_Tree *self) {
    return self->is_vscroll_visible();
}

int Fl_Tree_is_hscroll_visible(const Fl_Tree *self) {
    return self->is_hscroll_visible();
}

void Fl_Tree_set_callback_item(Fl_Tree *self, Fl_Tree_Item *item) {
    LOCK(self->callback_item(item);)
}

Fl_Tree_Item *Fl_Tree_callback_item(Fl_Tree *self) {
    return self->callback_item();
}

void Fl_Tree_set_callback_reason(Fl_Tree *self, int reason) {
    LOCK(self->callback_reason((Fl_Tree_Reason)reason);)
}

int Fl_Tree_callback_reason(const Fl_Tree *self) {
    return self->callback_reason();
}

// TreeItems

int Fl_Tree_Item_x(const Fl_Tree_Item *self) { return self->x(); }

int Fl_Tree_Item_y(const Fl_Tree_Item *self) { return self->y(); }

int Fl_Tree_Item_w(const Fl_Tree_Item *self) { return self->w(); }

int Fl_Tree_Item_h(const Fl_Tree_Item *self) { return self->h(); }

int Fl_Tree_Item_label_x(const Fl_Tree_Item *self) { return self->label_x(); }

int Fl_Tree_Item_label_y(const Fl_Tree_Item *self) { return self->label_y(); }

int Fl_Tree_Item_label_w(const Fl_Tree_Item *self) { return self->label_w(); }

int Fl_Tree_Item_label_h(const Fl_Tree_Item *self) { return self->label_h(); }

void Fl_Tree_Item_show_self(const Fl_Tree_Item *self, const char *indent) {
    LOCK(self->show_self(indent);)
}

void Fl_Tree_set_Item_label(Fl_Tree_Item *self, const char *val) {
    LOCK(self->label(val);)
}

const char *Fl_Tree_Item_label(const Fl_Tree_Item *self) { 
    auto label = self->label(); 
    char *buf = (char *)malloc(strlen(label) + 1);
    strncpy(buf, label, strlen(label) + 1);
    return buf;
}

void Fl_Tree_Item_set_labelfont(Fl_Tree_Item *self, int val) {
    LOCK(self->labelfont(val);)
}

int Fl_Tree_Item_labelfont(const Fl_Tree_Item *self) {
    return self->labelfont();
}

void Fl_Tree_Item_set_labelsize(Fl_Tree_Item *self, int val) {
    LOCK(self->labelsize(val);)
}

int Fl_Tree_Item_labelsize(const Fl_Tree_Item *self) {
    return self->labelsize();
}

void Fl_Tree_Item_set_labelfgcolor(Fl_Tree_Item *self, unsigned int val) {
    LOCK(self->labelfgcolor(val);)
}

unsigned int Fl_Tree_Item_labelfgcolor(const Fl_Tree_Item *self) {
    return self->labelfgcolor();
}

void Fl_Tree_Item_set_labelcolor(Fl_Tree_Item *self, unsigned int val) {
    LOCK(self->labelcolor(val);)
}

unsigned int Fl_Tree_Item_labelcolor(const Fl_Tree_Item *self) {
    return self->labelcolor();
}

void Fl_Tree_Item_set_labelbgcolor(Fl_Tree_Item *self, unsigned int val) {
    LOCK(self->labelbgcolor(val);)
}

unsigned int Fl_Tree_Item_labelbgcolor(const Fl_Tree_Item *self) {
    return self->labelbgcolor();
}

void Fl_Tree_Item_set_widget(Fl_Tree_Item *self, Fl_Widget *val) {
    LOCK(self->widget(val);)
}

Fl_Widget *Fl_Tree_Item_widget(const Fl_Tree_Item *self) {
    return self->widget();
}

int Fl_Tree_Item_children(const Fl_Tree_Item *self) { return self->children(); }

const Fl_Tree_Item *Fl_Tree_Item_child(const Fl_Tree_Item *self, int t) {
    return self->child(t);
}

int Fl_Tree_Item_has_children(const Fl_Tree_Item *self) {
    return self->has_children();
}

int Fl_Tree_Item_find_child(Fl_Tree_Item *self, const char *name) {
    return self->find_child(name);
}

int Fl_Tree_Item_remove_child(Fl_Tree_Item *self, const char *new_label) {
    return self->remove_child(new_label);
}

void Fl_Tree_Item_clear_children(Fl_Tree_Item *self) {
    LOCK(self->clear_children();)
}

int Fl_Tree_Item_swap_children(Fl_Tree_Item *self, Fl_Tree_Item *a,
                               Fl_Tree_Item *b) {
    return self->swap_children(a, b);
}

const Fl_Tree_Item *Fl_Tree_Item_find_child_item(const Fl_Tree_Item *self,
                                                 const char *name) {
    return self->find_child_item(name);
}

Fl_Tree_Item *Fl_Tree_Item_replace(Fl_Tree_Item *self, Fl_Tree_Item *new_item) {
    return self->replace(new_item);
}

Fl_Tree_Item *Fl_Tree_Item_replace_child(Fl_Tree_Item *self,
                                         Fl_Tree_Item *olditem,
                                         Fl_Tree_Item *newitem) {
    return self->replace_child(olditem, newitem);
}

Fl_Tree_Item *Fl_Tree_Item_deparent(Fl_Tree_Item *self, int index) {
    return self->deparent(index);
}

int Fl_Tree_Item_reparent(Fl_Tree_Item *self, Fl_Tree_Item *newchild,
                          int index) {
    return self->reparent(newchild, index);
}

int Fl_Tree_Item_move(Fl_Tree_Item *self, int to, int from) {
    return self->move(to, from);
}

int Fl_Tree_Item_move_above(Fl_Tree_Item *self, Fl_Tree_Item *item) {
    return self->move_above(item);
}

int Fl_Tree_Item_move_below(Fl_Tree_Item *self, Fl_Tree_Item *item) {
    return self->move_below(item);
}

int Fl_Tree_Item_move_into(Fl_Tree_Item *self, Fl_Tree_Item *item, int pos) {
    return self->move_into(item, pos);
}

int Fl_Tree_Item_depth(const Fl_Tree_Item *self) { return self->depth(); }

Fl_Tree_Item *Fl_Tree_Item_prev(Fl_Tree_Item *self) { return self->prev(); }

Fl_Tree_Item *Fl_Tree_Item_next(Fl_Tree_Item *self) { return self->next(); }

Fl_Tree_Item *Fl_Tree_Item_next_sibling(Fl_Tree_Item *self) {
    return self->next_sibling();
}

Fl_Tree_Item *Fl_Tree_Item_prev_sibling(Fl_Tree_Item *self) {
    return self->prev_sibling();
}

void Fl_Tree_Item_update_prev_next(Fl_Tree_Item *self, int index) {
    LOCK(return self->update_prev_next(index);)
}

const Fl_Tree_Item *Fl_Tree_Item_parent(const Fl_Tree_Item *self) {
    return self->parent();
}

void Fl_Tree_Item_set_parent(Fl_Tree_Item *self, Fl_Tree_Item *val) {
    LOCK(self->parent(val);)
}

const Fl_Tree *Fl_Tree_Item_tree(const Fl_Tree_Item *self) {
    return self->tree();
}

void Fl_Tree_Item_open(Fl_Tree_Item *self) { LOCK(self->open();) }

void Fl_Tree_Item_close(Fl_Tree_Item *self) { LOCK(self->close();) }

int Fl_Tree_Item_is_open(const Fl_Tree_Item *self) { return self->is_open(); }

int Fl_Tree_Item_is_close(const Fl_Tree_Item *self) { return self->is_close(); }

void Fl_Tree_Item_open_toggle(Fl_Tree_Item *self) { LOCK(self->open_toggle();) }

void Fl_Tree_Item_select(Fl_Tree_Item *self, int val) {
    LOCK(self->select(val);)
}

void Fl_Tree_Item_select_toggle(Fl_Tree_Item *self) {
    LOCK(self->select_toggle();)
}

int Fl_Tree_Item_select_all(Fl_Tree_Item *self) { return self->select_all(); }

void Fl_Tree_Item_deselect(Fl_Tree_Item *self) { LOCK(self->deselect();) }

int Fl_Tree_Item_deselect_all(Fl_Tree_Item *self) {
    return self->deselect_all();
}

int Fl_Tree_Item_is_root(const Fl_Tree_Item *self) { return self->is_root(); }

int Fl_Tree_Item_is_visible(const Fl_Tree_Item *self) {
    return self->is_visible();
}

char Fl_Tree_Item_is_active(const Fl_Tree_Item *self) {
    return self->is_active();
}

char Fl_Tree_Item_is_activated(const Fl_Tree_Item *self) {
    return self->is_activated();
}

void Fl_Tree_Item_deactivate(Fl_Tree_Item *self) { LOCK(self->deactivate();) }

void Fl_Tree_Item_activate(Fl_Tree_Item *self, int val) {
    LOCK(self->activate(val);)
}

char Fl_Tree_Item_is_selected(const Fl_Tree_Item *self) {
    return self->is_selected();
}

// TreeItemArray

int Fl_Tree_Item_Array_total(const Fl_Tree_Item_Array *self) {
    return self->total();
}

void Fl_Tree_Item_Array_swap(Fl_Tree_Item_Array *self, int ax, int bx) {
    return self->swap(ax, bx);
}

int Fl_Tree_Item_Array_move(Fl_Tree_Item_Array *self, int to, int from) {
    return self->move(to, from);
}

int Fl_Tree_Item_Array_deparent(Fl_Tree_Item_Array *self, int pos) {
    return self->deparent(pos);
}

int Fl_Tree_Item_Array_reparent(Fl_Tree_Item_Array *self, Fl_Tree_Item *item,
                                Fl_Tree_Item *newparent, int pos) {
    return self->reparent(item, newparent, pos);
}

void Fl_Tree_Item_Array_clear(Fl_Tree_Item_Array *self) {
    return self->clear();
}

void Fl_Tree_Item_Array_add(Fl_Tree_Item_Array *self, Fl_Tree_Item *val) {
    return self->add(val);
}

void Fl_Tree_Item_Array_insert(Fl_Tree_Item_Array *self, int pos,
                               Fl_Tree_Item *new_item) {
    return self->insert(pos, new_item);
}

void Fl_Tree_Item_Array_replace(Fl_Tree_Item_Array *self, int pos,
                                Fl_Tree_Item *new_item) {
    return self->replace(pos, new_item);
}

void Fl_Tree_Item_Array_remove(Fl_Tree_Item_Array *self, int index) {
    return self->remove(index);
}

int Fl_Tree_Item_Array_remove_item(Fl_Tree_Item_Array *self,
                                   Fl_Tree_Item *item) {
    return self->remove(item);
}

Fl_Tree_Item *Fl_Tree_Item_Array_at(Fl_Tree_Item_Array *self, int index) {
    auto total = self->total();
    if (index >= total) return NULL;
    return (*self)[index];
}

void Fl_Tree_Item_Array_delete(Fl_Tree_Item_Array *self) {
    delete self;
}
