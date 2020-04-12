#include "cfl_text.h"
#include <FL/Fl_Image.H>
#include <FL/Fl_Simple_Terminal.H>
#include <FL/Fl_Text_Buffer.H>
#include <FL/Fl_Text_Display.H>
#include <FL/Fl_Text_Editor.H>
#include <FL/Fl_Widget.H>
#include <cstring>
#include <string>

Fl_Text_Buffer *Fl_Text_Buffer_new(void) {
  return new (std::nothrow) Fl_Text_Buffer;
}

void Fl_Text_Buffer_delete(Fl_Text_Buffer *self) { delete self; }

const char *Fl_Text_Buffer_text(Fl_Text_Buffer *self) { return self->text(); }

void Fl_Text_Buffer_set_text(Fl_Text_Buffer *self, const char *txt) {
  LOCK(self->text(txt);)
}

void Fl_Text_Buffer_append(Fl_Text_Buffer *self, const char *txt) {
  LOCK(self->append(txt);)
}

void Fl_Text_Buffer_remove(Fl_Text_Buffer *self, int start, int end) {
  LOCK(self->remove(start, end);)
}

int Fl_Text_Buffer_length(const Fl_Text_Buffer *self) { return self->length(); }

char *Fl_Text_Buffer_text_range(const Fl_Text_Buffer *self, int start,
                                int end) {
  return self->text_range(start, end);
}
void Fl_Text_Buffer_insert(Fl_Text_Buffer *self, int pos, const char *text) {
  LOCK(self->insert(pos, text);)
}
void Fl_Text_Buffer_replace(Fl_Text_Buffer *self, int start, int end,
                            const char *text) {
  LOCK(self->replace(start, end, text);)
}
void Fl_Text_Buffer_copy(Fl_Text_Buffer *self, Fl_Text_Buffer *fromBuf,
                         int fromStart, int fromEnd, int toPos) {
  LOCK(self->copy(fromBuf, fromStart, fromEnd, toPos);)
}
int Fl_Text_Buffer_undo(Fl_Text_Buffer *self, int *cp) {
  return self->undo(NULL);
}
void Fl_Text_Buffer_canUndo(Fl_Text_Buffer *self, char flag) {
  LOCK(self->canUndo(flag);)
}
int Fl_Text_Buffer_loadfile(Fl_Text_Buffer *self, const char *file,
                            int buflen) {
  return self->loadfile(file, 128 * 1024);
}
int Fl_Text_Buffer_tab_distance(const Fl_Text_Buffer *self) {
  return self->tab_distance();
}
void Fl_Text_Buffer_set_tab_distance(Fl_Text_Buffer *self, int tabDist) {
  LOCK(self->tab_distance(tabDist);)
}
void Fl_Text_Buffer_select(Fl_Text_Buffer *self, int start, int end) {
  LOCK(self->select(start, end);)
}
int Fl_Text_Buffer_selected(const Fl_Text_Buffer *self) {
  return self->selected();
}
void Fl_Text_Buffer_unselect(Fl_Text_Buffer *self) { self->unselect(); }
int Fl_Text_Buffer_selection_position(Fl_Text_Buffer *self, int *start,
                                      int *end) {
  return self->selection_position(start, end);
}
char *Fl_Text_Buffer_selection_text(Fl_Text_Buffer *self) {
  return self->selection_text();
}
void Fl_Text_Buffer_remove_selection(Fl_Text_Buffer *self) {
  LOCK(self->remove_selection();)
}
void Fl_Text_Buffer_replace_selection(Fl_Text_Buffer *self, const char *text) {
  LOCK(self->replace_selection(text);)
}
void Fl_Text_Buffer_highlight(Fl_Text_Buffer *self, int start, int end) {
  LOCK(self->highlight(start, end);)
}
int Fl_Text_Buffer_is_highlighted(Fl_Text_Buffer *self) {
  return self->highlight();
}
void Fl_Text_Buffer_unhighlight(Fl_Text_Buffer *self) { LOCK(self->unhighlight();) }
int Fl_Text_Buffer_highlight_position(Fl_Text_Buffer *self, int *start,
                                      int *end) {
  return self->highlight_position(start, end);
}
char *Fl_Text_Buffer_highlight_text(Fl_Text_Buffer *self) {
  return self->highlight_text();
}
char *Fl_Text_Buffer_line_text(const Fl_Text_Buffer *self, int pos) {
  return self->line_text(pos);
}
int Fl_Text_Buffer_line_start(const Fl_Text_Buffer *self, int pos) {
  return self->line_start(pos);
}
int Fl_Text_Buffer_word_start(const Fl_Text_Buffer *self, int pos) {
  return self->word_start(pos);
}
int Fl_Text_Buffer_word_end(const Fl_Text_Buffer *self, int pos) {
  return self->word_end(pos);
}
int Fl_Text_Buffer_count_lines(const Fl_Text_Buffer *self, int startPos,
                               int endPos) {
  return self->count_lines(startPos, endPos);
}
void Fl_Text_Buffer_add_modify_callback(Fl_Text_Buffer *self,
                                        Fl_Text_Modify_Cb bufModifiedCB,
                                        void *cbArg) {
  LOCK(self->add_modify_callback(bufModifiedCB, cbArg);)
}
void Fl_Text_Buffer_remove_modify_callback(Fl_Text_Buffer *self,
                                           Fl_Text_Modify_Cb bufModifiedCB,
                                           void *cbArg) {
  LOCK(self->remove_modify_callback(bufModifiedCB, cbArg);)
}
void Fl_Text_Buffer_call_modify_callbacks(Fl_Text_Buffer *self) {
  LOCK(self->call_modify_callbacks();)
}

