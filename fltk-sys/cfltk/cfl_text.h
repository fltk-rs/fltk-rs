#pragma once

#include "global.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef struct Fl_Text_Buffer Fl_Text_Buffer;

Fl_Text_Buffer *Fl_Text_Buffer_new(void);

void Fl_Text_Buffer_delete(Fl_Text_Buffer *);

const char *Fl_Text_Buffer_text(Fl_Text_Buffer *self);

void Fl_Text_Buffer_set_text(Fl_Text_Buffer *self, const char *txt); 

void Fl_Text_Buffer_append(Fl_Text_Buffer *self, const char *txt); 

void Fl_Text_Buffer_remove(Fl_Text_Buffer *self, int start, int end); 

int Fl_Text_Buffer_length(const Fl_Text_Buffer *self);

WIDGET_DECLARE(Fl_Text_Display)

void Fl_Text_Display_init(Fl_Text_Display *);

Fl_Text_Buffer *Fl_Text_Display_get_buffer(Fl_Text_Display *);

void Fl_Text_Display_set_buffer(Fl_Text_Display *, Fl_Text_Buffer *);

DISPLAY_DECLARE(Fl_Text_Display)

WIDGET_DECLARE(Fl_Text_Editor)

void Fl_Text_Editor_init(Fl_Text_Editor *);

Fl_Text_Buffer *Fl_Text_Editor_get_buffer(Fl_Text_Editor *);

void Fl_Text_Editor_set_buffer(Fl_Text_Editor *, Fl_Text_Buffer *);

DISPLAY_DECLARE(Fl_Text_Editor)

int kf_copy(Fl_Text_Editor *e);

int kf_cut(Fl_Text_Editor *e);

int kf_paste(Fl_Text_Editor *e);

int kf_undo(Fl_Text_Editor *e);

WIDGET_DECLARE(Fl_Simple_Terminal)

void Fl_Simple_Terminal_init(Fl_Simple_Terminal *);

Fl_Text_Buffer *Fl_Simple_Terminal_get_buffer(Fl_Simple_Terminal *);

void Fl_Simple_Terminal_set_buffer(Fl_Simple_Terminal *, Fl_Text_Buffer *);

DISPLAY_DECLARE(Fl_Simple_Terminal)

#ifdef __cplusplus
}
#endif