#include "cfl_image.h"
#include <FL/Fl.H>

#include "cfl_new.hpp"
#include <FL/Fl_BMP_Image.H>
#include <FL/Fl_GIF_Image.H>
#include <FL/Fl_Image.H>
#include <FL/Fl_JPEG_Image.H>
#include <FL/Fl_PNG_Image.H>
#include <FL/Fl_PNM_Image.H>
#include <FL/Fl_Pixmap.H>
#include <FL/Fl_RGB_Image.H>
#include <FL/Fl_SVG_Image.H>
#include <FL/Fl_Shared_Image.H>
#include <FL/Fl_Tiled_Image.H>
#include <FL/Fl_XBM_Image.H>
#include <FL/Fl_XPM_Image.H>

#ifndef LOCK
#define LOCK(x)                                                                                    \
    Fl::lock();                                                                                    \
    x;                                                                                             \
    Fl::unlock();                                                                                  \
    Fl::awake();
#endif

#define IMAGE_DEFINE(image)                                                                        \
    void image##_draw(image *self, int X, int Y, int W, int H) {                                   \
        LOCK(self->draw(X, Y, W, H);)                                                              \
    }                                                                                              \
    int image##_width(image *self) {                                                               \
        return self->w();                                                                          \
    }                                                                                              \
    int image##_height(image *self) {                                                              \
        return self->h();                                                                          \
    }                                                                                              \
    void image##_delete(image *self) {                                                             \
        delete self;                                                                               \
    }                                                                                              \
    int image##_count(image *self) {                                                               \
        return self->count();                                                                      \
    }                                                                                              \
    const char *const *image##_data(image *self) {                                                 \
        return self->data();                                                                       \
    }                                                                                              \
    image *image##_copy(image *self) {                                                             \
        return (image *)self->copy();                                                              \
    }                                                                                              \
    void image##_scale(image *self, int width, int height, int proportional, int can_expand) {     \
        LOCK(self->scale(width, height, proportional, can_expand);)                                \
    }                                                                                              \
    int image##_fail(image *self) {                                                                \
        return self->fail();                                                                       \
    }                                                                                              \
    int image##_data_w(const image *self) {                                                        \
        return self->data_w();                                                                     \
    }                                                                                              \
    int image##_data_h(const image *self) {                                                        \
        return self->data_h();                                                                     \
    }                                                                                              \
    int image##_d(const image *self) {                                                             \
        return self->d();                                                                          \
    }                                                                                              \
    int image##_ld(const image *self) {                                                            \
        return self->ld();                                                                         \
    }                                                                                              \
    void image##_inactive(image *self) {                                                           \
        LOCK(self->inactive();)                                                                    \
    }

IMAGE_DEFINE(Fl_JPEG_Image)

Fl_JPEG_Image *Fl_JPEG_Image_new(const char *filename) {
    return new Fl_JPEG_Image(filename);
}

Fl_JPEG_Image *Fl_JPEG_Image_from(const unsigned char *data) {
    return new Fl_JPEG_Image(NULL, data);
}

IMAGE_DEFINE(Fl_Image)

IMAGE_DEFINE(Fl_PNG_Image)

Fl_PNG_Image *Fl_PNG_Image_new(const char *filename) {
    return new Fl_PNG_Image(filename);
}

Fl_PNG_Image *Fl_PNG_Image_from(const unsigned char *data, int size) {
    return new Fl_PNG_Image(NULL, data, size);
}

IMAGE_DEFINE(Fl_SVG_Image)

Fl_SVG_Image *Fl_SVG_Image_new(const char *filename) {
    return new Fl_SVG_Image(filename);
}

Fl_SVG_Image *Fl_SVG_Image_from(const char *data) {
    return new Fl_SVG_Image(NULL, data);
}

IMAGE_DEFINE(Fl_BMP_Image)

Fl_BMP_Image *Fl_BMP_Image_new(const char *filename) {
    return new Fl_BMP_Image(filename);
}

