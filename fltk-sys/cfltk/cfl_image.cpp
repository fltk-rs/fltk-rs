#include "cfl_image.h"
#include <FL/Fl_BMP_Image.H>
#include <FL/Fl_GIF_Image.H>
#include <FL/Fl_Image.H>
#include <FL/Fl_JPEG_Image.H>
#include <FL/Fl_PNG_Image.H>
#include <FL/Fl_RGB_Image.H>
#include <FL/Fl_SVG_Image.H>
#include <cstring>
#include <new>
#include <string>

IMAGE_DEFINE(Fl_JPEG_Image)

IMAGE_DEFINE(Fl_PNG_Image)

IMAGE_DEFINE(Fl_SVG_Image)

IMAGE_DEFINE(Fl_BMP_Image)

IMAGE_DEFINE(Fl_GIF_Image)


Fl_RGB_Image* Fl_RGB_Image_new(const unsigned char *bits, int W, int H) {
    return new (std::nothrow) Fl_RGB_Image(bits, W, H);
}
