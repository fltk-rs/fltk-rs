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

void show_self(Fl_Tree *self) { LOCK(self->show_self();) }

void root_label(Fl_Tree *self, const char *new_label) {
    LOCK(self->root_label(new_label);)
}

Fl_Tree_Item *root(Fl_Tree *self) { return self->root(); }

void set_root(Fl_Tree *self, Fl_Tree_Item *newitem) { LOCK(self->root(newitem);) }

Fl_Tree_Item *add(Fl_Tree *self, const char *name) {
    Fl_Tree_Item *ret; LOCK(ret = self->add(name)); return ret;
}

Fl_Tree_Item *insert_above(Fl_Tree *self, Fl_Tree_Item *above,
                           const char *name) {
    Fl_Tree_Item *ret; LOCK(self->insert_above(above, name)); return ret;
}

Fl_Tree_Item *insert(Fl_Tree *self, Fl_Tree_Item *item, const char *name,
                     int pos) {
    Fl_Tree_Item *ret; LOCK(self->insert(item, name, pos)); return ret;
}

const Fl_Tree_Item *find_item(const Fl_Tree *self, const char *path) {
    if (!path || strlen(path) == 0) return NULL;
    const Fl_Tree_Item *item = self->find_item(path); return item;
}                 

Fl_Tree_Item *find_item_mut(Fl_Tree *self, const char *path) {
    if (!path || strlen(path) == 0) return NULL;
    Fl_Tree_Item *item = self->find_item(path); return item;
}

int Fl_Tree_remove(Fl_Tree *self, Fl_Tree_Item *item) { int ret; LOCK(ret = self->remove(item)); return ret; }

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

int open(Fl_Tree *self, const char *path, int docallback) {
    if (!path) return 0;
    int ret;
    LOCK(ret = self->open(path, docallback));
    return ret;
}

void open_toggle(Fl_Tree *self, Fl_Tree_Item *item, int docallback) {
    LOCK(self->open_toggle(item, docallback);)
}

int close(Fl_Tree *self, const char *path, int docallback) {
    int ret; LOCK(self->close(path, 1)); return ret;
}

int is_open(const Fl_Tree *self, const char *path) {
    return self->is_open(path);
}

int is_close(const Fl_Tree *self, const char *path) {
    return self->is_close(path);
}

int Fl_Tree_select(Fl_Tree *self, const char *path, int docallback) {
    int ret; LOCK( ret = self->select(path, docallback)); return ret;
}

void select_toggle(Fl_Tree *self, Fl_Tree_Item *item, int docallback) {
    LOCK(self->select_toggle(item, docallback);)
}

int deselect(Fl_Tree *self, const char *path, int docallback) {
    int ret; LOCK(ret = self->deselect(path, docallback)); return ret;
}

int deselect_all(Fl_Tree *self, Fl_Tree_Item *item, int docallback) {
    int ret; LOCK(ret = self->deselect_all(item, docallback)); return ret;
}

int select_only(Fl_Tree *self, Fl_Tree_Item *selitem, int docallback) {
    int ret; LOCK(ret = self->select_only(selitem, docallback)); return ret;
}

int select_all(Fl_Tree *self, Fl_Tree_Item *item, int docallback) {
    int ret; LOCK(ret = self->select_all(item, docallback)); return ret;
}

int extend_selection_dir(Fl_Tree *self, Fl_Tree_Item *from, Fl_Tree_Item *to,
                         int dir, int val, int visible) {
    int ret; LOCK(ret= self->extend_selection_dir(from, to, dir, val, visible)); return ret;
}

int extend_selection(Fl_Tree *self, Fl_Tree_Item *from, Fl_Tree_Item *to,
                     int val, int visible) {
    int ret; LOCK(ret= self->extend_selection(from, to, val, visible)); return ret;
}

void set_item_focus(Fl_Tree *self, Fl_Tree_Item *item) {
    LOCK(self->set_item_focus(item);)
}

