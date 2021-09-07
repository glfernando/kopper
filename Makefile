# SPDX-License-Identifier: BSD-3-Clause
#
# Copyright (c) 2020 Fernando Lugo <lugo.fernando@gmail.com>

BOARD ?= qemu-aarch64
RELEASE ?= 0

ifeq ($(BOARD), qemu-aarch64)
BOARD_PATH = qemu/aarch64
else ifeq ($(BOARD), pico)
BOARD_PATH = raspberrypi/pico
else
$(error invalid board)
endif

# include board makefile
-include src/board/$(BOARD_PATH)/Makefile

LINKER_SCRIPT ?= src/board/$(BOARD_PATH)/linker.ld
RELOCATION ?= 0

# Export for build.rs
export LINKER_SCRIPT

RUSTFLAGS = -C link-arg=-T$(LINKER_SCRIPT) -C linker="rust-lld" $(BOARD_RUSTFLAGS)
ifeq ($(RELOCATION), 1)
RUSTFLAGS += -C relocation-model=pic -C link-arg=-pie
endif
RUSTFLAGS_PEDANTIC = $(RUSTFLAGS) -D warnings -D missing_docs

COMPILER_ARGS = --target=$(TARGET)
FEATURES = --features board_$(BOARD)

ifeq ($(RELEASE), 1)
COMPILER_ARGS += --release
TARGET_BUILD = release
else
TARGET_BUILD = debug
endif

KOPPER_ELF = target/$(TARGET)/$(TARGET_BUILD)/kopper

OBJCOPY = rust-objcopy

OBJCPYFLAGS = -O binary
RUSTC = cargo build

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
	RUSTFLAGS="$(RUSTFLAGS_PEDANTIC)" $(RUSTC) $(COMPILER_ARGS) $(FEATURES) -Z build-std=core,alloc

clean:
	cargo clean $(COMPILER_ARGS)
	rm -f kopper.bin

# allow board code to provide extra rules
-include src/board/$(BOARD_PATH)/rules.mk
