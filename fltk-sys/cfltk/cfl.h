#pragma once

#ifdef __cplusplus
extern "C" {
#endif

int Fl_run(void);
int Fl_event(void);
int Fl_event_key(void);
const char *Fl_event_text(void);
int Fl_event_button(void);
int Fl_event_clicks(void);
int Fl_event_dx(void);
int Fl_event_dy(void);
int Fl_event_inside(void *);
int Fl_event_is_click(void);
int Fl_event_length(void);
int Fl_event_state(void);
int Fl_screen_h(void);
int Fl_screen_w(void);
void *Fl_belowmouse(void);

void Fl_paste(void *);

#ifdef __cplusplus
}
#endif