Fl_Tree_Item *get_item_focus(const Fl_Tree *self) {
    return self->get_item_focus();
}

int is_selected(Fl_Tree *self, const char *path) {
    return self->is_selected(path);
}

int item_labelfont(const Fl_Tree *self) { return self->item_labelfont(); }

void set_item_labelfont(Fl_Tree *self, int val) { LOCK(self->item_labelfont(val);) }

int item_labelsize(const Fl_Tree *self) { return self->item_labelsize(); }

void set_item_labelsize(Fl_Tree *self, int val) { LOCK(self->item_labelsize(val);) }

unsigned int item_labelfgcolor(const Fl_Tree *self) {
    return self->item_labelfgcolor();
}

void set_item_labelfgcolor(Fl_Tree *self, unsigned int val) {
    LOCK(self->item_labelfgcolor(val);)
}

unsigned int item_labelbgcolor(const Fl_Tree *self) {
    return self->item_labelbgcolor();
}

void set_item_labelbgcolor(Fl_Tree *self, unsigned int val) {
    LOCK(self->item_labelbgcolor(val);)
}

unsigned int connectorcolor(const Fl_Tree *self) {
    return self->connectorcolor();
}

void set_connectorcolor(Fl_Tree *self, unsigned int val) {
    LOCK(self->connectorcolor(val);)
}

int marginleft(const Fl_Tree *self) { return self->marginleft(); }

void set_marginleft(Fl_Tree *self, int val) { LOCK(self->marginleft(val);) }

int margintop(const Fl_Tree *self) { return self->margintop(); }

void set_margintop(Fl_Tree *self, int val) { LOCK(self->margintop(val);) }

int marginbottom(const Fl_Tree *self) { return self->marginbottom(); }

void set_marginbottom(Fl_Tree *self, int val) { LOCK(self->marginbottom(val);) }

int linespacing(const Fl_Tree *self) { return self->linespacing(); }

void set_linespacing(Fl_Tree *self, int val) { LOCK(self->linespacing(val);) }

int openchild_marginbottom(const Fl_Tree *self) {
    return self->openchild_marginbottom();
}

void set_openchild_marginbottom(Fl_Tree *self, int val) {
    LOCK(self->openchild_marginbottom(val);)
}

int usericonmarginleft(const Fl_Tree *self) {
    return self->usericonmarginleft();
}

void set_usericonmarginleft(Fl_Tree *self, int val) {
    LOCK(self->usericonmarginleft(val);)
}

int labelmarginleft(const Fl_Tree *self) { return self->labelmarginleft(); }

void set_labelmarginleft(Fl_Tree *self, int val) { LOCK(self->labelmarginleft(val);) }

int widgetmarginleft(const Fl_Tree *self) { return self->widgetmarginleft(); }

void set_widgetmarginleft(Fl_Tree *self, int val) {
   LOCK( self->widgetmarginleft(val);)
}

int connectorwidth(const Fl_Tree *self) { return self->connectorwidth(); }

void set_connectorwidth(Fl_Tree *self, int val) { LOCK(self->connectorwidth(val);) }

void *usericon(const Fl_Tree *self) { return self->usericon(); }

void set_usericon(Fl_Tree *self, void *val) { LOCK(self->usericon((Fl_Image *)val);) }

void *openicon(const Fl_Tree *self) { return self->openicon(); }

void set_openicon(Fl_Tree *self, void *val) { LOCK(self->openicon((Fl_Image *)val);) }

void *closeicon(const Fl_Tree *self) { return self->closeicon(); }

void set_closeicon(Fl_Tree *self, void *val) {
    LOCK(self->closeicon((Fl_Image *)val);)
}

int showcollapse(const Fl_Tree *self) { return self->showcollapse(); }

void set_showcollapse(Fl_Tree *self, int val) { LOCK(self->showcollapse(val);) }

