#pragma once

#ifdef __cplusplus
extern "C" {
#endif

#define IMAGE_DECLARE(image)                                                   \
    typedef struct image image;                                                \
    void image##_draw(image *, int X, int Y, int W, int H);                    \
    int image##_width(image *);                                                \
    int image##_height(image *);                                               \
    void image##_delete(image *);                                              \
    int image##_count(image *self);                                            \
    const char *const *image##_data(image *self);                              \
    image *image##_copy(image *self);

typedef struct Fl_Image Fl_Image;

IMAGE_DECLARE(Fl_JPEG_Image)

Fl_JPEG_Image *Fl_JPEG_Image_new(const char *filename);

IMAGE_DECLARE(Fl_PNG_Image)

Fl_PNG_Image *Fl_PNG_Image_new(const char *filename);

IMAGE_DECLARE(Fl_SVG_Image)

Fl_SVG_Image *Fl_SVG_Image_new(const char *filename);

IMAGE_DECLARE(Fl_BMP_Image)

Fl_BMP_Image *Fl_BMP_Image_new(const char *filename);

IMAGE_DECLARE(Fl_GIF_Image)

Fl_GIF_Image *Fl_GIF_Image_new(const char *filename);

IMAGE_DECLARE(Fl_RGB_Image)

Fl_RGB_Image *Fl_RGB_Image_new(const unsigned char *bits, int W, int H,
                               int depth);

#ifdef __cplusplus
}
#endif