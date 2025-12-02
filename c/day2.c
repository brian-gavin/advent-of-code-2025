#include "include/log.h"
#include <stdbool.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <strings.h>

struct range {
  size_t start, end;
};

struct range parse_range(const char *s) {
  struct range r;
  char *delim = NULL;
  r.start = strtoul(s, &delim, 10);
  r.end = strtoul(delim++, NULL, 10);
  return r;
}

bool is_invalid_id(size_t n) {
  debug("n: %zu\n", n);
  char buf[255];
  snprintf(buf, 255, "%zu", n);
  for (char *first = buf; *first; first++) {
    for (char *second = first + 1; *second; second++) {
      debug("first: %.*s | second: %s\n", (int)(second - first), first, second);
      if (strncmp(first, second, second - first) == 0) {
        return true;
      }
    }
  }
  return false;
}

size_t sum_invalid_ids(struct range r) {
  size_t sum = 0;
  for (size_t n = r.start; n <= r.end; n++) {
    if (is_invalid_id(n)) {
      sum += n;
    }
  }
  return sum;
}

int main() {
  size_t bufsize = 255;
  char *line = malloc(bufsize);
  size_t sum = 0;
  debug("%d: %d\n", 11, is_invalid_id(11));
  debug("%d: %d\n", 12, is_invalid_id(12));
  debug("%d: %d\n", 6262, is_invalid_id(6262));
  debug("%d: %d\n", 420420, is_invalid_id(420420));
  debug("%d: %d\n", 42020, is_invalid_id(42020));
  debug("%d: %d\n", 420200, is_invalid_id(420200));
  while (getdelim(&line, &bufsize, ',', stdin) > 0) {
    struct range r = parse_range(line);
    debug("[%zu,%zu]\n", r.start, r.end);
    sum += sum_invalid_ids(r);
  }
  free(line);
  printf("%zu\n", sum);
  return 0;
}
