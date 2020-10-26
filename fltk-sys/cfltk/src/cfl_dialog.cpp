#include "cfl_dialog.h"
#include <FL/Fl.H>

#include "cfl_new.hpp"
#include <FL/Fl_Color_Chooser.H>
#include <FL/Fl_File_Chooser.H>
#include <FL/Fl_Help_Dialog.H>
#include <FL/Fl_Image.H>
#include <FL/Fl_Native_File_Chooser.H>
#include <FL/fl_ask.H>
#include <FL/platform.H>
#include <string.h>

#ifndef LOCK
#define LOCK(x)                                                                                    \
    Fl::lock();                                                                                    \
    x;                                                                                             \
    Fl::unlock();                                                                                  \
    Fl::awake();
#endif

Fl_Native_File_Chooser *Fl_Native_File_Chooser_new(int val) {
#ifndef __ANDROID__
    return new Fl_Native_File_Chooser(val);
#else
    return NULL;
#endif
}

void Fl_Native_File_Chooser_delete(Fl_Native_File_Chooser *self) {
    delete self;
}

const char *Fl_Native_File_Chooser_filename(Fl_Native_File_Chooser *self) {
    const char *x = self->filename();
    if (!strcmp(x, ""))
        return NULL;
    else
        return x;
}

const char *Fl_Native_File_Chooser_filenames(Fl_Native_File_Chooser *self, int cnt) {
    const char *x = self->filename(cnt);
    if (!strcmp(x, ""))
        return NULL;
    else
        return x;
}

int Fl_Native_File_Chooser_count(Fl_Native_File_Chooser *self) {
    return self->count();
}

void Fl_Native_File_Chooser_set_directory(Fl_Native_File_Chooser *self, const char *val) {
    LOCK(self->directory(val);)
}

const char *Fl_Native_File_Chooser_directory(Fl_Native_File_Chooser *self) {
    return self->directory();
}

int Fl_Native_File_Chooser_show(Fl_Native_File_Chooser *self) {
    int ret = 0;
    LOCK(fl_open_display(); ret = self->show());
    return ret;
}

void Fl_Native_File_Chooser_set_option(Fl_Native_File_Chooser *self, int opt) {
    LOCK(self->options(opt);)
}

void Fl_Native_File_Chooser_set_type(Fl_Native_File_Chooser *self, int typ) {
    LOCK(self->type(typ);)
}

void Fl_Native_File_Chooser_set_title(Fl_Native_File_Chooser *self, const char *title) {
    LOCK(self->title(title);)
}

void Fl_Native_File_Chooser_set_filter(Fl_Native_File_Chooser *self, const char *f) {
    LOCK(self->filter(f);)
}

void Fl_Native_File_Chooser_set_preset_file(Fl_Native_File_Chooser *self, const char *f) {
    LOCK(self->preset_file(f);)
}

const char *Fl_Native_File_Chooser_errmsg(Fl_Native_File_Chooser *self) {
    return self->errmsg();
}

void Fl_message(int x, int y, const char *txt) {
    fl_message_position(x, y, 0);
    fl_message("%s", txt);
}

void Fl_alert(int x, int y, const char *txt) {
    fl_message_position(x, y, 0);
    fl_alert("%s", txt);
}

int Fl_choice(int x, int y, const char *txt, const char *b0, const char *b1, const char *b2) {
    fl_message_position(x, y, 0);
    if (strlen(b2) == 0)
        b2 = NULL;
    return fl_choice("%s", b0, b1, b2, txt);
}

const char *Fl_input(int x, int y, const char *txt, const char *deflt) {
    fl_message_position(x, y, 0);
    return fl_input("%s", deflt, txt);
}

const char *Fl_password(int x, int y, const char *txt, const char *deflt) {
    fl_message_position(x, y, 0);
    return fl_password("%s", deflt, txt);
}

Fl_Help_Dialog *Fl_Help_Dialog_new(void) {
    return new Fl_Help_Dialog();
}

void Fl_Help_Dialog_delete(Fl_Help_Dialog *self) {
    delete self;
}

int Fl_Help_Dialog_h(Fl_Help_Dialog *self) {
    return self->h();
}

void Fl_Help_Dialog_hide(Fl_Help_Dialog *self) {
    return self->hide();
}

int Fl_Help_Dialog_load(Fl_Help_Dialog *self, const char *f) {
    int ret = 0;
    LOCK(ret = self->load(f));
    return ret;
}

void Fl_Help_Dialog_position(Fl_Help_Dialog *self, int xx, int yy) {
    LOCK(self->position(xx, yy);)
}

void Fl_Help_Dialog_resize(Fl_Help_Dialog *self, int xx, int yy, int ww, int hh) {
    LOCK(self->resize(xx, yy, ww, hh);)
}

void Fl_Help_Dialog_show(Fl_Help_Dialog *self) {
    LOCK(self->show();)
}

void Fl_Help_Dialog_set_text_size(Fl_Help_Dialog *self, int s) {
    LOCK(self->textsize(s);)
}

int Fl_Help_Dialog_text_size(Fl_Help_Dialog *self) {
    return self->textsize();
}

void Fl_Help_Dialog_set_value(Fl_Help_Dialog *self, const char *f) {
    LOCK(self->value(f);)
}

const char *Fl_Help_Dialog_value(const Fl_Help_Dialog *self) {
    return self->value();
}

int Fl_Help_Dialog_visible(Fl_Help_Dialog *self) {
    return self->visible();
}

int Fl_Help_Dialog_w(Fl_Help_Dialog *self) {
    return self->w();
}

int Fl_Help_Dialog_x(Fl_Help_Dialog *self) {
    return self->x();
}

