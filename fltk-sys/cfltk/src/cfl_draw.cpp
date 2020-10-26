#include "cfl_draw.h"
#include <FL/Enumerations.H>
#include <FL/Fl.H>
#include <FL/Fl_Image.H>
#include <FL/Fl_Window.H>
#include <FL/fl_draw.H>
#include <FL/fl_show_colormap.H>
#include <jpeg/jpeglib.h>
#include <png/png.h>

void Fl_set_color_int(unsigned int c) {
    fl_color(c);
}

void Fl_set_color_rgb(unsigned char r, unsigned char g, unsigned char b) {
    fl_color(r, g, b);
}

unsigned int Fl_get_color(void) {
    return fl_color();
}

void Fl_push_clip(int x, int y, int w, int h) {
    fl_push_clip(x, y, w, h);
}

void Fl_push_no_clip(void) {
    fl_push_no_clip();
}

void Fl_pop_clip(void) {
    fl_pop_clip();
}

int Fl_not_clipped(int x, int y, int w, int h) {
    return fl_not_clipped(x, y, w, h);
}

int Fl_clip_box(int x, int y, int w, int h, int *X, int *Y, int *W, int *H) {
    return fl_clip_box(x, y, w, h, *X, *Y, *W, *H);
}

void Fl_restore_clip(void) {
    fl_restore_clip();
}

void Fl_set_clip_region(void *r) {
    fl_clip_region((Fl_Region)r);
}

void *Fl_clip_region(void) {
    return (void *)fl_clip_region();
}

void Fl_point(int x, int y) {
    fl_point(x, y);
}

void Fl_line_style(int style, int width, char *dashes) {
    fl_line_style(style, width, dashes);
}

void Fl_rect(int x, int y, int w, int h) {
    fl_rect(x, y, w, h);
}

void Fl_focus_rect(int x, int y, int w, int h) {
    fl_focus_rect(x, y, w, h);
}

void Fl_rect_with_color(int x, int y, int w, int h, unsigned int c) {
    fl_rect(x, y, w, h, c);
}

void Fl_rectf(int x, int y, int w, int h) {
    fl_rectf(x, y, w, h);
}

void Fl_rectf_with_color(int x, int y, int w, int h, unsigned int c) {
    fl_rectf(x, y, w, h, c);
}

void Fl_rectf_with_rgb(int x, int y, int w, int h, unsigned char r, unsigned char g,
                       unsigned char b) {
    fl_rectf(x, y, w, h, r, g, b);
}

void Fl_line(int x, int y, int x1, int y1) {
    fl_line(x, y, x1, y1);
}

void Fl_line2(int x, int y, int x1, int y1, int x2, int y2) {
    fl_line(x, y, x1, y1, x2, y2);
}

void Fl_loop(int x, int y, int x1, int y1, int x2, int y2) {
    fl_loop(x, y, x1, y1, x2, y2);
}

void Fl_loop2(int x, int y, int x1, int y1, int x2, int y2, int x3, int y3) {
    fl_loop(x, y, x1, y1, x2, y2, x3, y3);
}

void Fl_polygon(int x, int y, int x1, int y1, int x2, int y2) {
    fl_polygon(x, y, x1, y1, x2, y2);
}

void Fl_polygon2(int x, int y, int x1, int y1, int x2, int y2, int x3, int y3) {
    fl_polygon(x, y, x1, y1, x2, y2, x3, y3);
}

void Fl_xyline(int x, int y, int x1) {
    fl_xyline(x, y, x1);
}

void Fl_xyline2(int x, int y, int x1, int y2) {
    fl_xyline(x, y, x1, y2);
}

void Fl_xyline3(int x, int y, int x1, int y2, int x3) {
    fl_xyline(x, y, x1, y2, x3);
}

void Fl_yxline(int x, int y, int y1) {
    fl_yxline(x, y, y1);
}

void Fl_yxline2(int x, int y, int y1, int x2) {
    fl_yxline(x, y, y1, x2);
}

void Fl_yxline3(int x, int y, int y1, int x2, int y3) {
    fl_yxline(x, y, y1, x2, y3);
}

void Fl_arc(int x, int y, int w, int h, double a1, double a2) {
    fl_arc(x, y, w, h, a1, a2);
}

void Fl_pie(int x, int y, int w, int h, double a1, double a2) {
    fl_pie(x, y, w, h, a1, a2);
}

void Fl_push_matrix(void) {
    fl_push_matrix();
}

