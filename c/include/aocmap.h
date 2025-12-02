#ifndef AOCMAP_H
#define AOCMAP_H

#include <stddef.h>

struct am_entry {
  struct am_entry *next;
  const char *key;
  size_t key_len;
  void *val;
};

struct am_bucket {
  struct am_entry *head;
};

struct aocmap {
  struct am_bucket *buckets;
  size_t buckets_len;
};

// inits am by allocating a bucket array.
void am_init(struct aocmap *am, size_t size);

// deinitializes am, freeing the buckets all their entries, but not the keys or values.
void am_fini(struct aocmap *am);

// puts the value in the map with key. note that it does not take ownership of key or val,
// the caller must manage this memory.
void am_put(struct aocmap *cm, const char *key, size_t key_len,
            void *val);

// gets the entry of key
void *am_get(const struct aocmap *am, const char *key, size_t key_len);

// deletes the entry of key, and returns its associated val.
void *am_del(struct aocmap *am, const char *key, size_t key_len);

// debug prints the map
void am_print(const struct aocmap *cm);

#define am_foreach(am, iter)                                                   \
  for (struct am_iter iter = am_iter_init(&am); !am_iter_done(iter);           \
       am_iter_next(&iter))

struct am_iter {
  struct aocmap *am;
  size_t cur_bucket;
  struct am_entry *cur_ent;
};

struct am_iter am_iter_init(struct aocmap *am);
int am_iter_done(struct am_iter iter);
void am_iter_next(struct am_iter *iter);

#endif
