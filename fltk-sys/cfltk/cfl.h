#pragma once

#ifdef __cplusplus
extern "C" {
#endif

typedef struct Fl_Widget Fl_Widget;

typedef struct Fl_Widget_Tracker Fl_Widget_Tracker;

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

int Fl_event_x_root(void);

int Fl_event_y_root(void);

int Fl_event_dx(void);

int Fl_event_dy(void);

void Fl_get_mouse(int *, int *);

int Fl_event_is_click(void);

int Fl_event_length(void);

int Fl_event_state(void);

int Fl_screen_h(void);

int Fl_screen_w(void);

void Fl_paste(Fl_Widget *, int src);

void Fl_set_scheme(const char *scheme);

int Fl_scheme(void);

unsigned int Fl_get_rgb_color(unsigned char r, unsigned char g, unsigned char b);

const char *Fl_get_font(int idx);

unsigned char Fl_set_fonts(const char *c);

void Fl_add_handler(int (*ev_handler)(int ev));

void Fl_awake_msg(void *msg);

void *Fl_thread_msg(void);

int Fl_wait(void);

double Fl_wait_for(double);

void Fl_add_timeout(double t, void (*)(void *), void *);

void Fl_repeat_timeout(double t, void (*)(void *), void *);

void Fl_remove_timeout(void (*)(void *), void *);

int Fl_dnd(void);

void *Fl_first_window(void);

void *Fl_next_window(const void *);

int Fl_should_program_quit(void);

void Fl_program_should_quit(int flag);

unsigned int Fl_rand(void);

int Fl_event_inside(int, int, int, int);

Fl_Widget *Fl_belowmouse(void);

void Fl_delete_widget(Fl_Widget *w);

Fl_Widget_Tracker *Fl_Widget_Tracker_new(Fl_Widget *w);

int Fl_Widget_Tracker_deleted(Fl_Widget_Tracker *self);

void Fl_Widget_Tracker_delete(Fl_Widget_Tracker *self);

void Fl_init_all(void);

void Fl_redraw(void);

int Fl_event_shift(void);

int Fl_event_ctrl(void);

int Fl_event_command(void);

int Fl_event_alt(void);

void Fl_set_damage(int flag);

int Fl_damage(void);

int Fl_visual(int);

void Fl_own_colormap(void);

Fl_Widget *Fl_pushed(void);

Fl_Widget *Fl_focus(void);

void Fl_set_focus(void *);

double Fl_version(void);

int Fl_api_version(void);

int Fl_abi_version(void);

#ifdef __cplusplus
}
#endif
