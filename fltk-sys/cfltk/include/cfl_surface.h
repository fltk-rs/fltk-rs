#pragma once

#ifdef __cplusplus
extern "C" {
#endif

typedef struct Fl_Surface_Device Fl_Surface_Device;

void Fl_Surface_Device_set_current(Fl_Surface_Device *self);

int Fl_Surface_Device_is_current(Fl_Surface_Device *self);

Fl_Surface_Device *Fl_Surface_Device_surface(void);

void Fl_Surface_Device_push_current(Fl_Surface_Device *new_current);

Fl_Surface_Device *Fl_Surface_Device_pop_current(void);

typedef struct Fl_Image_Surface Fl_Image_Surface;

Fl_Image_Surface *Fl_Image_Surface_new(int w, int h, int high_res);

void Fl_Image_Surface_delete(Fl_Image_Surface *s);

void Fl_Image_Surface_set_current(Fl_Image_Surface *self);

int Fl_Image_Surface_is_current(Fl_Image_Surface *self);

void *Fl_Image_Surface_image(Fl_Image_Surface *self);

void *Fl_Image_Surface_highres_image(Fl_Image_Surface *self);

void Fl_Image_Surface_origin(Fl_Image_Surface *self, int *x, int *y);

void Fl_Image_Surface_set_origin(Fl_Image_Surface *self, int x, int y);

void Fl_Image_Surface_rescale(Fl_Image_Surface *self);

void Fl_Image_Surface_draw(Fl_Image_Surface *self, void* widget, int delta_x, int delta_y);

#ifdef __cplusplus
}
#endif