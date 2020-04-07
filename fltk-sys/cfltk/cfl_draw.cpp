#include "cfl_draw.h"
#include <FL/Enumerations.H>
#include <FL/Fl_Image.H>
#include <FL/fl_draw.H>
#include <FL/Fl_Window.H>
#include <string>

void cfl_set_color_int(unsigned int c) { fl_color(c); }
void cfl_set_color_rgb(unsigned char r, unsigned char g, unsigned char b) {
  fl_color(r, g, b);
}
unsigned int cfl_get_color(void) { return fl_color(); }
void cfl_push_clip(int x, int y, int w, int h) { fl_push_clip(x, y, w, h); }
void cfl_push_no_clip(void) { fl_push_no_clip(); }
void cfl_pop_clip(void) { fl_pop_clip(); }
int cfl_not_clipped(int x, int y, int w, int h) { return fl_not_clipped(x, y, w, h); }
int cfl_clip_box(int x, int y, int w, int h, int *X, int *Y, int *W, int *H) {
  return fl_clip_box(x, y, w, h, *X, *Y, *W, *H);
}
void cfl_restore_clip(void) { fl_restore_clip(); }
void cfl_set_clip_region(void *r) { fl_clip_region(*(Fl_Region *)r); }
void *cfl_clip_region(void) { return (void *)cfl_clip_region(); }
void cfl_point(int x, int y) { fl_point(x, y); }
void cfl_line_style(int style, int width, char *dashes) {
  fl_line_style(style, width, dashes);
}
void cfl_rect(int x, int y, int w, int h) { fl_rect(x, y, w, h); }
void cfl_focus_rect(int x, int y, int w, int h) { fl_focus_rect(x, y, w, h); }
void cfl_rect_with_color(int x, int y, int w, int h, unsigned int c) {
  fl_rect(x, y, w, h, c);
}
void cfl_rectf(int x, int y, int w, int h) { fl_rectf(x, y, w, h); }
void cfl_rectf_with_color(int x, int y, int w, int h, unsigned int c) {
  fl_rectf(x, y, w, h, c);
}
void cfl_rectf_with_rgb(int x, int y, int w, int h, unsigned char r,
                        unsigned char g, unsigned char b) {
  fl_rectf(x, y, w, h, r, g, b);
}
void cfl_line(int x, int y, int x1, int y1) { fl_line(x, y, x1, y1); }
void cfl_line2(int x, int y, int x1, int y1, int x2, int y2) {
  fl_line(x, y, x1, y1, x2, y2);
}
void cfl_loop(int x, int y, int x1, int y1, int x2, int y2) {
  fl_loop(x, y, x1, y1, x2, y2);
}
void cfl_loop2(int x, int y, int x1, int y1, int x2, int y2, int x3, int y3) {
  fl_loop(x, y, x1, y1, x2, y2, x3, y3);
}
void cfl_polygon(int x, int y, int x1, int y1, int x2, int y2) {
  fl_polygon(x, y, x1, y1, x2, y2);
}
void cfl_polygon2(int x, int y, int x1, int y1, int x2, int y2, int x3,
                  int y3) {
  fl_polygon(x, y, x1, y1, x2, y2, x3, y3);
}
void cfl_xyline(int x, int y, int x1) { fl_xyline(x, y, x1); }
void cfl_xyline2(int x, int y, int x1, int y2) { fl_xyline(x, y, x1, y2); }
void cfl_xyline3(int x, int y, int x1, int y2, int x3) {
  fl_xyline(x, y, x1, y2, x3);
}
void cfl_yxline(int x, int y, int y1) { fl_yxline(x, y, y1); }
void cfl_yxline2(int x, int y, int y1, int x2) { fl_yxline(x, y, y1, x2); }
void cfl_yxline3(int x, int y, int y1, int x2, int y3) {
  fl_yxline(x, y, y1, x2, y3);
}
void cfl_arc(int x, int y, int w, int h, double a1, double a2) {
  fl_arc(x, y, w, h, a1, a2);
}
void cfl_pie(int x, int y, int w, int h, double a1, double a2) {
  fl_pie(x, y, w, h, a1, a2);
}
// void cfl_chord(int x, int y, int w, int h, double a1, double a2) {
//   fl_chord(x, y, w, h, a1, a2);
// }
void cfl_push_matrix(void) { fl_push_matrix(); }
void cfl_pop_matrix(void) { fl_pop_matrix(); }
void cfl_scale(double x, double y) { fl_scale(x, y); }
void cfl_scale2(double x) { fl_scale(x); }
void cfl_translate(double x, double y) { fl_translate(x, y); }
void cfl_rotate(double d) { fl_rotate(d); }
void cfl_mult_matrix(double a, double b, double c, double d, double x,
                     double y) {
  fl_mult_matrix(a, b, c, d, x, y);
}
void cfl_begin_points(void) { fl_begin_points(); }
void cfl_begin_line(void) { fl_begin_line(); }
void cfl_begin_loop(void) { fl_begin_loop(); }
void cfl_begin_polygon(void) { fl_begin_polygon(); }
void cfl_vertex(double x, double y) { fl_vertex(x, y); }
void cfl_curve(double X0, double Y0, double X1, double Y1, double X2, double Y2,
               double X3, double Y3) {
  fl_curve(X0, Y0, X1, Y1, X2, Y2, X3, Y3);
}
void cfl_arc2(double x, double y, double r, double start, double end) {
  fl_arc(x, y, r, start, end);
}
void cfl_circle(double x, double y, double r) { fl_circle(x, y, r); }
void cfl_end_points(void) { fl_end_points(); }
void cfl_end_line(void) { fl_end_line(); }
void cfl_end_loop(void) { fl_end_loop(); }
void cfl_end_polygon(void) { fl_end_polygon(); }
void cfl_begin_complex_polygon(void) { fl_begin_complex_polygon(); }
void cfl_gap(void) { fl_gap(); }
void cfl_end_complex_polygon(void) { fl_end_complex_polygon(); }
double cfl_transform_x(double x, double y) { return fl_transform_x(x, y); }
double cfl_transform_y(double x, double y) { return fl_transform_y(x, y); }
double cfl_transform_dx(double x, double y) { return fl_transform_dx(x, y); }
double cfl_transform_dy(double x, double y) { return fl_transform_dy(x, y); }
void cfl_transformed_vertex(double xf, double yf) {
  fl_transformed_vertex(xf, yf);
}
void cfl_end_offscreen(void) { fl_end_offscreen(); }
void cfl_set_font(int face, int fsize) { fl_font(face, fsize); }
int cfl_font(void) { return fl_font(); }
int cfl_size(void) { return fl_size(); }
int cfl_height(void) { return fl_height(); }
int cfl_set_height(int font, int size) { return fl_height(font, size); }
int cfl_descent(void) { return fl_descent(); }
double cfl_width(const char *txt) { return fl_width(txt); }
double cfl_width2(const char *txt, int n) { return fl_width(txt, n); }
double cfl_width3(unsigned int c) { return fl_width(c); }
void cfl_text_extents(const char *txt, int *dx, int *dy, int *w, int *h) {
  return fl_text_extents(txt, *dx, *dy, *w, *h);
}
void cfl_text_extents2(const char *t, int n, int *dx, int *dy, int *w, int *h) {
  return fl_text_extents(t, n, *dx, *dy, *w, *h);
}
const char *cfl_latin1_to_local(const char *t, int n) {
  return fl_latin1_to_local(t, n);
}
const char *cfl_local_to_latin1(const char *t, int n) {
  return fl_local_to_latin1(t, n);
}
const char *cfl_mac_roman_to_local(const char *t, int n) {
  return fl_mac_roman_to_local(t, n);
}
const char *cfl_local_to_mac_roman(const char *t, int n) {
  return fl_local_to_mac_roman(t, n);
}
void cfl_draw(const char *str, int x, int y) { fl_draw(str, x, y); }
void cfl_draw2(int angle, const char *str, int x, int y) {
  fl_draw(angle, str, x, y);
}
void cfl_draw3(const char *str, int n, int x, int y) { fl_draw(str, n, x, y); }
void cfl_draw4(int angle, const char *str, int n, int x, int y) {
  fl_draw(angle, str, n, x, y);
}
void cfl_rtl_draw(const char *str, int n, int x, int y) {
  fl_rtl_draw(str, n, x, y);
}
void cfl_measure(const char *str, int *x, int *y, int draw_symbols) {
  fl_measure(str, *x, *y, draw_symbols);
}
void cfl_draw5(const char *str, int x, int y, int w, int h, int align,
               void **img, int draw_symbols) {
  fl_draw(str, x, y, w, h, align, (Fl_Image *)*img, draw_symbols);
}
void cfl_frame7(const char *s, int x, int y, int w, int h) {
  fl_frame(s, x, y, w, h);
}
void cfl_frame2(const char *s, int x, int y, int w, int h) {
  fl_frame(s, x, y, w, h);
}
void cfl_draw_box(int box_type, int x, int y, int w, int h, int c) {
  fl_draw_box((Fl_Boxtype)box_type, x, y, w, h, c);
}
void cfl_draw_image(const unsigned char *buf, int X, int Y, int W, int H, int D,
                    int L) {
  fl_draw_image(buf, X, Y, W, H, D, L);
}
void cfl_draw_image_mono(const unsigned char *buf, int X, int Y, int W, int H,
                         int D, int L) {
  fl_draw_image_mono(buf, X, Y, W, H, D, L);
}
char cfl_can_do_alpha_blending(void) { return fl_can_do_alpha_blending(); }
unsigned char *cfl_read_image(unsigned char *p, int X, int Y, int W, int H,
                              int alpha) {
  return fl_read_image(p, X, Y, W, H, alpha);
}
void *cfl_capture_window_part(void *win, int x, int y, int w, int h) {
  return (void *)fl_capture_window_part((Fl_Window *)win, x, y, w, h);
}
int cfl_draw_pixmap(const char *const *data, int x, int y, int bg) {
  return fl_draw_pixmap(data, x, y, bg);
}
int cfl_draw_pixmap2(/*const*/ char *const *data, int x, int y, int bg) {
  return fl_draw_pixmap(data, x, y, bg);
}
int cfl_measure_pixmap(/*const*/ char *const *data, int *w, int *h) {
  return fl_measure_pixmap(data, *w, *h);
}
int cfl_measure_pixmap2(const char *const *cdata, int *w, int *h) {
  return fl_measure_pixmap(cdata, *w, *h);
}
const char *cfl_shortcut_label(unsigned int shortcut) {
  return fl_shortcut_label(shortcut);
}
const char *cfl_shortcut_label2(unsigned int shortcut, const char **eom) {
  return fl_shortcut_label(shortcut, eom);
}
unsigned int cfl_old_shortcut(const char *s) { return fl_old_shortcut(s); }
void cfl_overlay_rect(int x, int y, int w, int h) {
  return fl_overlay_rect(x, y, w, h);
}
void cfl_overlay_clear(void) { return fl_overlay_clear(); }
void cfl_set_cursor(int cursor) { return fl_cursor((Fl_Cursor)cursor); }
void cfl_set_cursor2(int cursor, int fg, int bg) {
  return fl_cursor((Fl_Cursor)cursor, fg, bg);
}
const char *cfl_expand_text(const char *from, char *buf, int maxbuf,
                            double maxw, int *n, double *width, int wrap,
                            int draw_symbols) {
  return fl_expand_text(from, buf, maxbuf, maxw, *n, *width, wrap, draw_symbols);
}
void cfl_set_status(int X, int Y, int W, int H) { fl_set_status(X, Y, W, H); }
void cfl_set_spot(int font, int size, int X, int Y, int W, int H, void *win) {
  fl_set_spot(font, size, X, Y, W, H, (Fl_Window *)win);
}
void cfl_reset_spot(void) { fl_reset_spot(); }
int cfl_draw_symbol(const char *label, int x, int y, int w, int h, int c) {
  return fl_draw_symbol(label, x, y, w, h, c);
}