Fl_BMP_Image *Fl_BMP_Image_from(const unsigned char *data) {
    return new Fl_BMP_Image(NULL, data);
}

IMAGE_DEFINE(Fl_GIF_Image)

Fl_GIF_Image *Fl_GIF_Image_new(const char *filename) {
    return new Fl_GIF_Image(filename);
}

Fl_GIF_Image *Fl_GIF_Image_from(const unsigned char *data) {
    return new Fl_GIF_Image(NULL, data);
}

IMAGE_DEFINE(Fl_Pixmap)

Fl_Pixmap *Fl_Pixmap_new(const unsigned char *const *D) {
    return new Fl_Pixmap(D);
}

IMAGE_DEFINE(Fl_XPM_Image)

Fl_XPM_Image *Fl_XPM_Image_new(const char *filename) {
    return new Fl_XPM_Image(filename);
}

IMAGE_DEFINE(Fl_XBM_Image)

Fl_XBM_Image *Fl_XBM_Image_new(const char *filename) {
    return new Fl_XBM_Image(filename);
}

IMAGE_DEFINE(Fl_PNM_Image)

Fl_PNM_Image *Fl_PNM_Image_new(const char *filename) {
    return new Fl_PNM_Image(filename);
}

IMAGE_DEFINE(Fl_Tiled_Image)

Fl_Tiled_Image *Fl_Tiled_Image_new(Fl_Image *i, int w, int h) {
    return new Fl_Tiled_Image(i, w, h);
}

IMAGE_DEFINE(Fl_RGB_Image)

Fl_RGB_Image *Fl_RGB_Image_new(const unsigned char *bits, int W, int H, int depth) {
    auto img = new Fl_RGB_Image(bits, W, H, depth);
    img->alloc_array = 1;
    return img;
}

void Fl_Shared_Image_draw(Fl_Shared_Image *self, int X, int Y, int W, int H) {
    LOCK(self->draw(X, Y, W, H);)
}

int Fl_Shared_Image_width(Fl_Shared_Image *self) {
    return self->w();
}

int Fl_Shared_Image_height(Fl_Shared_Image *self) {
    return self->h();
}

void Fl_Shared_Image_delete(Fl_Shared_Image *self) {
    self->release();
}

int Fl_Shared_Image_count(Fl_Shared_Image *self) {
    return self->count();
}

const char *const *Fl_Shared_Image_data(Fl_Shared_Image *self) {
    return self->data();
}

Fl_Shared_Image *Fl_Shared_Image_copy(Fl_Shared_Image *self) {
    return (Fl_Shared_Image *)self->copy();
}

void Fl_Shared_Image_scale(Fl_Shared_Image *self, int width, int height, int proportional,
                           int can_expand) {
    LOCK(self->scale(width, height, proportional, can_expand);)
}

Fl_Shared_Image *Fl_Shared_Image_get(const char *name, int W, int H) {
    return Fl_Shared_Image::get(name, W, H);
}

Fl_Shared_Image *Fl_Shared_Image_from_rgb(Fl_RGB_Image *rgb, int own_it) {
    return Fl_Shared_Image::get(rgb, own_it);
}

int Fl_Shared_Image_fail(Fl_Shared_Image *self) {
    return self->fail();
}

int Fl_Shared_Image_data_w(const Fl_Shared_Image *self) {
    return self->data_w();
}

int Fl_Shared_Image_data_h(const Fl_Shared_Image *self) {
    return self->data_h();
}

int Fl_Shared_Image_d(const Fl_Shared_Image *self) {
    return self->d();
}

int Fl_Shared_Image_ld(const Fl_Shared_Image *self) {
    return self->ld();
}

void Fl_Shared_Image_inactive(Fl_Shared_Image *self) {
    LOCK(self->inactive();)
}

void Fl_register_images() {
    fl_register_images();
}

