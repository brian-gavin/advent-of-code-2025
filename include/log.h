#ifndef LOG_H
#define LOG_H

#include <stdio.h>

#ifdef DEBUG
#define debug(msg, ...) printf(msg, __VA_ARGS__)
#else
#define debug(msg, ...)
#endif

#endif
