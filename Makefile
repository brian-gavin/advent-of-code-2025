SESSION ?=

ifndef SESSION
$(error "SESSION must be set")
endif

# INPUT_FILES := $(patsubst src/bin/day%.rs, src/bin/%.txt, $(wildcard src/bin/*.rs))
SOLUTIONS_C := day1

# input: $(INPUT_FILES)

%.txt:
	curl https://adventofcode.com/2025/day/$(basename $@)/input --cookie "session=$(SESSION)" > $@

CFLAGS = -O2 -Wall -Wextra -I include
DFLAGS = -D DEBUG -g -Wall -Wextra -I include
CC = clang

.SECONDARY:
%.o: %.c
	$(CC) -c -o $@ $(CFLAGS) $^

%.debug.o: %.c
	$(CC) -c -o $@ $(DFLAGS) $^

%.out: %.o aocmap.o
	$(CC) -o $@ $(CFLAGS) $^

%.debug.out: %.debug.o aocmap.debug.o
	$(CC) -o $@ $(DFLAGS) $^

all: $(addsuffix .out, $(SOLUTIONS_C))

debug: $(addsuffix .debug.out, $(SOLUTIONS_C))

.PHONY: clean fmt

fmt:
	clang-format -style llvm *.c -i

clean:
	rm *.o *.out
