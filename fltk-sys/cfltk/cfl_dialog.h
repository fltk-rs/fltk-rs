#pragma once

#ifdef __cplusplus
extern "C" {
#endif

typedef struct Fl_Native_File_Chooser Fl_Native_File_Chooser;

Fl_Native_File_Chooser *Fl_Native_File_Chooser_new(int);

const char *Fl_Native_File_Chooser_filename(Fl_Native_File_Chooser *);

const char *Fl_Native_File_Chooser_filenames(Fl_Native_File_Chooser *, int);

int Fl_Native_File_Chooser_count(Fl_Native_File_Chooser *);

void Fl_Native_File_Chooser_set_directory(Fl_Native_File_Chooser *,
                                          const char *val);

const char *Fl_Native_File_Chooser_directory(Fl_Native_File_Chooser *);

int Fl_Native_File_Chooser_show(Fl_Native_File_Chooser *);

void Fl_Native_File_Chooser_set_option(Fl_Native_File_Chooser *, int opt);

void Fl_Native_File_Chooser_set_type(Fl_Native_File_Chooser *, int typ);

void Fl_Native_File_Chooser_set_title(Fl_Native_File_Chooser *,
                                      const char *title);

void Fl_Native_File_Chooser_set_filter(Fl_Native_File_Chooser *, const char *f);

void Fl_Native_File_Chooser_set_preset_file(Fl_Native_File_Chooser *,
                                            const char *f);

const char *Fl_Native_File_Chooser_errmsg(Fl_Native_File_Chooser *);

void cfl_message(const char *txt);

void cfl_alert(const char *txt);

int cfl_choice(const char *txt, const char *b0, const char *b1, const char *b2);

const char *cfl_input(const char *txt, const char *deflt);

const char *cfl_password(const char *txt, const char *deflt);

typedef struct Fl_Help_Dialog Fl_Help_Dialog;

Fl_Help_Dialog* Fl_Help_Dialog_new(void);

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

#ifdef __cplusplus
}
#endif