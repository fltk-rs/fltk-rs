#pragma once

#ifdef __cplusplus
extern "C" {
#endif

void cfl_set_color_int(unsigned int c);
void cfl_set_color_rgb(unsigned char r, unsigned char g, unsigned char b);
unsigned int cfl_get_color(void);
void cfl_push_clip(int x, int y, int w, int h);
void cfl_push_no_clip(void);
void cfl_pop_clip(void);
int cfl_not_clipped(int x, int y, int w, int h);
int cfl_clip_box(int x, int y, int w, int h, int *X, int *Y, int *W, int *H);
void cfl_restore_clip(void);
void cfl_set_clip_region(void *r);
void *cfl_clip_region(void);
void cfl_point(int x, int y);
void cfl_line_style(int style, int width, char *dashes);
void cfl_rect(int x, int y, int w, int h);
void cfl_focus_rect(int x, int y, int w, int h);
void cfl_rect_with_color(int x, int y, int w, int h, unsigned int c);
void cfl_rectf(int x, int y, int w, int h);
void cfl_rectf_with_color(int x, int y, int w, int h, unsigned int c);
void cfl_rectf_with_rgb(int x, int y, int w, int h, unsigned char r, unsigned char g,
              unsigned char b);
void cfl_line(int x, int y, int x1, int y1);
void cfl_line2(int x, int y, int x1, int y1, int x2, int y2);
void cfl_loop(int x, int y, int x1, int y1, int x2, int y2);
void cfl_loop2(int x, int y, int x1, int y1, int x2, int y2, int x3, int y3);
void cfl_polygon(int x, int y, int x1, int y1, int x2, int y2);
void cfl_polygon2(int x, int y, int x1, int y1, int x2, int y2, int x3, int y3);
void cfl_xyline(int x, int y, int x1);
void cfl_xyline2(int x, int y, int x1, int y2);
void cfl_xyline3(int x, int y, int x1, int y2, int x3);
void cfl_yxline(int x, int y, int y1);
void cfl_yxline2(int x, int y, int y1, int x2);
void cfl_yxline3(int x, int y, int y1, int x2, int y3);
void cfl_arc(int x, int y, int w, int h, double a1, double a2);
void cfl_pie(int x, int y, int w, int h, double a1, double a2);
void cfl_push_matrix(void);
void cfl_pop_matrix(void);
void cfl_scale(double x, double y);
void cfl_scale2(double x);
void cfl_translate(double x, double y);
void cfl_rotate(double d);
void cfl_mult_matrix(double a, double b, double c, double d, double x, double y);
void cfl_begin_points(void);
void cfl_begin_line(void);
void cfl_begin_loop(void);
void cfl_begin_polygon(void);
void cfl_vertex(double x, double y);
void cfl_curve(double X0, double Y0, double X1, double Y1, double X2, double Y2,
              double X3, double Y3);
void cfl_arc2(double x, double y, double r, double start, double end);
void cfl_circle(double x, double y, double r);
void cfl_end_points(void);
void cfl_end_line(void);
void cfl_end_loop(void);
void cfl_end_polygon(void);
void cfl_begin_complex_polygon(void);
void cfl_gap(void);
void cfl_end_complex_polygon(void);
double cfl_transform_x(double x, double y);
double cfl_transform_y(double x, double y);
double cfl_transform_dx(double x, double y);
double cfl_transform_dy(double x, double y);
void cfl_transformed_vertex(double xf, double yf);
void cfl_end_offscreen(void);
void cfl_set_font(int face, int fsize);
int cfl_font(void);
int cfl_size(void);
int cfl_height(void);
int cfl_set_height(int font, int size);
int cfl_descent(void);
double cfl_width(const char *txt);
double cfl_width2(const char *txt, int n);
double cfl_width3(unsigned int c);
void cfl_text_extents(const char *, int *dx, int *dy, int *w, int *h);
void cfl_text_extents2(const char *t, int n, int *dx, int *dy, int *w, int *h);
const char *cfl_latin1_to_local(const char *t, int n);
const char *cfl_local_to_latin1(const char *t, int n);
const char *cfl_mac_roman_to_local(const char *t, int n);
const char *cfl_local_to_mac_roman(const char *t, int n);
void cfl_draw(const char *str, int x, int y);
void cfl_draw2(int angle, const char *str, int x, int y);
void cfl_draw3(const char *str, int n, int x, int y);
void cfl_draw4(int angle, const char *str, int n, int x, int y);
void cfl_rtl_draw(const char *str, int n, int x, int y);
void cfl_measure(const char *str, int *x, int *y, int draw_symbols);
void cfl_draw5(const char *str, int x, int y, int w, int h, int align, void **img,
             int draw_symbols);
void cfl_frame7(const char *s, int x, int y, int w, int h);
void cfl_frame2(const char *s, int x, int y, int w, int h);
void cfl_draw_box(int box_type, int x, int y, int w, int h, int);
void cfl_draw_image(const unsigned char *buf, int X, int Y, int W, int H, int D,
                   int L);
void cfl_draw_image_mono(const unsigned char *buf, int X, int Y, int W, int H,
                        int D, int L);
char cfl_can_do_alpha_blending(void);
unsigned char *cfl_read_image(unsigned char *p, int X, int Y, int W, int H,
                             int alpha);
unsigned char *cfl_capture_window_part(void *win, int x, int y, int w, int h);
int cfl_draw_pixmap(const char *const *data, int x, int y, int bg);
int cfl_draw_pixmap2(/*const*/ char *const *data, int x, int y, int bg);
int cfl_measure_pixmap(/*const*/ char *const *data, int *w, int *h);
int cfl_measure_pixmap2(const char *const *cdata, int *w, int *h);
const char *cfl_shortcut_label(unsigned int shortcut);
const char *cfl_shortcut_label2(unsigned int shortcut, const char **eom);
unsigned int cfl_old_shortcut(const char *s);
void cfl_overlay_rect(int x, int y, int w, int h);
void cfl_overlay_clear(void);
void cfl_set_cursor(int cursor);
void cfl_set_cursor2(int cursor, int fg, int bg);
const char *cfl_expand_text(const char *from, char *buf, int maxbuf, double maxw,
                           int *n, double *width, int wrap, int draw_symbols);
void cfl_set_status(int X, int Y, int W, int H);
void cfl_set_spot(int font, int size, int X, int Y, int W, int H,
                 void *win);
void cfl_reset_spot(void);
int cfl_draw_symbol(const char *label, int x, int y, int w, int h, int);

int cfl_raw_image_to_png(unsigned char *data, const char* fname, int w, int h);

int cfl_raw_image_to_jpg(unsigned char *data, const char* fname, int w, int h);

int cfl_raw_image_to_bmp(unsigned char *data, const char* fname, int w, int h);

#ifdef __cplusplus
}
#endif