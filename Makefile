# SPDX-License-Identifier: BSD-3-Clause
#
# Copyright (c) 2020 Fernando Lugo <lugo.fernando@gmail.com>

BOARD ?= qemu-aarch64
RELEASE ?= 0

ifeq  ($(BOARD), qemu-aarch64)
BOARD_PATH = qemu/aarch64
else
$(error invalid board)
endif

# include board makefile
-include src/board/$(BOARD_PATH)/Makefile

LINKER_SCRIPT ?= src/board/$(BOARD_PATH)/linker.ld

# Export for build.rs
export LINKER_SCRIPT

RUSTFLAGS = -C link-arg=-T$(LINKER_SCRIPT) -C linker="rust-lld" -C relocation-model=pic \
	    $(BOARD_RUSTFLAGS)
RUSTFLAGS_PEDANTIC = $(RUSTFLAGS) -D warnings -D missing_docs

COMPILER_ARGS = --target=$(TARGET)

ifeq ($(RELEASE), 1)
COMPILER_ARGS += --release
TARGET_BUILD = release
else
TARGET_BUILD = debug
endif

KOPPER_ELF = target/$(TARGET)/$(TARGET_BUILD)/kopper

OBJCOPY = rust-objcopy

OBJCPYFLAGS = -S -O binary
RUSTC = cargo rustc

# Quiet mode
ifeq ($(V), 1)
Q =
COMPILER_ARGS += --verbose
else
Q = @
endif

.PHONY: $(KOPPER_ELF)

all: kopper.bin


kopper.bin: $(KOPPER_ELF)
	$(Q)$(OBJCOPY) $(OBJCPYFLAGS) $< $@

$(KOPPER_ELF):
	RUSTFLAGS="$(RUSTFLAGS_PEDANTIC)" $(RUSTC) $(COMPILER_ARGS) -Z build-std=core

clean:
	cargo clean $(COMPILER_ARGS)
	rm -f kopper.bin
