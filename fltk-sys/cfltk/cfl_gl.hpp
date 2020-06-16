#pragma once

#ifdef __APPLE__
#include <OpenGL/glu.h>
#else
#include <GL/glu.h>
#endif

#ifdef __cplusplus
extern "C" {
#endif

void cgl_start(void);

void cgl_finish(void);

#ifdef __cplusplus
}
#endif
