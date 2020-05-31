#include "cfl_text.h"
#include <FL/Fl.H>
#include <FL/Fl_Image.H>
#include <FL/Fl_Simple_Terminal.H>
#include <FL/Fl_Text_Buffer.H>
#include <FL/Fl_Text_Display.H>
#include <FL/Fl_Text_Editor.H>
#include <FL/Fl_Widget.H>
#include <new>

#define DISPLAY_DEFINE(widget)                                                                     \
    int widget##_text_font(const widget *self) {                                                   \
        return self->textfont();                                                                   \
    }                                                                                              \
    void widget##_set_text_font(widget *self, int s) {                                             \
        LOCK(self->textfont(s);)                                                                   \
    }                                                                                              \
    int widget##_text_size(const widget *self) {                                                   \
        return self->textsize();                                                                   \
    }                                                                                              \
    void widget##_set_text_size(widget *self, int s) {                                             \
        LOCK(self->textsize(s);)                                                                   \
    }                                                                                              \
    unsigned int widget##_text_color(const widget *self) {                                         \
        return self->textcolor();                                                                  \
    }                                                                                              \
    void widget##_set_text_color(widget *self, unsigned int n) {                                   \
        LOCK(self->textcolor(n);)                                                                  \
    }                                                                                              \
    void widget##_scroll(widget *self, int topLineNum, int horizOffset) {                          \
        LOCK(self->scroll(topLineNum, horizOffset);)                                               \
    }                                                                                              \
    void widget##_insert(widget *self, const char *text) {                                         \
        LOCK(((Fl_Text_Display *)self)->insert(text);)                                             \
    }                                                                                              \
    void widget##_set_insert_position(widget *self, int newPos) {                                  \
        LOCK(self->insert_position(newPos);)                                                       \
    }                                                                                              \
    int widget##_insert_position(const widget *self) {                                             \
        return self->insert_position();                                                            \
    }                                                                                              \
    int widget##_position_to_xy(const widget *self, int pos, int *x, int *y) {                     \
        return self->position_to_xy(pos, x, y);                                                    \
    }                                                                                              \
    int widget##_count_lines(const widget *self, int start, int end,                               \
                             int start_pos_is_line_start) {                                        \
        return self->count_lines(start, end, start_pos_is_line_start);                             \
    }                                                                                              \
    int widget##_move_right(widget *self) {                                                        \
        int ret;                                                                                   \
        LOCK(ret = self->move_right());                                                            \
        return ret;                                                                                \
    }                                                                                              \
    int widget##_move_left(widget *self) {                                                         \
        int ret;                                                                                   \
        LOCK(ret = self->move_left());                                                             \
        return ret;                                                                                \
    }                                                                                              \
    int widget##_move_up(widget *self) {                                                           \
        int ret;                                                                                   \
        LOCK(ret = self->move_up());                                                               \
        return ret;                                                                                \
    }                                                                                              \
    int widget##_move_down(widget *self) {                                                         \
        int ret;                                                                                   \
        LOCK(ret = self->move_down());                                                             \
        return ret;                                                                                \
    }                                                                                              \
    void widget##_show_cursor(widget *self, int boolean) {                                         \
        LOCK(if (boolean) self->show_cursor(); else self->hide_cursor();)                          \
    }                                                                                              \
    void widget##_set_style_table_entry(widget *self, void *sbuff, unsigned int *color, int *font, \
                                        int *fontsz, int sz) {                                     \
        Fl_Text_Display::Style_Table_Entry *stable =                                               \
            new (std::nothrow) Fl_Text_Display::Style_Table_Entry[sz];                             \
        if (!stable)                                                                               \
            return;                                                                                \
        for (int i = 0; i < sz; ++i) {                                                             \
            stable[i] = {color[i], font[i], fontsz[i]};                                            \
        }                                                                                          \
        LOCK(self->highlight_data((Fl_Text_Buffer *)sbuff, stable, sz, 'A', 0, 0);)                \
        delete[] stable;                                                                           \
    }                                                                                              \
    void widget##_set_cursor_style(widget *self, int style) {                                      \
        LOCK(self->cursor_style(style);)                                                           \
    }                                                                                              \
    void widget##_set_cursor_color(widget *self, unsigned int color) {                             \
        LOCK(self->cursor_color(color);)                                                           \
    }                                                                                              \
    void widget##_set_scrollbar_width(widget *self, int width) {                                   \
        LOCK(self->scrollbar_width(width);)                                                        \
    }                                                                                              \
    void widget##_set_scrollbar_size(widget *self, int newSize) {                                  \
        LOCK(self->scrollbar_size(newSize);)                                                       \
    }                                                                                              \
    void widget##_set_scrollbar_align(widget *self, int align) {                                   \
        LOCK(self->scrollbar_align(align);)                                                        \
    }                                                                                              \
    int widget##_cursor_style(widget *self) {                                                      \
        return self->cursor_style();                                                               \
    }                                                                                              \
    unsigned int widget##_cursor_color(widget *self) {                                             \
        return self->cursor_color();                                                               \
    }                                                                                              \
    int widget##_scrollbar_width(widget *self) {                                                   \
        return self->scrollbar_width();                                                            \
    }                                                                                              \
    int widget##_scrollbar_size(widget *self) {                                                    \
        return self->scrollbar_size();                                                             \
    }                                                                                              \
    int widget##_scrollbar_align(widget *self) {                                                   \
        return self->scrollbar_align();                                                            \
    }                                                                                              \
    int widget##_line_start(const widget *self, int pos) {                                         \
        return self->line_start(pos);                                                              \
    }                                                                                              \
    int widget##_line_end(const widget *self, int startPos, int startPosIsLineStart) {             \
        return self->line_end(startPos, startPosIsLineStart);                                      \
    }                                                                                              \
    int widget##_skip_lines(widget *self, int startPos, int nLines, int startPosIsLineStart) {     \
        int ret;                                                                                   \
        LOCK(ret = self->skip_lines(startPos, nLines, startPosIsLineStart);)                       \
        return ret;                                                                                \
    }                                                                                              \
    int widget##_rewind_lines(widget *self, int startPos, int nLines) {                            \
        int ret;                                                                                   \
        LOCK(ret = self->rewind_lines(startPos, nLines);)                                          \
        return ret;                                                                                \
    }                                                                                              \
    void widget##_next_word(widget *self) {                                                        \
        LOCK(self->next_word();)                                                                   \
    }                                                                                              \
    void widget##_previous_word(widget *self) {                                                    \
        LOCK(self->previous_word();)                                                               \
    }                                                                                              \
    int widget##_word_start(const widget *self, int pos) {                                         \
        return self->word_start(pos);                                                              \
    }                                                                                              \
    int widget##_word_end(const widget *self, int pos) {                                           \
        return self->word_end(pos);                                                                \
    }                                                                                              \
    double widget##_x_to_col(const widget *self, double x) {                                       \
        return self->x_to_col(x);                                                                  \
    }                                                                                              \
    double widget##_col_to_x(const widget *self, double col) {                                     \
        return self->col_to_x(col);                                                                \
    }                                                                                              \
    void widget##_set_linenumber_width(widget *self, int width) {                                  \
        LOCK(self->linenumber_width(width);)                                                       \
    }                                                                                              \
    int widget##_linenumber_width(const widget *self) {                                            \
        return self->linenumber_width();                                                           \
    }                                                                                              \
    void widget##_set_linenumber_font(widget *self, int val) {                                     \
        LOCK(self->linenumber_font(val);)                                                          \
    }                                                                                              \
    int widget##_linenumber_font(const widget *self) {                                             \
        return self->linenumber_font();                                                            \
    }                                                                                              \
    void widget##_set_linenumber_size(widget *self, int val) {                                     \
        LOCK(self->linenumber_size(val);)                                                          \
    }                                                                                              \
    int widget##_linenumber_size(const widget *self) {                                             \
        return self->linenumber_size();                                                            \
    }                                                                                              \
    void widget##_set_linenumber_fgcolor(widget *self, unsigned int val) {                         \
        LOCK(self->linenumber_fgcolor(val);)                                                       \
    }                                                                                              \
    unsigned int widget##_linenumber_fgcolor(const widget *self) {                                 \
        return self->linenumber_fgcolor();                                                         \
    }                                                                                              \
    void widget##_set_linenumber_bgcolor(widget *self, unsigned int val) {                         \
        LOCK(self->linenumber_bgcolor(val);)                                                       \
    }                                                                                              \
    unsigned int widget##_linenumber_bgcolor(const widget *self) {                                 \
        return self->linenumber_bgcolor();                                                         \
    }                                                                                              \
    void widget##_set_linenumber_align(widget *self, int val) {                                    \
        LOCK(self->linenumber_align(val);)                                                         \
    }                                                                                              \
    int widget##_linenumber_align(const widget *self) {                                            \
        return self->linenumber_align();                                                           \
    }                                                                                              \
    int widget##_in_selection(const widget *self, int x, int y) {                                  \
        return self->in_selection(x, y);                                                           \
    }

