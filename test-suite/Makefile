CROSS   ?= m32r-elf-
CC      := $(CROSS)gcc
OBJCOPY := $(CROSS)objcopy

CFLAGS  := -ffreestanding -nostdlib -nostartfiles -fno-builtin -fno-stack-protector \
           -O2 -g -Wall -Wextra
LDFLAGS := -T m32r-test.ld -nostdlib -nostartfiles

SRCS := $(wildcard test_*.c)
BINS := $(patsubst %.c,build/%.bin,$(SRCS))

all: $(BINS)

build:
	mkdir -p build

build/%.elf: %.c crt0.S m32r-test.ld | build
	$(CC) $(CFLAGS) $(LDFLAGS) crt0.S $< -o $@

build/%.bin: build/%.elf
	$(OBJCOPY) -O binary $< $@

clean:
	rm -rf build

.PHONY: all clean
