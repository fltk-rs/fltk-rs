#include "cfl_new.hpp"

#include <stdlib.h>

void *operator new(size_t size) {
    return malloc(size);
}

void *operator new[](size_t size) {
    return malloc(size);
}

void operator delete(void *val) noexcept {
    free(val);
}

void operator delete[](void *val) noexcept {
    free(val);
}

#ifndef __ANDROID__
void *__gxx_personality_v0 = 0;
#endif

extern "C" int __cxa_guard_acquire(long *g) {
    return !*(char *)(g);
}

extern "C" void __cxa_guard_release(long *g) {
    *(char *)g = 1;
}

extern "C" void __cxa_pure_virtual() {
    // Do nothing
}

extern "C" void __cxa_guard_abort(long *) {
    // Do nothing
}

extern "C" void __cxa_atexit() {
    // Do nothing
}