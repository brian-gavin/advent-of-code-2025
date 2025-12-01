SESSION ?=

ifndef SESSION
$(error "SESSION must be set")
endif

# INPUT_FILES := $(patsubst src/bin/day%.rs, src/bin/%.txt, $(wildcard src/bin/*.rs))

# input: $(INPUT_FILES)

%.txt:
	curl https://adventofcode.com/2025/day/$(basename $@)/input --cookie "session=$(SESSION)" > $@

CFLAGS ?=
CFLAGS += -O2 -Wall -Wextra -I include
CC = clang

.SECONDARY:
%.o: %.c
	$(CC) -c -o $@ $(CFLAGS) $^

%.out: %.o aocmap.o
	$(CC) -o $@ $(CFLAGS) $^

.PHONY: clean fmt

fmt:
	clang-format -style llvm *.c -i

clean:
	rm *.o *.out
