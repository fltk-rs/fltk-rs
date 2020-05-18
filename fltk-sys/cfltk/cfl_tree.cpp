#include "cfl_tree.h"
#include <FL/Fl.H>
#include <FL/Fl_Image.H>
#include <FL/Fl_Tree.H>
#include <FL/Fl_Tree_Item.H>
#include <FL/Fl_Tree_Item_Array.H>
#include <FL/Fl_Widget.H>
#include <new>

WIDGET_DEFINE(Fl_Tree)

void Fl_Tree_begin(Fl_Tree *self) {
    LOCK(self->begin();)
}      
                                 
void Fl_Tree_end(Fl_Tree *self) {
    LOCK(self->end();)
}    

void Fl_Tree_show_self(Fl_Tree *self) { LOCK(self->show_self();) }

void Fl_Tree_root_label(Fl_Tree *self, const char *new_label) {
    LOCK(self->root_label(new_label);)
}

Fl_Tree_Item *Fl_Tree_root(Fl_Tree *self) { return self->root(); }

void Fl_Tree_set_root(Fl_Tree *self, Fl_Tree_Item *newitem) { LOCK(self->root(newitem);) }

Fl_Tree_Item *Fl_Tree_add(Fl_Tree *self, const char *name) {
    Fl_Tree_Item *ret; LOCK(ret = self->add(name)); return ret;
}

Fl_Tree_Item *Fl_Tree_insert_above(Fl_Tree *self, Fl_Tree_Item *above,
                           const char *name) {
    Fl_Tree_Item *ret; LOCK(self->insert_above(above, name)); return ret;
}

Fl_Tree_Item *Fl_Tree_insert(Fl_Tree *self, Fl_Tree_Item *item, const char *name,
                     int pos) {
    Fl_Tree_Item *ret; LOCK(self->insert(item, name, pos)); return ret;
}

const Fl_Tree_Item *Fl_Tree_find_item(const Fl_Tree *self, const char *path) {
    if (!path || strlen(path) == 0) return NULL;
    const Fl_Tree_Item *item = self->find_item(path); return item;
}                 

Fl_Tree_Item *Fl_Tree_find_item_mut(Fl_Tree *self, const char *path) {
    if (!path || strlen(path) == 0) return NULL;
    Fl_Tree_Item *item = self->find_item(path); return item;
}

int Fl_Tree_remove(Fl_Tree *self, Fl_Tree_Item *item) { int ret; LOCK(ret = self->remove(item)); return ret; }

void Fl_Tree_clear(Fl_Tree *self) { self->clear(); }

void Fl_Tree_clear_children(Fl_Tree *self, Fl_Tree_Item *item) {
    self->clear_children(item);
}

const Fl_Tree_Item *Fl_Tree_find_clicked(const Fl_Tree *self, int yonly) {
    return self->find_clicked(yonly);
}

Fl_Tree_Item *Fl_Tree_item_clicked(Fl_Tree *self) { return self->item_clicked(); }

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

