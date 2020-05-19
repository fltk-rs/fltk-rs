#include "cfl_image.h"
#include <FL/Fl.H>
#include <FL/Fl_BMP_Image.H>
#include <FL/Fl_GIF_Image.H>
#include <FL/Fl_Image.H>
#include <FL/Fl_JPEG_Image.H>
#include <FL/Fl_PNG_Image.H>
#include <FL/Fl_RGB_Image.H>
#include <FL/Fl_SVG_Image.H>
#include <new>

#ifndef LOCK
#define LOCK(x)                                                                \
    Fl::lock();                                                                \
    x;                                                                         \
    Fl::unlock();                                                              \
    Fl::awake();
#endif

#define IMAGE_DEFINE(image)                                                    \
    void image##_draw(image *self, int X, int Y, int W, int H) {               \
        LOCK(self->draw(X, Y, W, H);)                                          \
    }                                                                          \
    int image##_width(image *self) { return self->w(); }                       \
    int image##_height(image *self) { return self->h(); }                      \
    void image##_delete(image *self) { delete self; }                          \
    int image##_count(image *self) { return self->count(); }                   \
    const char *const *image##_data(image *self) { return self->data(); }      \
    image *image##_copy(image *self) { return (image *)self->copy(); }

IMAGE_DEFINE(Fl_JPEG_Image)

Fl_JPEG_Image *Fl_JPEG_Image_new(const char *filename) {
    return new (std::nothrow) Fl_JPEG_Image(filename);
}

Fl_JPEG_Image *Fl_JPEG_Image_from(const unsigned char *data) {
    return new (std::nothrow) Fl_JPEG_Image(NULL, data);
}

IMAGE_DEFINE(Fl_PNG_Image)

Fl_PNG_Image *Fl_PNG_Image_new(const char *filename) {
    return new (std::nothrow) Fl_PNG_Image(filename);
}

Fl_PNG_Image *Fl_PNG_Image_from(const unsigned char *data, int size) {
    return new (std::nothrow) Fl_PNG_Image(NULL, data, size);
}

IMAGE_DEFINE(Fl_SVG_Image)

Fl_SVG_Image *Fl_SVG_Image_new(const char *filename) {
    return new (std::nothrow) Fl_SVG_Image(filename);
}

Fl_SVG_Image *Fl_SVG_Image_from(const char *data) {
    return new (std::nothrow) Fl_SVG_Image(NULL, data);
}

IMAGE_DEFINE(Fl_BMP_Image)

Fl_BMP_Image *Fl_BMP_Image_new(const char *filename) {
    return new (std::nothrow) Fl_BMP_Image(filename);
}

Fl_BMP_Image *Fl_BMP_Image_from(const unsigned char *data) {
    return new (std::nothrow) Fl_BMP_Image(NULL, data);
}

IMAGE_DEFINE(Fl_GIF_Image)

Fl_GIF_Image *Fl_GIF_Image_new(const char *filename) {
    return new (std::nothrow) Fl_GIF_Image(filename);
}

Fl_GIF_Image *Fl_GIF_Image_from(const unsigned char *data) {
    return new (std::nothrow) Fl_GIF_Image(NULL, data);
}

IMAGE_DEFINE(Fl_RGB_Image)
Fl_RGB_Image *Fl_RGB_Image_new(const unsigned char *bits, int W, int H,
                               int depth) {
    return new (std::nothrow) Fl_RGB_Image(bits, W, H, depth);
}

// void Fl_RGB_Image_draw(Fl_RGB_Image *self, int X, int Y, int W, int H) {
//   self->draw(X, Y, W, H);
// }

// int Fl_RGB_Image_width(Fl_RGB_Image *self) { return self->w(); }

// int Fl_RGB_Image_height(Fl_RGB_Image *self) { return self->h(); }

// void Fl_RGB_Image_delete(Fl_RGB_Image *self) { delete self; }

// int Fl_RGB_Image_count(Fl_RGB_Image *self) { return self->count(); }

// const char *const *Fl_RGB_Image_data(Fl_RGB_Image *self) {
//   return self->data();
// }

// Fl_RGB_Image *Fl_RGB_Image_copy(Fl_RGB_Image *self) {
//   return (Fl_RGB_Image *)self->copy();
// }
