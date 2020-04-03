#include "cfl_dialog.h"
#include <FL/Fl_Help_Dialog.H>
#include <FL/Fl_Image.H>
#include <FL/Fl_Native_File_Chooser.H>
#include <FL/fl_ask.H>
#include <new>

Fl_Native_File_Chooser *Fl_Native_File_Chooser_new(int val) {
  return new (std::nothrow) Fl_Native_File_Chooser(val);
}

const char *Fl_Native_File_Chooser_filename(Fl_Native_File_Chooser *self) {
  auto x = self->filename();
  if (!strcmp(x, ""))
    return NULL;
  else
    return x;
}

const char *Fl_Native_File_Chooser_filenames(Fl_Native_File_Chooser *self,
                                             int cnt) {
  auto x = self->filename(cnt);
  if (!strcmp(x, ""))
    return NULL;
  else
    return x;
}

int Fl_Native_File_Chooser_count(Fl_Native_File_Chooser *self) {
  return self->count();
}

void Fl_Native_File_Chooser_set_directory(Fl_Native_File_Chooser *self,
                                          const char *val) {
  self->directory(val);
}

const char *Fl_Native_File_Chooser_directory(Fl_Native_File_Chooser *self) {
  return self->directory();
}

int Fl_Native_File_Chooser_show(Fl_Native_File_Chooser *self) {
  return self->show();
}

void Fl_Native_File_Chooser_set_option(Fl_Native_File_Chooser *self, int opt) {
  self->options(opt);
}

void Fl_Native_File_Chooser_set_type(Fl_Native_File_Chooser *self, int typ) {
  self->type(typ);
}

void Fl_Native_File_Chooser_set_title(Fl_Native_File_Chooser *self,
                                      const char *title) {
  self->title(title);
}

void Fl_Native_File_Chooser_set_filter(Fl_Native_File_Chooser *self,
                                       const char *f) {
  self->filter(f);
}

void Fl_Native_File_Chooser_set_preset_file(Fl_Native_File_Chooser *self,
                                            const char *f) {
  self->preset_file(f);
}

const char *Fl_Native_File_Chooser_errmsg(Fl_Native_File_Chooser *self) {
  return self->errmsg();
}

void cfl_message(const char *txt) { fl_message("%s", txt); }

void cfl_alert(const char *txt) { fl_alert("%s", txt); }

int cfl_choice(const char *txt, const char *b0, const char *b1,
               const char *b2) {
  if (strlen(b2) == 0)
    b2 = NULL;
  return fl_choice("%s", b0, b1, b2, txt);
}

const char *cfl_input(const char *txt, const char *deflt) {
  return fl_input("%s", deflt, txt);
}

const char *cfl_password(const char *txt, const char *deflt) {
  return fl_password("%s", deflt, txt);
}

Fl_Help_Dialog *Fl_Help_Dialog_new(void) {
  return new (std::nothrow) Fl_Help_Dialog();
}

int Fl_Help_Dialog_h(Fl_Help_Dialog *self) { return self->h(); }
void Fl_Help_Dialog_hide(Fl_Help_Dialog *self) { return self->hide(); }
int Fl_Help_Dialog_load(Fl_Help_Dialog *self, const char *f) {
  return self->load(f);
}
void Fl_Help_Dialog_position(Fl_Help_Dialog *self, int xx, int yy) {
  self->position(xx, yy);
}
void Fl_Help_Dialog_resize(Fl_Help_Dialog *self, int xx, int yy, int ww,
                           int hh) {
  self->resize(xx, yy, ww, hh);
}
void Fl_Help_Dialog_show(Fl_Help_Dialog *self) { self->show(); }
void Fl_Help_Dialog_set_text_size(Fl_Help_Dialog *self, int s) {
  self->textsize(s);
}
int Fl_Help_Dialog_text_size(Fl_Help_Dialog *self) { return self->textsize(); }
void Fl_Help_Dialog_set_value(Fl_Help_Dialog *self, const char *f) {
  self->value(f);
}
const char *Fl_Help_Dialog_value(const Fl_Help_Dialog *self) {
  return self->value();
}
int Fl_Help_Dialog_visible(Fl_Help_Dialog *self) { return self->visible(); }
int Fl_Help_Dialog_w(Fl_Help_Dialog *self) { return self->w(); }
int Fl_Help_Dialog_x(Fl_Help_Dialog *self) { return self->x(); }
int Fl_Help_Dialog_y(Fl_Help_Dialog *self) { return self->y(); }