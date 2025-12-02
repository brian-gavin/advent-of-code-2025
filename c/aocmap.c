#include "include/aocmap.h"
#include <stddef.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

static size_t hash(const char *key, size_t key_len) {
  // if the key can fit in a size_t then it goes in the size_t
  if (key_len <= sizeof(size_t)) {
    size_t h = 0;
    memcpy(&h, key, key_len);
    return h;
  }
  fputs("panic: figure out a way to hash keys larger than 8 bytes.", stderr);
  abort();
}

static struct am_entry *get_entry(const struct am_bucket *bucket,
                                  const char *key, size_t key_len) {
  for (struct am_entry *cur = bucket->head; cur; cur = cur->next) {
    if (memcmp(cur->key, key, key_len) == 0) {
      return cur;
    }
  }
  return NULL;
}

static struct am_entry *remove_entry(struct am_bucket *bucket, const char *key,
                                     size_t key_len) {
  for (struct am_entry **cur = &bucket->head; *cur; cur = &(*cur)->next) {
    if (memcmp((*cur)->key, key, key_len) == 0) {
      struct am_entry *rv = *cur;
      *cur = (*cur)->next;
      return rv;
    }
  }
  return NULL;
}

static void insert_bucket(struct am_bucket *bucket, const char *key,
                          size_t key_len, void *val) {
  struct am_entry *entry = malloc(sizeof(struct am_entry));
  entry->next = bucket->head;
  entry->key = key;
  entry->key_len = key_len;
  entry->val = val;
  bucket->head = entry;
}

static void insert_or_set(struct am_bucket *bucket, const char *key,
                          size_t key_len, void *val) {
  struct am_entry *entry = get_entry(bucket, key, key_len);
  if (!entry) {
    insert_bucket(bucket, key, key_len, val);
    return;
  }
  entry->val = val;
}

void am_init(struct aocmap *am, size_t size) {
  am->buckets = (struct am_bucket *)calloc(size, sizeof(struct bucket *));
  am->buckets_len = size;
}

void am_fini(struct aocmap *am) {
  for (size_t i = 0; i < am->buckets_len; i++) {
    struct am_entry **head = &am->buckets[i].head;
    while (*head) {
      struct am_entry *tmp = *head;
      *head = (*head)->next;
      free(tmp);
    }
  }
  free(am->buckets);
}

void am_put(struct aocmap *am, const char *key, size_t key_len, void *val) {
  size_t bucket = hash(key, key_len) % am->buckets_len;
  insert_or_set(&am->buckets[bucket], key, key_len, val);
}

void *am_get(const struct aocmap *am, const char *key, size_t key_len) {
  size_t bucket = hash(key, key_len) % am->buckets_len;
  struct am_entry *entry = get_entry(&am->buckets[bucket], key, key_len);
  return entry ? entry->val : NULL;
}

void *am_del(struct aocmap *am, const char *key, size_t key_len) {
  size_t bucket = hash(key, key_len) % am->buckets_len;
  struct am_entry *entry = remove_entry(&am->buckets[bucket], key, key_len);
  void *val = NULL;
  if (entry) {
    val = entry->val;
    free(entry);
  }
  return val;
}

void am_print(const struct aocmap *am) {
  for (size_t i = 0; i < am->buckets_len; i++) {
    struct am_bucket bucket = am->buckets[i];
    printf("buckets[%zu]\n", i);
    for (struct am_entry *cur = bucket.head; cur; cur = cur->next) {
      printf("\tkey:%.*s\n\tval:%p\n", (int)cur->key_len, cur->key, cur->val);
    }
  }
}

static void first_entry(struct am_iter *iter) {
  if (iter->cur_bucket >= iter->am->buckets_len) {
    return;
  }
  iter->cur_ent = iter->am->buckets[iter->cur_bucket].head;
  if (!iter->cur_ent) {
    iter->cur_bucket++;
    first_entry(iter);
  }
}

struct am_iter am_iter_init(struct aocmap *am) {
  struct am_iter iter;
  iter.am = am;
  iter.cur_bucket = 0;
  first_entry(&iter);
  return iter;
}

int am_iter_done(struct am_iter iter) {
  return iter.cur_bucket == iter.am->buckets_len;
}

void am_iter_next(struct am_iter *iter) {
  if (iter->cur_ent) {
    iter->cur_ent = iter->cur_ent->next;
    if (iter->cur_ent) {
      return;
    }
  }
  iter->cur_bucket++;
  first_entry(iter);
}
