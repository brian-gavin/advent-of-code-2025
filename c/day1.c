#include "include/log.h"
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>

struct rotation {
  int32_t distance;
  char direction;
};

static struct rotation parse_rotation(char *line) {
  struct rotation r;
  r.direction = *line;
  r.distance = (int32_t)strtol(line + 1, NULL, 10);
  return r;
}

struct solution {
  uint64_t zeroes;
  int32_t dial;
};

static void apply_rotation_1(struct solution *s, struct rotation r) {
  debug("apply: %c%d | init dial: %d ", r.direction, r.distance, s->dial);
  switch (r.direction) {
  case 'R':
    for (int32_t i = 0; i < r.distance % 100; i++) {
      s->dial++;
      if (s->dial > 99) {
        s->dial = 0;
      }
    }
    break;
  case 'L':
    for (int32_t i = 0; i < r.distance % 100; i++) {
      s->dial--;
      if (s->dial < 0) {
        s->dial = 99;
      }
    }
    break;
  }
  if (s->dial == 0)
    s->zeroes++;
  debug("| dial: %d | zeroes: %zu\n", s->dial, s->zeroes);
}

static void apply_rotation_2(struct solution *s, struct rotation r) {
  debug("apply: %c%d | init dial: %d ", r.direction, r.distance, s->dial);
  switch (r.direction) {
  case 'R':
    s->zeroes += (r.distance / 100);
    for (int32_t i = 0; i < r.distance % 100; i++) {
      s->dial++;
      if (s->dial > 99) {
        s->dial = 0;
        s->zeroes++;
      }
    }
    break;
  case 'L':
    s->zeroes += (r.distance / 100);
    for (int32_t i = 0; i < r.distance % 100; i++) {
      s->dial--;
      if (s->dial == 0) {
        s->zeroes++;
      }
      if (s->dial < 0) {
        s->dial = 99;
      }
    }
    break;
  }
  debug("| dial: %d | zeroes: %zu\n", s->dial, s->zeroes);
}

int main() {
  size_t bufsize = 255;
  char *line = malloc(bufsize);
  struct solution sol = {
      .zeroes = 0,
      .dial = 50,
  };
  while (getline(&line, &bufsize, stdin) != -1) {
    apply_rotation_2(&sol, parse_rotation(line));
  }
  free(line);
  printf("%zu\n", sol.zeroes);
  return 0;
}
