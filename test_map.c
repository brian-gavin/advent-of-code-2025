#include "include/aocmap.h"
#include <stdio.h>
#include <stdlib.h>

void test(const char *msg) {
  puts(msg);
  struct aocmap am;
  am_init(&am, 10);
  for (const char *s = msg; *s; s++) {
    int *count;
    if ((count = (int *)am_get(&am, s, 1))) {
      (*count)++;
      continue;
    }
    count = malloc(sizeof(int));
    *count = 1;
    am_put(&am, s, 1, count);
  }
  am_print(&am);
  am_foreach(am, it) {
    char c = *it.cur_ent->key;
    int *count = (int *)it.cur_ent->val;
    printf("'%c': %d\n", c, *count);
  }
  for (const char *s = msg; *s; s++) {
    int *count = (int *)am_del(&am, s, 1);
    if (!count)
      printf("remove: '%c' | <nil>\n", *s);
    else
      printf("remove: '%c' | %d\n", *s, *count);
    free(count);
  }
  am_print(&am);
  am_fini(&am);
}

void test2(const char *msg) {
  puts(msg);
  struct aocmap am;
  am_init(&am, 10);
  for (const char *s = msg; *s; s++) {
    int *count;
    if ((count = (int *)am_get(&am, s, 1))) {
      (*count)++;
      continue;
    }
    count = malloc(sizeof(int));
    *count = 1;
    am_put(&am, s, 1, count);
  }
  am_print(&am);
  int *count = (int *)am_del(&am, "w", 1);
  free(count);
  am_print(&am);
  puts("");
  count = (int *)am_del(&am, "m", 1);
  free(count);
  am_print(&am);
  am_fini(&am);
}

int main() {
  test("hello world");
  puts("");
  test("abcdefghijklmnopqrstuvwxyz");
  test2("abcdefghijklmnopqrstuvwxyz");
}