Fl_Text_Buffer *Fl_Text_Buffer_new(void) {
    return new (std::nothrow) Fl_Text_Buffer;
}

void Fl_Text_Buffer_delete(Fl_Text_Buffer *self) {
    delete self;
}

const char *Fl_Text_Buffer_text(Fl_Text_Buffer *self) {
    return self->text();
}

void Fl_Text_Buffer_set_text(Fl_Text_Buffer *self, const char *txt) {
    LOCK(self->text(txt);)
}

void Fl_Text_Buffer_append(Fl_Text_Buffer *self, const char *txt) {
    LOCK(self->append(txt);)
}

void Fl_Text_Buffer_remove(Fl_Text_Buffer *self, int start, int end) {
    LOCK(self->remove(start, end);)
}

int Fl_Text_Buffer_length(const Fl_Text_Buffer *self) {
    return self->length();
}

char *Fl_Text_Buffer_text_range(const Fl_Text_Buffer *self, int start, int end) {
    return self->text_range(start, end);
}

void Fl_Text_Buffer_insert(Fl_Text_Buffer *self, int pos, const char *text) {
    LOCK(self->insert(pos, text);)
}

void Fl_Text_Buffer_replace(Fl_Text_Buffer *self, int start, int end, const char *text) {
    LOCK(self->replace(start, end, text);)
}

