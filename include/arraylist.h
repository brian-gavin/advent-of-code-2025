#ifndef ARRAYLIST_H
#define ARRAYLIST_H

#include <stdlib.h>

#define AL_FULLFUNC(prefix, fn, typname, sep) prefix##sep##typname##sep##fn
#define AL_FUNC(fn, typname) AL_FULLFUNC(al, fn, typname, _)

#define DEFINE_ARRAYLIST(typname, typ)                                         \
                                                                               \
  struct arraylist_##typname {                                                 \
    typ *p;                                                                    \
    size_t len, cap;                                                           \
  };                                                                           \
                                                                               \
  void AL_FUNC(init, typname)(struct arraylist_##typname * al) {               \
    al->p = calloc(10, sizeof(typ));                                           \
    al->len = 0;                                                               \
    al->cap = 10;                                                              \
  }                                                                            \
                                                                               \
  void AL_FUNC(fini, typname)(struct arraylist_##typname * al) {               \
    free(al->p);                                                               \
    al->len = 0;                                                               \
    al->cap = 0;                                                               \
  }                                                                            \
                                                                               \
  void AL_FUNC(append, typname)(struct arraylist_##typname * al, typ t) {      \
    if (al->len >= al->cap) {                                                  \
      al->cap *= 2;                                                            \
      al->p = reallocarray(al->p, al->cap, sizeof(typ));                       \
    }                                                                          \
    al->p[al->len++] = t;                                                      \
  }                                                                            \
                                                                               \
  size_t AL_FUNC(len, typname)(struct arraylist_##typname * al) {              \
    return al->len;                                                            \
  }                                                                            \
  size_t AL_FUNC(cap, typname)(struct arraylist_##typname * al) {              \
    return al->cap;                                                            \
  }                                                                            \
  typ AL_FUNC(at, typname)(struct arraylist_##typname * al, size_t i) {        \
    if (i >= al->len) {                                                        \
      fputs("oob: al_at", stderr);                                             \
      abort();                                                                 \
    }                                                                          \
    return al->p[i];                                                           \
  }

#endif