void Fl_pop_matrix(void) {
    fl_pop_matrix();
}

void Fl_scale(double x, double y) {
    fl_scale(x, y);
}

void Fl_scale2(double x) {
    fl_scale(x);
}

void Fl_translate(double x, double y) {
    fl_translate(x, y);
}

void Fl_rotate(double d) {
    fl_rotate(d);
}

void Fl_mult_matrix(double a, double b, double c, double d, double x, double y) {
    fl_mult_matrix(a, b, c, d, x, y);
}

void Fl_begin_points(void) {
    fl_begin_points();
}

void Fl_begin_line(void) {
    fl_begin_line();
}

void Fl_begin_loop(void) {
    fl_begin_loop();
}

void Fl_begin_polygon(void) {
    fl_begin_polygon();
}

void Fl_vertex(double x, double y) {
    fl_vertex(x, y);
}

void Fl_curve(double X0, double Y0, double X1, double Y1, double X2, double Y2, double X3,
              double Y3) {
    fl_curve(X0, Y0, X1, Y1, X2, Y2, X3, Y3);
}

void Fl_arc2(double x, double y, double r, double start, double end) {
    fl_arc(x, y, r, start, end);
}

void Fl_circle(double x, double y, double r) {
    fl_circle(x, y, r);
}

void Fl_end_points(void) {
    fl_end_points();
}

void Fl_end_line(void) {
    fl_end_line();
}

void Fl_end_loop(void) {
    fl_end_loop();
}

void Fl_end_polygon(void) {
    fl_end_polygon();
}

void Fl_begin_complex_polygon(void) {
    fl_begin_complex_polygon();
}

void Fl_gap(void) {
    fl_gap();
}

void Fl_end_complex_polygon(void) {
    fl_end_complex_polygon();
}

double Fl_transform_x(double x, double y) {
    return fl_transform_x(x, y);
}

double Fl_transform_y(double x, double y) {
    return fl_transform_y(x, y);
}

double Fl_transform_dx(double x, double y) {
    return fl_transform_dx(x, y);
}

double Fl_transform_dy(double x, double y) {
    return fl_transform_dy(x, y);
}

void Fl_transformed_vertex(double xf, double yf) {
    fl_transformed_vertex(xf, yf);
}

void Fl_set_font(int face, int fsize) {
    fl_font(face, fsize);
}

int Fl_font(void) {
    return fl_font();
}

int Fl_size(void) {
    return fl_size();
}

int Fl_height(void) {
    return fl_height();
}

int Fl_set_height(int font, int size) {
    return fl_height(font, size);
}

int Fl_descent(void) {
    return fl_descent();
}

double Fl_width(const char *txt) {
    return fl_width(txt);
}

double Fl_width2(const char *txt, int n) {
    return fl_width(txt, n);
}

double Fl_width3(unsigned int c) {
    return fl_width(c);
}

void Fl_text_extents(const char *txt, int *dx, int *dy, int *w, int *h) {
    return fl_text_extents(txt, *dx, *dy, *w, *h);
}

void Fl_text_extents2(const char *t, int n, int *dx, int *dy, int *w, int *h) {
    return fl_text_extents(t, n, *dx, *dy, *w, *h);
}

const char *Fl_latin1_to_local(const char *t, int n) {
    return fl_latin1_to_local(t, n);
}

const char *Fl_local_to_latin1(const char *t, int n) {
    return fl_local_to_latin1(t, n);
}

const char *Fl_mac_roman_to_local(const char *t, int n) {
    return fl_mac_roman_to_local(t, n);
}

const char *Fl_local_to_mac_roman(const char *t, int n) {
    return fl_local_to_mac_roman(t, n);
}

void Fl_draw(const char *str, int x, int y) {
    fl_draw(str, x, y);
}

void Fl_draw2(int angle, const char *str, int x, int y) {
    fl_draw(angle, str, x, y);
}

void Fl_draw3(const char *str, int n, int x, int y) {
    fl_draw(str, n, x, y);
}

void Fl_draw4(int angle, const char *str, int n, int x, int y) {
    fl_draw(angle, str, n, x, y);
}

void Fl_rtl_draw(const char *str, int n, int x, int y) {
    fl_rtl_draw(str, n, x, y);
}

void Fl_measure(const char *str, int *x, int *y, int draw_symbols) {
    fl_measure(str, *x, *y, draw_symbols);
}