void Fl_Text_Buffer_copy(Fl_Text_Buffer *self, Fl_Text_Buffer *fromBuf, int fromStart, int fromEnd,
                         int toPos) {
    LOCK(self->copy(fromBuf, fromStart, fromEnd, toPos);)
}

int Fl_Text_Buffer_undo(Fl_Text_Buffer *self, int *cp) {
    return self->undo(NULL);
}

void Fl_Text_Buffer_canUndo(Fl_Text_Buffer *self, char flag) {
    LOCK(self->canUndo(flag);)
}

int Fl_Text_Buffer_loadfile(Fl_Text_Buffer *self, const char *file, int buflen) {
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

void Fl_Text_Buffer_unselect(Fl_Text_Buffer *self) {
    self->unselect();
}

int Fl_Text_Buffer_selection_position(Fl_Text_Buffer *self, int *start, int *end) {
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

void Fl_Text_Buffer_unhighlight(Fl_Text_Buffer *self) {
    LOCK(self->unhighlight();)
}

int Fl_Text_Buffer_highlight_position(Fl_Text_Buffer *self, int *start, int *end) {
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

int Fl_Text_Buffer_count_lines(const Fl_Text_Buffer *self, int startPos, int endPos) {
    return self->count_lines(startPos, endPos);
}

void Fl_Text_Buffer_add_modify_callback(Fl_Text_Buffer *self, Fl_Text_Modify_Cb bufModifiedCB,
                                        void *cbArg) {
    LOCK(self->add_modify_callback(bufModifiedCB, cbArg);)
}

void Fl_Text_Buffer_remove_modify_callback(Fl_Text_Buffer *self, Fl_Text_Modify_Cb bufModifiedCB,
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

void Fl_Simple_Terminal_set_buffer(Fl_Simple_Terminal *self, Fl_Text_Buffer *buf) {
    LOCK(self->buffer(buf);)
}

void Fl_Simple_Terminal_set_stay_at_bottom(Fl_Simple_Terminal *self, int flag) {
    LOCK(self->stay_at_bottom(flag);)
}

int Fl_Simple_Terminal_stay_at_bottom(const Fl_Simple_Terminal *self) {
    return self->stay_at_bottom();
}

void Fl_Simple_Terminal_set_history_lines(Fl_Simple_Terminal *self, int cnt) {
    LOCK(self->history_lines(cnt);)
}

int Fl_Simple_Terminal_history_lines(const Fl_Simple_Terminal *self) {
    return self->history_lines();
}

void Fl_Simple_Terminal_set_ansi(Fl_Simple_Terminal *self, int val) {
    LOCK(self->ansi(val);)
}

int Fl_Simple_Terminal_ansi(const Fl_Simple_Terminal *self) {
    return self->ansi();
}

void Fl_Simple_Terminal_append(Fl_Simple_Terminal *self, const char *s) {
    LOCK(self->append(s);)
}

void Fl_Simple_Terminal_set_text(Fl_Simple_Terminal *self, const char *s) {
    LOCK(self->text(s);)
}

const char *Fl_Simple_Terminal_text(const Fl_Simple_Terminal *self) {
    return self->text();
}

void Fl_Simple_Terminal_clear(Fl_Simple_Terminal *self) {
    LOCK(self->clear();)
}

void Fl_Simple_Terminal_remove_lines(Fl_Simple_Terminal *self, int start, int count) {
    LOCK(self->remove_lines(start, count);)
}

DISPLAY_DEFINE(Fl_Simple_Terminal)