//
// SPDX-License-Identifier: BSD-3-Clause
//
// Copyright (c) 2020 Fernando Lugo <lugo.fernando@gmail.com>
//

#[cfg(target_arch = "aarch64")]
#[path = "_arch/aarch64/cpu.rs"]
mod arch_cpu;

// TODO: make sure it is only for arm-m
#[cfg(target_arch = "arm")]
#[path = "_arch/armv6m/cpu.rs"]
mod arch_cpu;

pub use arch_cpu::*;
