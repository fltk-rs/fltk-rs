#include "cfl_dialog.h"
#include <Fl/Fl_Native_File_Chooser.H>
#include <Fl/Fl_Help_Dialog.H>

Fl_Native_File_Chooser* Fl_Native_File_Chooser_new(int val) {
    return new Fl_Native_File_Chooser(val);
}

const char *Fl_Native_File_Chooser_filename(Fl_Native_File_Chooser* self) {
    return self->filename();
}

void Fl_Native_File_Chooser_set_directory(Fl_Native_File_Chooser* self, const char *val) {
    self->directory(val);
}

const char *Fl_Native_File_Chooser_directory(Fl_Native_File_Chooser* self) {
    return self->directory();
}

int Fl_Native_File_Chooser_show(Fl_Native_File_Chooser* self) {
    return self->show();
}