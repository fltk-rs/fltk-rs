#pragma once

#ifdef __APPLE__
#include <OpenGL/glu.h>
#else
#include <GL/glu.h>
#endif

void gl_start(void);

void gl_finish(void);

