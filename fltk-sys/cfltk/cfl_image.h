#pragma once

#include "global.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef struct Fl_Image Fl_Image;

IMAGE_DECLARE(Fl_JPEG_Image)

IMAGE_DECLARE(Fl_PNG_Image)

IMAGE_DECLARE(Fl_SVG_Image)

IMAGE_DECLARE(Fl_BMP_Image)

IMAGE_DECLARE(Fl_GIF_Image)

typedef struct Fl_RGB_Image Fl_RGB_Image;

Fl_RGB_Image* Fl_RGB_Image_new(const unsigned char *bits, int W, int H);

#ifdef __cplusplus
}
#endif