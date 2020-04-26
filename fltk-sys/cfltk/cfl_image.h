#pragma once

#ifdef __cplusplus
extern "C" {
#endif

#define IMAGE_DECLARE(image)                                                   \
  typedef struct image image;                                                  \
  image *image##_new(const char *filename);                                    \
  void image##_draw(image *, int X, int Y, int W, int H);                      \
  int image##_width(image *);                                                  \
  int image##_height(image *);                                                 \
  void image##_delete(image *);                                                \
  int image##_count(image *self);                                              \
  const char *const *image##_data(image *self);                                \
  image *image##_copy(image *self);

typedef struct Fl_Image Fl_Image;

IMAGE_DECLARE(Fl_JPEG_Image)

IMAGE_DECLARE(Fl_PNG_Image)

IMAGE_DECLARE(Fl_SVG_Image)

IMAGE_DECLARE(Fl_BMP_Image)

IMAGE_DECLARE(Fl_GIF_Image)

typedef struct Fl_RGB_Image Fl_RGB_Image;

Fl_RGB_Image *Fl_RGB_Image_new(const unsigned char *bits, int W, int H, int depth);

void Fl_RGB_Image_draw(Fl_RGB_Image *, int X, int Y, int W, int H);

int Fl_RGB_Image_width(Fl_RGB_Image *);

int Fl_RGB_Image_height(Fl_RGB_Image *);

void Fl_RGB_Image_delete(Fl_RGB_Image *);

int Fl_RGB_Image_count(Fl_RGB_Image *self);

const char *const *Fl_RGB_Image_data(Fl_RGB_Image *self);

Fl_RGB_Image *Fl_RGB_Image_copy(Fl_RGB_Image *self);

#ifdef __cplusplus
}
#endif