void Fl_draw5(const char *str, int x, int y, int w, int h, int align, void **img,
              int draw_symbols) {
    fl_draw(str, x, y, w, h, align, (Fl_Image *)*img, draw_symbols);
}

void Fl_frame(const char *s, int x, int y, int w, int h) {
    fl_frame(s, x, y, w, h);
}

void Fl_frame2(const char *s, int x, int y, int w, int h) {
    fl_frame2(s, x, y, w, h);
}

void Fl_draw_box(int box_type, int x, int y, int w, int h, unsigned int c) {
    fl_draw_box((Fl_Boxtype)box_type, x, y, w, h, c);
}

void Fl_draw_image(const unsigned char *buf, int X, int Y, int W, int H, int D, int L) {
    fl_draw_image(buf, X, Y, W, H, D, L);
}

void Fl_draw_image_mono(const unsigned char *buf, int X, int Y, int W, int H, int D, int L) {
    fl_draw_image_mono(buf, X, Y, W, H, D, L);
}
char Fl_can_do_alpha_blending(void) {
    return fl_can_do_alpha_blending();
}

unsigned char *Fl_read_image(unsigned char *p, int X, int Y, int W, int H, int alpha) {
    return fl_read_image(p, X, Y, W, H, alpha);
}

unsigned char *Fl_capture_window_part(void *win, int x, int y, int w, int h) {
    Fl_RGB_Image *tmp = fl_capture_window_part((Fl_Window *)win, x, y, w, h);
    return (unsigned char *)tmp->data();
}

int Fl_draw_pixmap(const char *const *data, int x, int y, int bg) {
    return fl_draw_pixmap(data, x, y, bg);
}

int Fl_draw_pixmap2(/*const*/ char *const *data, int x, int y, int bg) {
    return fl_draw_pixmap(data, x, y, bg);
}

int Fl_measure_pixmap(/*const*/ char *const *data, int *w, int *h) {
    return fl_measure_pixmap(data, *w, *h);
}

int Fl_measure_pixmap2(const char *const *cdata, int *w, int *h) {
    return fl_measure_pixmap(cdata, *w, *h);
}

const char *Fl_shortcut_label(unsigned int shortcut) {
    return fl_shortcut_label(shortcut);
}

const char *Fl_shortcut_label2(unsigned int shortcut, const char **eom) {
    return fl_shortcut_label(shortcut, eom);
}

unsigned int Fl_old_shortcut(const char *s) {
    return fl_old_shortcut(s);
}

void Fl_overlay_rect(int x, int y, int w, int h) {
    return fl_overlay_rect(x, y, w, h);
}

void Fl_overlay_clear(void) {
    return fl_overlay_clear();
}

void Fl_set_cursor(int cursor) {
    return fl_cursor((Fl_Cursor)cursor);
}

void Fl_set_cursor2(int cursor, int fg, int bg) {
    return fl_cursor((Fl_Cursor)cursor, fg, bg);
}

const char *Fl_expand_text(const char *from, char *buf, int maxbuf, double maxw, int *n,
                           double *width, int wrap, int draw_symbols) {
    return fl_expand_text(from, buf, maxbuf, maxw, *n, *width, wrap, draw_symbols);
}

void Fl_set_status(int X, int Y, int W, int H) {
    fl_set_status(X, Y, W, H);
}

void Fl_set_spot(int font, int size, int X, int Y, int W, int H, void *win) {
    fl_set_spot(font, size, X, Y, W, H, (Fl_Window *)win);
}

void Fl_reset_spot(void) {
    fl_reset_spot();
}

unsigned int Fl_show_colormap(unsigned int old_col) {
    return fl_show_colormap((Fl_Color)old_col);
}

void Fl_copy_offscreen(int x, int y, int w, int h, void *pixmap, int srcx, int srcy) {
    fl_copy_offscreen(x, y, w, h, (Fl_Offscreen)pixmap, srcx, srcy);
}

void *Fl_create_offscreen(int w, int h) {
    return (void *)fl_create_offscreen(w, h);
}

void Fl_begin_offscreen(void *b) {
    fl_begin_offscreen((Fl_Offscreen)b);
}

void Fl_end_offscreen(void) {
    fl_end_offscreen();
}

void Fl_delete_offscreen(void *bitmap) {
    fl_delete_offscreen((Fl_Offscreen)bitmap);
}

void Fl_rescale_offscreen(void *ctx) {
    fl_rescale_offscreen(*(Fl_Offscreen *)ctx);
}

