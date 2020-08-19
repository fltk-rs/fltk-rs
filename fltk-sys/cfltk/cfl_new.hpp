#pragma once

#include <stddef.h>

void *operator new(size_t size);

void *operator new[](size_t size);

void operator delete(void *)noexcept;

void operator delete[](void *)noexcept;

// extern "C" int __cxa_guard_acquire (long *);

// extern "C" void __cxa_guard_release (long *);

// extern "C" void __cxa_pure_virtual();

// extern "C" void __cxa_atexit();