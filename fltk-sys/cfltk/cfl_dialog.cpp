#include "cfl_dialog.h"
#include <Fl/Fl_Help_Dialog.H>
#include <Fl/Fl_Native_File_Chooser.H>
#include <FL/Fl_Image.H>
#include <Fl/fl_ask.H>

Fl_Native_File_Chooser *Fl_Native_File_Chooser_new(int val) {
  return new Fl_Native_File_Chooser(val);
}

const char *Fl_Native_File_Chooser_filename(Fl_Native_File_Chooser *self) {
  return self->filename();
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

void cfl_message(const char *txt) { fl_message(txt); }

void cfl_alert(const char *txt) { fl_alert(txt); }

int cfl_choice(const char *txt, const char *b0, const char *b1,
               const char *b2) {
  if (strlen(b2) == 0)
    b2 = NULL;
  return fl_choice(txt, b0, b1, b2);
}

const char *cfl_input(const char *txt, const char *deflt) {
  return fl_input(txt, deflt);
}

const char *cfl_password(const char *txt, const char *deflt) {
  return fl_password(txt, deflt);
}