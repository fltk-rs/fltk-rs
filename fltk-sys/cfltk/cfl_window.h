#pragma once

#ifdef __cplusplus
extern "C" {
#endif

typedef void cfl_window;

cfl_window *cfl_window_new(int width, int height, const char *title);
void cfl_window_end(cfl_window*);
void cfl_window_show(cfl_window*);


#ifdef __cplusplus
}
#endif