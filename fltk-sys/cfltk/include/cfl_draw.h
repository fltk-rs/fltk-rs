#pragma once

#ifdef __cplusplus
extern "C" {
#endif

void Fl_set_color_int(unsigned int c);

void Fl_set_color_rgb(unsigned char r, unsigned char g, unsigned char b);

unsigned int Fl_get_color(void);

void Fl_push_clip(int x, int y, int w, int h);

void Fl_push_no_clip(void);

void Fl_pop_clip(void);

int Fl_not_clipped(int x, int y, int w, int h);

int Fl_clip_box(int x, int y, int w, int h, int *X, int *Y, int *W, int *H);

void Fl_restore_clip(void);

void Fl_set_clip_region(void *r);

void *Fl_clip_region(void);

void Fl_point(int x, int y);

void Fl_line_style(int style, int width, char *dashes);

void Fl_rect(int x, int y, int w, int h);

void Fl_focus_rect(int x, int y, int w, int h);

void Fl_rect_with_color(int x, int y, int w, int h, unsigned int c);

void Fl_rectf(int x, int y, int w, int h);

void Fl_rectf_with_color(int x, int y, int w, int h, unsigned int c);

void Fl_rectf_with_rgb(int x, int y, int w, int h, unsigned char r, unsigned char g,
                        unsigned char b);

void Fl_line(int x, int y, int x1, int y1);

void Fl_line2(int x, int y, int x1, int y1, int x2, int y2);

void Fl_loop(int x, int y, int x1, int y1, int x2, int y2);

void Fl_loop2(int x, int y, int x1, int y1, int x2, int y2, int x3, int y3);

void Fl_polygon(int x, int y, int x1, int y1, int x2, int y2);

void Fl_polygon2(int x, int y, int x1, int y1, int x2, int y2, int x3, int y3);

void Fl_xyline(int x, int y, int x1);

void Fl_xyline2(int x, int y, int x1, int y2);

void Fl_xyline3(int x, int y, int x1, int y2, int x3);

void Fl_yxline(int x, int y, int y1);

void Fl_yxline2(int x, int y, int y1, int x2);

void Fl_yxline3(int x, int y, int y1, int x2, int y3);

void Fl_arc(int x, int y, int w, int h, double a1, double a2);

void Fl_pie(int x, int y, int w, int h, double a1, double a2);

void Fl_push_matrix(void);

void Fl_pop_matrix(void);

void Fl_scale(double x, double y);

void Fl_scale2(double x);

void Fl_translate(double x, double y);

void Fl_rotate(double d);

void Fl_mult_matrix(double a, double b, double c, double d, double x, double y);

void Fl_begin_points(void);

void Fl_begin_line(void);

void Fl_begin_loop(void);

void Fl_begin_polygon(void);

void Fl_vertex(double x, double y);

void Fl_curve(double X0, double Y0, double X1, double Y1, double X2, double Y2, double X3,
               double Y3);

void Fl_arc2(double x, double y, double r, double start, double end);

void Fl_circle(double x, double y, double r);

void Fl_end_points(void);

void Fl_end_line(void);

void Fl_end_loop(void);

void Fl_end_polygon(void);

void Fl_begin_complex_polygon(void);

void Fl_gap(void);

void Fl_end_complex_polygon(void);

double Fl_transform_x(double x, double y);

double Fl_transform_y(double x, double y);

double Fl_transform_dx(double x, double y);

double Fl_transform_dy(double x, double y);

void Fl_transformed_vertex(double xf, double yf);

void Fl_end_offscreen(void);

void Fl_set_font(int face, int fsize);

int Fl_font(void);

int Fl_size(void);

int Fl_height(void);

int Fl_set_height(int font, int size);

int Fl_descent(void);

double Fl_width(const char *txt);

double Fl_width2(const char *txt, int n);

double Fl_width3(unsigned int c);

void Fl_text_extents(const char *, int *dx, int *dy, int *w, int *h);

void Fl_text_extents2(const char *t, int n, int *dx, int *dy, int *w, int *h);

const char *Fl_latin1_to_local(const char *t, int n);

const char *Fl_local_to_latin1(const char *t, int n);

const char *Fl_mac_roman_to_local(const char *t, int n);

const char *Fl_local_to_mac_roman(const char *t, int n);

void Fl_draw(const char *str, int x, int y);

void Fl_draw2(int angle, const char *str, int x, int y);

void Fl_draw3(const char *str, int n, int x, int y);

void Fl_draw4(int angle, const char *str, int n, int x, int y);

void Fl_rtl_draw(const char *str, int n, int x, int y);

void Fl_measure(const char *str, int *x, int *y, int draw_symbols);

void Fl_draw5(const char *str, int x, int y, int w, int h, int align, void **img,
               int draw_symbols);

void Fl_frame(const char *s, int x, int y, int w, int h);

void Fl_frame2(const char *s, int x, int y, int w, int h);

void Fl_draw_box(int box_type, int x, int y, int w, int h, unsigned int);

void Fl_draw_image(const unsigned char *buf, int X, int Y, int W, int H, int D, int L);

void Fl_draw_image_mono(const unsigned char *buf, int X, int Y, int W, int H, int D, int L);

char Fl_can_do_alpha_blending(void);

unsigned char *Fl_read_image(unsigned char *p, int X, int Y, int W, int H, int alpha);

unsigned char *Fl_capture_window_part(void *win, int x, int y, int w, int h);

int Fl_draw_pixmap(const char *const *data, int x, int y, int bg);

int Fl_draw_pixmap2(/*const*/ char *const *data, int x, int y, int bg);

int Fl_measure_pixmap(/*const*/ char *const *data, int *w, int *h);

int Fl_measure_pixmap2(const char *const *cdata, int *w, int *h);

const char *Fl_shortcut_label(unsigned int shortcut);

const char *Fl_shortcut_label2(unsigned int shortcut, const char **eom);

unsigned int Fl_old_shortcut(const char *s);

void Fl_overlay_rect(int x, int y, int w, int h);

void Fl_overlay_clear(void);

void Fl_set_cursor(int cursor);

void Fl_set_cursor2(int cursor, int fg, int bg);

const char *Fl_expand_text(const char *from, char *buf, int maxbuf, double maxw, int *n,
                            double *width, int wrap, int draw_symbols);

void Fl_set_status(int X, int Y, int W, int H);

void Fl_set_spot(int font, int size, int X, int Y, int W, int H, void *win);

void Fl_reset_spot(void);

int Fl_raw_image_to_png(unsigned char *data, const char *fname, int w, int h);

int Fl_raw_image_to_jpg(unsigned char *data, const char *fname, int w, int h);

int Fl_raw_image_to_bmp(unsigned char *data, const char *fname, int w, int h);

unsigned int Fl_show_colormap(unsigned int old_col);

void Fl_copy_offscreen(int x, int y, int w, int h, void *pixmap, int srcx, int srcy);

void *Fl_create_offscreen(int w, int h);

void Fl_begin_offscreen(void *b);

void Fl_end_offscreen(void);

void Fl_delete_offscreen(void *bitmap);

void Fl_rescale_offscreen(void *ctx);

void Fl_draw_text2(const char* str, int x, int y, int w, int h,
                       int align);

#ifdef __cplusplus
}
#endif