Fl_Tree_Item *Fl_Tree_next_visible_item(Fl_Tree *self, Fl_Tree_Item *start, int dir) {
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

Fl_Tree_Item *Fl_Tree_next_selected_item(Fl_Tree *self, Fl_Tree_Item *item, int dir) {
    return self->next_selected_item(item, dir);
}

int Fl_Tree_get_selected_items(Fl_Tree *self, Fl_Tree_Item **items, int *cnt) {
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

int Fl_Tree_open(Fl_Tree *self, const char *path, int docallback) {
    if (!path) return 0;
    int ret;
    LOCK(ret = self->open(path, docallback));
    return ret;
}

void Fl_Tree_open_toggle(Fl_Tree *self, Fl_Tree_Item *item, int docallback) {
    LOCK(self->open_toggle(item, docallback);)
}

int Fl_Tree_close(Fl_Tree *self, const char *path, int docallback) {
    int ret; LOCK(self->close(path, 1)); return ret;
}

int Fl_Tree_is_open(const Fl_Tree *self, const char *path) {
    return self->is_open(path);
}

int Fl_Tree_is_close(const Fl_Tree *self, const char *path) {
    return self->is_close(path);
}

int Fl_Tree_select(Fl_Tree *self, const char *path, int docallback) {
    int ret; LOCK( ret = self->select(path, docallback)); return ret;
}

void Fl_Tree_select_toggle(Fl_Tree *self, Fl_Tree_Item *item, int docallback) {
    LOCK(self->select_toggle(item, docallback);)
}

int Fl_Tree_deselect(Fl_Tree *self, const char *path, int docallback) {
    int ret; LOCK(ret = self->deselect(path, docallback)); return ret;
}

int Fl_Tree_deselect_all(Fl_Tree *self, Fl_Tree_Item *item, int docallback) {
    int ret; LOCK(ret = self->deselect_all(item, docallback)); return ret;
}

int Fl_Tree_select_only(Fl_Tree *self, Fl_Tree_Item *selitem, int docallback) {
    int ret; LOCK(ret = self->select_only(selitem, docallback)); return ret;
}

int Fl_Tree_select_all(Fl_Tree *self, Fl_Tree_Item *item, int docallback) {
    int ret; LOCK(ret = self->select_all(item, docallback)); return ret;
}

int Fl_Tree_extend_selection_dir(Fl_Tree *self, Fl_Tree_Item *from, Fl_Tree_Item *to,
                         int dir, int val, int visible) {
    int ret; LOCK(ret= self->extend_selection_dir(from, to, dir, val, visible)); return ret;
}

int Fl_Tree_extend_selection(Fl_Tree *self, Fl_Tree_Item *from, Fl_Tree_Item *to,
                     int val, int visible) {
    int ret; LOCK(ret= self->extend_selection(from, to, val, visible)); return ret;
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

int Fl_Tree_item_labelfont(const Fl_Tree *self) { return self->item_labelfont(); }

void Fl_Tree_set_item_labelfont(Fl_Tree *self, int val) { LOCK(self->item_labelfont(val);) }

int Fl_Tree_item_labelsize(const Fl_Tree *self) { return self->item_labelsize(); }

void Fl_Tree_set_item_labelsize(Fl_Tree *self, int val) { LOCK(self->item_labelsize(val);) }

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

void Fl_Tree_set_marginleft(Fl_Tree *self, int val) { LOCK(self->marginleft(val);) }

int Fl_Tree_margintop(const Fl_Tree *self) { return self->margintop(); }

void Fl_Tree_set_margintop(Fl_Tree *self, int val) { LOCK(self->margintop(val);) }

int Fl_Tree_marginbottom(const Fl_Tree *self) { return self->marginbottom(); }

void Fl_Tree_set_marginbottom(Fl_Tree *self, int val) { LOCK(self->marginbottom(val);) }

int Fl_Tree_linespacing(const Fl_Tree *self) { return self->linespacing(); }

void Fl_Tree_set_linespacing(Fl_Tree *self, int val) { LOCK(self->linespacing(val);) }

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

int Fl_Tree_labelmarginleft(const Fl_Tree *self) { return self->labelmarginleft(); }

void Fl_Tree_set_labelmarginleft(Fl_Tree *self, int val) { LOCK(self->labelmarginleft(val);) }

int Fl_Tree_widgetmarginleft(const Fl_Tree *self) { return self->widgetmarginleft(); }

void Fl_Tree_set_widgetmarginleft(Fl_Tree *self, int val) {
   LOCK( self->widgetmarginleft(val);)
}

int Fl_Tree_connectorwidth(const Fl_Tree *self) { return self->connectorwidth(); }

void Fl_Tree_set_connectorwidth(Fl_Tree *self, int val) { LOCK(self->connectorwidth(val);) }

void *Fl_Tree_usericon(const Fl_Tree *self) { return self->usericon(); }

void Fl_Tree_set_usericon(Fl_Tree *self, void *val) { LOCK(self->usericon((Fl_Image *)val);) }

void *Fl_Tree_openicon(const Fl_Tree *self) { return self->openicon(); }

void Fl_Tree_set_openicon(Fl_Tree *self, void *val) { LOCK(self->openicon((Fl_Image *)val);) }

void *Fl_Tree_closeicon(const Fl_Tree *self) { return self->closeicon(); }

void Fl_Tree_set_closeicon(Fl_Tree *self, void *val) {
    LOCK(self->closeicon((Fl_Image *)val);)
}

int Fl_Tree_showcollapse(const Fl_Tree *self) { return self->showcollapse(); }

void Fl_Tree_set_showcollapse(Fl_Tree *self, int val) { LOCK(self->showcollapse(val);) }

int Fl_Tree_showroot(const Fl_Tree *self) { return self->showroot(); }

void Fl_Tree_set_showroot(Fl_Tree *self, int val) { LOCK(self->showroot(val);) }

int Fl_Tree_connectorstyle(const Fl_Tree *self) { return self->connectorstyle(); }

void Fl_Tree_set_connectorstyle(Fl_Tree *self, int val) {
    LOCK(self->connectorstyle((Fl_Tree_Connector)val);)
}

int Fl_Tree_sortorder(const Fl_Tree *self) { return self->sortorder(); }

void Fl_Tree_set_sortorder(Fl_Tree *self, int val) {
    LOCK(self->sortorder((Fl_Tree_Sort)val);)
}

int Fl_Tree_selectbox(const Fl_Tree *self) { return self->selectbox(); }

void Fl_Tree_set_selectbox(Fl_Tree *self, int val) { LOCK(self->selectbox((Fl_Boxtype)val);) }

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

int Fl_Tree_item_draw_mode(const Fl_Tree *self) { return self->item_draw_mode(); }

void Fl_Tree_set_item_draw_mode(Fl_Tree *self, int mode) { LOCK(self->item_draw_mode(mode);) }

void Fl_Tree_calc_dimensions(Fl_Tree *self) { LOCK(self->calc_dimensions();) }

void Fl_Tree_calc_tree(Fl_Tree *self) { LOCK(self->calc_tree();) }

void Fl_Tree_recalc_tree(Fl_Tree *self) { LOCK(self->recalc_tree();) }

int Fl_Tree_displayed(Fl_Tree *self, Fl_Tree_Item *item) {
    int ret; LOCK(ret=  self->displayed(item)); return ret;
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

void Fl_Tree_display(Fl_Tree *self, Fl_Tree_Item *item) { LOCK(self->display(item);) }

int Fl_Tree_vposition(const Fl_Tree *self) { return self->vposition(); }

void Fl_Tree_set_vposition(Fl_Tree *self, int pos) { LOCK(self->vposition(pos);) }

int Fl_Tree_hposition(const Fl_Tree *self) { return self->hposition(); }

void Fl_Tree_set_hposition(Fl_Tree *self, int pos) { LOCK(self->hposition(pos);) }

int Fl_Tree_is_scrollbar(Fl_Tree *self, Fl_Widget *w) { return self->is_scrollbar(w); }

int Fl_Tree_scrollbar_size(const Fl_Tree *self) { return self->scrollbar_size(); }

void Fl_Tree_set_scrollbar_size(Fl_Tree *self, int size) { LOCK(self->scrollbar_size(size);) }

int Fl_Tree_is_vscroll_visible(const Fl_Tree *self) {
    return self->is_vscroll_visible();
}

int Fl_Tree_is_hscroll_visible(const Fl_Tree *self) {
    return self->is_hscroll_visible();
}

void Fl_Tree_set_callback_item(Fl_Tree *self, Fl_Tree_Item *item) {
    LOCK(self->callback_item(item);)
}

Fl_Tree_Item *Fl_Tree_callback_item(Fl_Tree *self) { return self->callback_item(); }

void Fl_Tree_set_callback_reason(Fl_Tree *self, int reason) {
    LOCK(self->callback_reason((Fl_Tree_Reason)reason);)
}

int Fl_Tree_callback_reason(const Fl_Tree *self) { return self->callback_reason(); }
