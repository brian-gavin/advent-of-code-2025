#include "include/arraylist.h"
#include <stdio.h>

DEFINE_ARRAYLIST(str, char *)

int main() {
  struct arraylist_str al;
  al_str_init(&al);
  al_str_append(&al, "a");
  al_str_append(&al, "b");
  al_str_append(&al, "b");
  al_str_append(&al, "b");
  al_str_append(&al, "b");
  al_str_append(&al, "b");
  al_str_append(&al, "b");
  al_str_append(&al, "b");
  al_str_append(&al, "b");
  al_str_append(&al, "b");
  al_str_append(&al, "b");
  al_str_append(&al, "c");
  puts(al_str_at(&al, 0));
  puts(al_str_at(&al, 1));
  puts(al_str_at(&al, al_str_len(&al) - 1));
  return 0;
}
