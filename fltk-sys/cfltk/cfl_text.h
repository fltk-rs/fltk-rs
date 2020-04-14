#pragma once

#include "global.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef void (*Fl_Text_Modify_Cb)(int pos, int nInserted, int nDeleted,
                                  int nRestyled, const char *deletedText,
                                  void *cbArg);

typedef struct Fl_Text_Buffer Fl_Text_Buffer;

Fl_Text_Buffer *Fl_Text_Buffer_new(void);

void Fl_Text_Buffer_delete(Fl_Text_Buffer *);

const char *Fl_Text_Buffer_text(Fl_Text_Buffer *self);

void Fl_Text_Buffer_set_text(Fl_Text_Buffer *self, const char *txt);

void Fl_Text_Buffer_append(Fl_Text_Buffer *self, const char *txt);

void Fl_Text_Buffer_remove(Fl_Text_Buffer *self, int start, int end);

int Fl_Text_Buffer_length(const Fl_Text_Buffer *self);

char *Fl_Text_Buffer_text_range(const Fl_Text_Buffer *self, int start, int end);
void Fl_Text_Buffer_insert(Fl_Text_Buffer *self, int pos, const char *text);
void Fl_Text_Buffer_replace(Fl_Text_Buffer *self, int start, int end, const char *text);
void Fl_Text_Buffer_copy(Fl_Text_Buffer *self, Fl_Text_Buffer *fromBuf, int fromStart, int fromEnd, int toPos);
int Fl_Text_Buffer_undo(Fl_Text_Buffer *self, int *cp);
void Fl_Text_Buffer_canUndo(Fl_Text_Buffer *self, char flag);
int Fl_Text_Buffer_loadfile(Fl_Text_Buffer *self, const char *file, int buflen);
int Fl_Text_Buffer_tab_distance(const Fl_Text_Buffer *self);
void Fl_Text_Buffer_set_tab_distance(Fl_Text_Buffer *self, int tabDist);
void Fl_Text_Buffer_select(Fl_Text_Buffer *self, int start, int end);
int Fl_Text_Buffer_selected(const Fl_Text_Buffer *self);
void Fl_Text_Buffer_unselect(Fl_Text_Buffer *self);
int Fl_Text_Buffer_selection_position(Fl_Text_Buffer *self, int *start, int *end);
char *Fl_Text_Buffer_selection_text(Fl_Text_Buffer *self);
void Fl_Text_Buffer_remove_selection(Fl_Text_Buffer *self);
void Fl_Text_Buffer_replace_selection(Fl_Text_Buffer *self, const char *text);
void Fl_Text_Buffer_highlight(Fl_Text_Buffer *self, int start, int end);
int Fl_Text_Buffer_is_highlighted(Fl_Text_Buffer *self);
void Fl_Text_Buffer_unhighlight(Fl_Text_Buffer *self);
int Fl_Text_Buffer_highlight_position(Fl_Text_Buffer *self, int *start, int *end);
char *Fl_Text_Buffer_highlight_text(Fl_Text_Buffer *self);
char *Fl_Text_Buffer_line_text(const Fl_Text_Buffer *self, int pos);
int Fl_Text_Buffer_line_start(const Fl_Text_Buffer *self, int pos);
int Fl_Text_Buffer_word_start(const Fl_Text_Buffer *self, int pos);
int Fl_Text_Buffer_word_end(const Fl_Text_Buffer *self, int pos);
int Fl_Text_Buffer_count_lines(const Fl_Text_Buffer *self, int startPos, int endPos);
void Fl_Text_Buffer_add_modify_callback(Fl_Text_Buffer *self, Fl_Text_Modify_Cb bufModifiedCB, void* cbArg);
void Fl_Text_Buffer_remove_modify_callback(Fl_Text_Buffer *self, Fl_Text_Modify_Cb bufModifiedCB, void* cbArg);
void Fl_Text_Buffer_call_modify_callbacks(Fl_Text_Buffer *self);

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