int showroot(const Fl_Tree *self) { return self->showroot(); }

void set_showroot(Fl_Tree *self, int val) { LOCK(self->showroot(val);) }

int connectorstyle(const Fl_Tree *self) { return self->connectorstyle(); }

void set_connectorstyle(Fl_Tree *self, int val) {
    LOCK(self->connectorstyle((Fl_Tree_Connector)val);)
}

int sortorder(const Fl_Tree *self) { return self->sortorder(); }

void set_sortorder(Fl_Tree *self, int val) {
    LOCK(self->sortorder((Fl_Tree_Sort)val);)
}

int selectbox(const Fl_Tree *self) { return self->selectbox(); }

void set_selectbox(Fl_Tree *self, int val) { LOCK(self->selectbox((Fl_Boxtype)val);) }

int selectmode(const Fl_Tree *self) { return self->selectmode(); }

void set_selectmode(Fl_Tree *self, int val) {
    LOCK(self->selectmode((Fl_Tree_Select)val);)
}

int item_reselect_mode(const Fl_Tree *self) {
    return self->item_reselect_mode();
}

void set_item_reselect_mode(Fl_Tree *self, int mode) {
    LOCK(self->item_reselect_mode((Fl_Tree_Item_Reselect_Mode)mode);)
}

int item_draw_mode(const Fl_Tree *self) { return self->item_draw_mode(); }

void set_item_draw_mode(Fl_Tree *self, int mode) { LOCK(self->item_draw_mode(mode);) }

void calc_dimensions(Fl_Tree *self) { LOCK(self->calc_dimensions();) }

void calc_tree(Fl_Tree *self) { LOCK(self->calc_tree();) }

void recalc_tree(Fl_Tree *self) { LOCK(self->recalc_tree();) }

int displayed(Fl_Tree *self, Fl_Tree_Item *item) {
    int ret; LOCK(ret=  self->displayed(item)); return ret;
}

void show_item(Fl_Tree *self, Fl_Tree_Item *item, int yoff) {
    LOCK(self->show_item(item, yoff);)
}

void show_item_top(Fl_Tree *self, Fl_Tree_Item *item) {
    LOCK(self->show_item_top(item);)
}

void show_item_middle(Fl_Tree *self, Fl_Tree_Item *item) {
    LOCK(self->show_item_middle(item);)
}

void show_item_bottom(Fl_Tree *self, Fl_Tree_Item *item) {
    LOCK(self->show_item_bottom(item);)
}

void display(Fl_Tree *self, Fl_Tree_Item *item) { LOCK(self->display(item);) }

int vposition(const Fl_Tree *self) { return self->vposition(); }

void set_vposition(Fl_Tree *self, int pos) { LOCK(self->vposition(pos);) }

int hposition(const Fl_Tree *self) { return self->hposition(); }

void set_hposition(Fl_Tree *self, int pos) { LOCK(self->hposition(pos);) }

int is_scrollbar(Fl_Tree *self, Fl_Widget *w) { return self->is_scrollbar(w); }

int scrollbar_size(const Fl_Tree *self) { return self->scrollbar_size(); }

void set_scrollbar_size(Fl_Tree *self, int size) { LOCK(self->scrollbar_size(size);) }

int is_vscroll_visible(const Fl_Tree *self) {
    return self->is_vscroll_visible();
}

int is_hscroll_visible(const Fl_Tree *self) {
    return self->is_hscroll_visible();
}

void set_callback_item(Fl_Tree *self, Fl_Tree_Item *item) {
    LOCK(self->callback_item(item);)
}

Fl_Tree_Item *callback_item(Fl_Tree *self) { return self->callback_item(); }

void set_callback_reason(Fl_Tree *self, int reason) {
    LOCK(self->callback_reason((Fl_Tree_Reason)reason);)
}

int callback_reason(const Fl_Tree *self) { return self->callback_reason(); }
