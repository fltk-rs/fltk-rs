#pragma once

#ifdef __cplusplus
extern "C" {
#endif

typedef struct Fl_Native_File_Chooser Fl_Native_File_Chooser;

Fl_Native_File_Chooser* Fl_Native_File_Chooser_new(int);

const char *Fl_Native_File_Chooser_filename(Fl_Native_File_Chooser*);
void Fl_Native_File_Chooser_set_directory(Fl_Native_File_Chooser*, const char *val) ;
const char *Fl_Native_File_Chooser_directory(Fl_Native_File_Chooser*);
int Fl_Native_File_Chooser_show(Fl_Native_File_Chooser*);



#ifdef __cplusplus
}
#endif