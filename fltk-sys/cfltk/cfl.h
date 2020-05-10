#pragma once

#ifdef __cplusplus
extern "C" {
#endif

typedef void (*Fl_Awake_Handler)(void *data);

int Fl_run(void);

int Fl_lock(void);

void Fl_unlock(void);

int Fl_awake(Fl_Awake_Handler handler, void *data);

int Fl_event(void);

int Fl_event_key(void);

const char *Fl_event_text(void);

int Fl_event_button(void);

int Fl_event_clicks(void);

int Fl_event_x(void);

int Fl_event_y(void);

int Fl_event_is_click(void);

int Fl_event_length(void);

int Fl_event_state(void);

int Fl_screen_h(void);

int Fl_screen_w(void);

void Fl_paste(void *, int src);

void Fl_set_scheme(const char *scheme);

unsigned int Fl_get_color(unsigned char r, unsigned char g, unsigned char b);

const char *Fl_get_font(int idx);

unsigned char Fl_set_fonts(const char *c);

void Fl_add_handler(int (*ev_handler)(int ev));

void Fl_awake_msg(void *msg);

void *Fl_thread_msg(void);

int Fl_wait(void);

void Fl_add_timeout(double t, void (*)(void *), void *);

void Fl_repeat_timeout(double t, void (*)(void *), void *);

void Fl_remove_timeout(void (*)(void *), void *);

int Fl_dnd(void);

void *Fl_first_window(void);

#ifdef __cplusplus
}
#endif