void Fl_draw_text2(const char *str, int x, int y, int w, int h, int align) {
    fl_draw(str, x, y, w, h, (Fl_Align)align, 0, 1);
}

// The following code was copied from stackoverflow
// https://stackoverflow.com/questions/1821806/how-to-encode-png-to-buffer-using-libpng/
int Fl_raw_image_to_png(unsigned char *data, const char *fname, int w, int h) {
    if (!data || !fname)
        return -1;

    FILE *fp;

    if ((fp = fopen(fname, "wb")) == NULL) {
        return -1;
    }

    png_structp pptr = png_create_write_struct(PNG_LIBPNG_VER_STRING, 0, 0, 0);
    if (!pptr)
        return -1;
    png_infop iptr = png_create_info_struct(pptr);
    if (!iptr)
        return -1;
    png_bytep ptr = (png_bytep)data;

    png_init_io(pptr, fp);
    png_set_IHDR(pptr, iptr, w, h, 8, PNG_COLOR_TYPE_RGB, PNG_INTERLACE_NONE,
                 PNG_COMPRESSION_TYPE_DEFAULT, PNG_FILTER_TYPE_DEFAULT);
    png_set_sRGB(pptr, iptr, PNG_sRGB_INTENT_PERCEPTUAL);

    png_write_info(pptr, iptr);

    for (int i = h; i > 0; i--, ptr += w * 3) {
        png_write_row(pptr, ptr);
    }

    png_write_end(pptr, iptr);
    png_destroy_write_struct(&pptr, &iptr);

    fclose(fp);
    return 0;
}

int Fl_raw_image_to_jpg(unsigned char *data, const char *fname, int w, int h) {
    if (!data || !fname)
        return -1;
    struct jpeg_compress_struct cinfo;
    struct jpeg_error_mgr jerr;

    JSAMPROW row_pointer[1];
    FILE *outfile = fopen(fname, "wb");

    if (!outfile) {
        return -1;
    }
    cinfo.err = jpeg_std_error(&jerr);
    jpeg_create_compress(&cinfo);
    jpeg_stdio_dest(&cinfo, outfile);

    cinfo.image_width = w;
    cinfo.image_height = h;
    cinfo.input_components = 3;
    cinfo.in_color_space = JCS_RGB;

    jpeg_set_defaults(&cinfo);

    jpeg_start_compress(&cinfo, (boolean)1);

    while (cinfo.next_scanline < cinfo.image_height) {
        row_pointer[0] = &data[cinfo.next_scanline * cinfo.image_width * cinfo.input_components];
        jpeg_write_scanlines(&cinfo, row_pointer, 1);
    }

    jpeg_finish_compress(&cinfo);
    jpeg_destroy_compress(&cinfo);
    fclose(outfile);

    return 0;
}

int Fl_raw_image_to_bmp(unsigned char *data, const char *fname, int w, int h) {
    if (!data || !fname)
        return -1;
    FILE *f;
    int filesize = 54 + 3 * w * h;

    unsigned char bmpfileheader[14] = {'B', 'M', 0, 0, 0, 0, 0, 0, 0, 0, 54, 0, 0, 0};
    unsigned char bmpinfoheader[40] = {40, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 24, 0};
    unsigned char bmppad[3] = {0, 0, 0};

    bmpfileheader[2] = (unsigned char)(filesize);
    bmpfileheader[3] = (unsigned char)(filesize >> 8);
    bmpfileheader[4] = (unsigned char)(filesize >> 16);
    bmpfileheader[5] = (unsigned char)(filesize >> 24);

    bmpinfoheader[4] = (unsigned char)(w);
    bmpinfoheader[5] = (unsigned char)(w >> 8);
    bmpinfoheader[6] = (unsigned char)(w >> 16);
    bmpinfoheader[7] = (unsigned char)(w >> 24);
    bmpinfoheader[8] = (unsigned char)(h);
    bmpinfoheader[9] = (unsigned char)(h >> 8);
    bmpinfoheader[10] = (unsigned char)(h >> 16);
    bmpinfoheader[11] = (unsigned char)(h >> 24);

    f = fopen(fname, "wb");
    if (!f)
        return -1;
    fwrite(bmpfileheader, 1, 14, f);
    fwrite(bmpinfoheader, 1, 40, f);
    for (int i = 0; i < h; i++) {
        fwrite(data + (w * (h - i - 1) * 3), 3, w, f);
        fwrite(bmppad, 1, (4 - (w * 3) % 4) % 4, f);
    }

    fclose(f);
    return 0;
}