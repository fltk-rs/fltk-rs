#pragma once

#ifdef __cplusplus
extern "C" {
#endif

#define IMAGE_DECLARE(image)                                                                       \
    typedef struct image image;                                                                    \
    void image##_draw(image *, int X, int Y, int W, int H);                                        \
    int image##_width(image *);                                                                    \
    int image##_height(image *);                                                                   \
    void image##_delete(image *);                                                                  \
    int image##_count(image *self);                                                                \
    const char *const *image##_data(image *self);                                                  \
    image *image##_copy(image *self);                                                              \
    void image##_scale(image *self, int width, int height, int proportional, int can_expand);      \
    int image##_fail(image *self);                                                                 \
    int image##_data_w(const image *self);                                                         \
    int image##_data_h(const image *self);                                                         \
    int image##_d(const image *self);                                                              \
    int image##_ld(const image *self);                                                             \
    void image##_inactive(image *self);

IMAGE_DECLARE(Fl_Image)

IMAGE_DECLARE(Fl_JPEG_Image)

Fl_JPEG_Image *Fl_JPEG_Image_new(const char *filename);

Fl_JPEG_Image *Fl_JPEG_Image_from(const unsigned char *data);

IMAGE_DECLARE(Fl_PNG_Image)

Fl_PNG_Image *Fl_PNG_Image_new(const char *filename);

Fl_PNG_Image *Fl_PNG_Image_from(const unsigned char *data, int size);

IMAGE_DECLARE(Fl_SVG_Image)

Fl_SVG_Image *Fl_SVG_Image_new(const char *filename);

Fl_SVG_Image *Fl_SVG_Image_from(const char *data);

IMAGE_DECLARE(Fl_BMP_Image)

Fl_BMP_Image *Fl_BMP_Image_new(const char *filename);

Fl_BMP_Image *Fl_BMP_Image_from(const unsigned char *data);

IMAGE_DECLARE(Fl_GIF_Image)

Fl_GIF_Image *Fl_GIF_Image_new(const char *filename);

Fl_GIF_Image *Fl_GIF_Image_from(const unsigned char *data);

IMAGE_DECLARE(Fl_RGB_Image)

Fl_RGB_Image *Fl_RGB_Image_new(const unsigned char *bits, int W, int H, int depth);

IMAGE_DECLARE(Fl_Shared_Image)

Fl_Shared_Image *Fl_Shared_Image_get(const char *name, int W, int H);

Fl_Shared_Image *Fl_Shared_Image_from_rgb(Fl_RGB_Image *rgb, int own_it);

int Fl_Shared_Image_fail(Fl_Shared_Image *self);

void Fl_register_images(void);

#ifdef __cplusplus
}
#endif