WIDGET_DEFINE(Fl_Text_Display)

void Fl_Text_Display_init(Fl_Text_Display *self) {
  Fl_Text_Buffer *buff = new (std::nothrow) Fl_Text_Buffer();
  self->buffer(buff);
}

Fl_Text_Buffer *Fl_Text_Display_get_buffer(Fl_Text_Display *self) {
  return self->buffer();
}

void Fl_Text_Display_set_buffer(Fl_Text_Display *self, Fl_Text_Buffer *buf) {
  LOCK(self->buffer(buf);)
}

DISPLAY_DEFINE(Fl_Text_Display)

WIDGET_DEFINE(Fl_Text_Editor)

void Fl_Text_Editor_init(Fl_Text_Editor *self) {
  Fl_Text_Buffer *buff = new (std::nothrow) Fl_Text_Buffer();
  self->buffer(buff);
}

Fl_Text_Buffer *Fl_Text_Editor_get_buffer(Fl_Text_Editor *self) {
  return self->buffer();
}

void Fl_Text_Editor_set_buffer(Fl_Text_Editor *self, Fl_Text_Buffer *buf) {
  LOCK(self->buffer(buf);)
}

DISPLAY_DEFINE(Fl_Text_Editor)

int kf_copy(Fl_Text_Editor *e) {
  int ret;
  LOCK(ret = Fl_Text_Editor::kf_copy(1, e));
  return ret;
}

int kf_cut(Fl_Text_Editor *e) {
  int ret;
  LOCK(ret = Fl_Text_Editor::kf_cut(1, e));
  return ret;
}

int kf_paste(Fl_Text_Editor *e) {
  int ret;
  LOCK(ret = Fl_Text_Editor::kf_paste(1, e));
  return ret;
}

int kf_undo(Fl_Text_Editor *e) {
  int ret;
  LOCK(ret = Fl_Text_Editor::kf_undo(1, e));
  return ret;
}

WIDGET_DEFINE(Fl_Simple_Terminal)

void Fl_Simple_Terminal_init(Fl_Simple_Terminal *self) {
  Fl_Text_Buffer *buff = new (std::nothrow) Fl_Text_Buffer();
  self->buffer(buff);
}

Fl_Text_Buffer *Fl_Simple_Terminal_get_buffer(Fl_Simple_Terminal *self) {
  return self->buffer();
}

void Fl_Simple_Terminal_set_buffer(Fl_Simple_Terminal *self,
                                   Fl_Text_Buffer *buf) {
  LOCK(self->buffer(buf);)
}

DISPLAY_DEFINE(Fl_Simple_Terminal)