int Fl_Help_Dialog_y(Fl_Help_Dialog *self) {
    return self->y();
}

void Fl_beep(int type) {
    fl_beep(type);
}

Fl_File_Chooser *Fl_File_Chooser_new(const char *d, const char *p, int t, const char *title) {
    return new Fl_File_Chooser(d, p, t, title);
}

void Fl_File_Chooser_delete(Fl_File_Chooser *self) {
    delete self;
}

void *Fl_File_Chooser_newButton(Fl_File_Chooser *self) {
    return self->newButton;
}

void *Fl_File_Chooser_previewButton(Fl_File_Chooser *self) {
    return self->previewButton;
}

void *Fl_File_Chooser_showHiddenButton(Fl_File_Chooser *self) {
    return self->showHiddenButton;
}

void Fl_File_Chooser_set_callback(Fl_File_Chooser *self, void (*cb)(Fl_File_Chooser *, void *),
                              void *d) {
    LOCK(self->callback(cb, d);)
}

void Fl_File_Chooser_set_color(Fl_File_Chooser *self, unsigned int c) {
    LOCK(self->color(c);)
}

unsigned int Fl_File_Chooser_color(Fl_File_Chooser *self) {
    return self->color();
}

int Fl_File_Chooser_count(Fl_File_Chooser *self) {
    return self->count();
}

void Fl_File_Chooser_set_directory(Fl_File_Chooser *self, const char *d) {
    LOCK(self->directory(d);)
}

char *Fl_File_Chooser_directory(Fl_File_Chooser *self) {
    return self->directory();
}

void Fl_File_Chooser_set_filter(Fl_File_Chooser *self, const char *p) {
    LOCK(self->filter(p);)
}

const char *Fl_File_Chooser_filter(Fl_File_Chooser *self) {
    return self->filter();
}

int Fl_File_Chooser_filter_value(Fl_File_Chooser *self) {
    return self->filter_value();
}

void Fl_File_Chooser_set_filter_value(Fl_File_Chooser *self, int f) {
    LOCK(self->filter_value(f);)
}

void Fl_File_Chooser_hide(Fl_File_Chooser *self) {
    LOCK(self->hide();)
}

void Fl_File_Chooser_set_iconsize(Fl_File_Chooser *self, unsigned char s) {
    LOCK(self->iconsize(s);)
}

unsigned char Fl_File_Chooser_iconsize(Fl_File_Chooser *self) {
    return self->iconsize();
}

void Fl_File_Chooser_set_label(Fl_File_Chooser *self, const char *l) {
    LOCK(self->label(l);)
}

const char *Fl_File_Chooser_label(Fl_File_Chooser *self) {
    return self->label();
}

void Fl_File_Chooser_set_ok_label(Fl_File_Chooser *self, const char *l) {
    LOCK(self->ok_label(l);)
}

const char *Fl_File_Chooser_ok_label(Fl_File_Chooser *self) {
    return self->ok_label();
}

void Fl_File_Chooser_set_preview(Fl_File_Chooser *self, int e) {
    LOCK(self->preview(e);)
}

int Fl_File_Chooser_preview(const Fl_File_Chooser *self) {
    return self->preview();
}

void Fl_File_Chooser_rescan(Fl_File_Chooser *self) {
    LOCK(self->rescan();)
}

void Fl_File_Chooser_rescan_keep_filename(Fl_File_Chooser *self) {
    LOCK(self->rescan_keep_filename();)
}

void Fl_File_Chooser_show(Fl_File_Chooser *self) {
    LOCK(self->show();)
}

int Fl_File_Chooser_shown(Fl_File_Chooser *self) {
    return self->shown();
}

void Fl_File_Chooser_set_textcolor(Fl_File_Chooser *self, unsigned int c) {
    LOCK(self->textcolor(c);)
}

unsigned int Fl_File_Chooser_textcolor(Fl_File_Chooser *self) {
    return self->textcolor();
}

void Fl_File_Chooser_set_textfont(Fl_File_Chooser *self, int f) {
    LOCK(self->textfont(f);)
}

int Fl_File_Chooser_textfont(Fl_File_Chooser *self) {
    return self->textfont();
}

void Fl_File_Chooser_set_textsize(Fl_File_Chooser *self, int s) {
    LOCK(self->textsize(s);)
}

int Fl_File_Chooser_textsize(Fl_File_Chooser *self) {
    return self->textsize();
}

void Fl_File_Chooser_set_type(Fl_File_Chooser *self, int t) {
    LOCK(self->type(t);)
}

int Fl_File_Chooser_type(Fl_File_Chooser *self) {
    return self->type();
}

void *Fl_File_Chooser_user_data(const Fl_File_Chooser *self) {
    return self->user_data();
}

void Fl_File_Chooser_set_user_data(Fl_File_Chooser *self, void *d) {
    LOCK(self->user_data(d);)
}

const char *Fl_File_Chooser_value(Fl_File_Chooser *self, int f) {
    return self->value(f);
}

void Fl_File_Chooser_set_value(Fl_File_Chooser *self, const char *filename) {
    LOCK(self->value(filename);)
}

int Fl_File_Chooser_visible(Fl_File_Chooser *self) {
    return self->visible();
}

char *Fl_dir_chooser(const char *message, const char *fname, int relative) {
    return fl_dir_chooser(message, fname, relative);
}

char *Fl_file_chooser(const char *message, const char *pat, const char *fname, int relative) {
    return fl_file_chooser(message, pat, fname, relative);
}

int Fl_color_chooser(const char *name, unsigned char *r, unsigned char *g, unsigned char *b,
                     int cmode) {
    return fl_color_chooser(name, *r, *g, *b, cmode);
}

#undef LOCK