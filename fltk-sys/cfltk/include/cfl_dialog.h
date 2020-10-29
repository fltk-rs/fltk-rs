#pragma once

#ifdef __cplusplus
extern "C" {
#endif

typedef struct Fl_Native_File_Chooser Fl_Native_File_Chooser;

Fl_Native_File_Chooser *Fl_Native_File_Chooser_new(int);

void Fl_Native_File_Chooser_delete(Fl_Native_File_Chooser *self);

const char *Fl_Native_File_Chooser_filename(Fl_Native_File_Chooser *);

const char *Fl_Native_File_Chooser_filenames(Fl_Native_File_Chooser *, int);

int Fl_Native_File_Chooser_count(Fl_Native_File_Chooser *);

void Fl_Native_File_Chooser_set_directory(Fl_Native_File_Chooser *, const char *val);

const char *Fl_Native_File_Chooser_directory(Fl_Native_File_Chooser *);

int Fl_Native_File_Chooser_show(Fl_Native_File_Chooser *);

void Fl_Native_File_Chooser_set_option(Fl_Native_File_Chooser *, int opt);

void Fl_Native_File_Chooser_set_type(Fl_Native_File_Chooser *, int typ);

void Fl_Native_File_Chooser_set_title(Fl_Native_File_Chooser *, const char *title);

void Fl_Native_File_Chooser_set_filter(Fl_Native_File_Chooser *, const char *f);

void Fl_Native_File_Chooser_set_preset_file(Fl_Native_File_Chooser *, const char *f);

const char *Fl_Native_File_Chooser_errmsg(Fl_Native_File_Chooser *);

void Fl_message(int x, int y, const char *txt);

void Fl_alert(int x, int y, const char *txt);

int Fl_choice(int x, int y, const char *txt, const char *b0, const char *b1, const char *b2);

const char *Fl_input(int x, int y, const char *txt, const char *deflt);

const char *Fl_password(int x, int y, const char *txt, const char *deflt);

typedef struct Fl_Help_Dialog Fl_Help_Dialog;

Fl_Help_Dialog *Fl_Help_Dialog_new(void);

void Fl_Help_Dialog_delete(Fl_Help_Dialog *self);

int Fl_Help_Dialog_h(Fl_Help_Dialog *);

void Fl_Help_Dialog_hide(Fl_Help_Dialog *);

int Fl_Help_Dialog_load(Fl_Help_Dialog *, const char *f);

void Fl_Help_Dialog_position(Fl_Help_Dialog *, int xx, int yy);

void Fl_Help_Dialog_resize(Fl_Help_Dialog *, int xx, int yy, int ww, int hh);

void Fl_Help_Dialog_show(Fl_Help_Dialog *);

void Fl_Help_Dialog_set_text_size(Fl_Help_Dialog *, int s);

int Fl_Help_Dialog_text_size(Fl_Help_Dialog *);

void Fl_Help_Dialog_set_value(Fl_Help_Dialog *, const char *f);

const char *Fl_Help_Dialog_value(const Fl_Help_Dialog *);

int Fl_Help_Dialog_visible(Fl_Help_Dialog *);

int Fl_Help_Dialog_w(Fl_Help_Dialog *);

int Fl_Help_Dialog_x(Fl_Help_Dialog *);

int Fl_Help_Dialog_y(Fl_Help_Dialog *);

void Fl_beep(int type);

typedef struct Fl_File_Chooser Fl_File_Chooser;

Fl_File_Chooser *Fl_File_Chooser_new(const char *d, const char *p, int t, const char *title);

void Fl_File_Chooser_delete(Fl_File_Chooser *self);

void *Fl_File_Chooser_newButton(Fl_File_Chooser *self);

void *Fl_File_Chooser_previewButton(Fl_File_Chooser *self);

void *Fl_File_Chooser_showHiddenButton(Fl_File_Chooser *self);

void Fl_File_Chooser_set_callback(Fl_File_Chooser *self, void (*cb)(Fl_File_Chooser *, void *),
                              void *d);

void Fl_File_Chooser_set_color(Fl_File_Chooser *self, unsigned int c);

unsigned int Fl_File_Chooser_color(Fl_File_Chooser *self);

int Fl_File_Chooser_count(Fl_File_Chooser *self);

void Fl_File_Chooser_set_directory(Fl_File_Chooser *self, const char *d);

char *Fl_File_Chooser_directory(Fl_File_Chooser *self);

void Fl_File_Chooser_set_filter(Fl_File_Chooser *self, const char *p);

const char *Fl_File_Chooser_filter(Fl_File_Chooser *self);

int Fl_File_Chooser_filter_value(Fl_File_Chooser *self);

void Fl_File_Chooser_set_filter_value(Fl_File_Chooser *self, int f);

void Fl_File_Chooser_hide(Fl_File_Chooser *self);

void Fl_File_Chooser_set_iconsize(Fl_File_Chooser *self, unsigned char s);

unsigned char Fl_File_Chooser_iconsize(Fl_File_Chooser *self);

void Fl_File_Chooser_set_label(Fl_File_Chooser *self, const char *l);

const char *Fl_File_Chooser_label(Fl_File_Chooser *self);

void Fl_File_Chooser_set_ok_label(Fl_File_Chooser *self, const char *l);

const char *Fl_File_Chooser_ok_label(Fl_File_Chooser *self);

void Fl_File_Chooser_set_preview(Fl_File_Chooser *self, int e);

int Fl_File_Chooser_preview(const Fl_File_Chooser *self);

void Fl_File_Chooser_rescan(Fl_File_Chooser *self);

void Fl_File_Chooser_rescan_keep_filename(Fl_File_Chooser *self);

void Fl_File_Chooser_show(Fl_File_Chooser *self);

int Fl_File_Chooser_shown(Fl_File_Chooser *self);

void Fl_File_Chooser_set_textcolor(Fl_File_Chooser *self, unsigned int c);

unsigned int Fl_File_Chooser_textcolor(Fl_File_Chooser *self);

void Fl_File_Chooser_set_textfont(Fl_File_Chooser *self, int f);

int Fl_File_Chooser_textfont(Fl_File_Chooser *self);

void Fl_File_Chooser_set_textsize(Fl_File_Chooser *self, int s);

int Fl_File_Chooser_textsize(Fl_File_Chooser *self);

void Fl_File_Chooser_set_type(Fl_File_Chooser *self, int t);

int Fl_File_Chooser_type(Fl_File_Chooser *self);

void *Fl_File_Chooser_user_data(const Fl_File_Chooser *self);

void Fl_File_Chooser_set_user_data(Fl_File_Chooser *self, void *d);

const char *Fl_File_Chooser_value(Fl_File_Chooser *self, int f);

void Fl_File_Chooser_set_value(Fl_File_Chooser *self, const char *filename);

int Fl_File_Chooser_visible(Fl_File_Chooser *self);

char *Fl_dir_chooser(const char *message, const char *fname, int relative);

char *Fl_file_chooser(const char *message, const char *pat, const char *fname, int relative);

int Fl_color_chooser(const char *name, unsigned char *r, unsigned char *g, unsigned char *b,
                     int cmode);

#ifdef __cplusplus
}
#endif
