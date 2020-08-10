/*
 * SPDX-License-Identifier: BSD-3-Clause
 *
 * Copyright (c) 2020 Fernando Lugo <lugo.fernando@gmail.com>
 */

use std::env;

fn main() {
    let linker_script = env::var("LINKER_SCRIPT").unwrap();

    println!("cargo:rerun-if-changed={}", linker_script);
}
