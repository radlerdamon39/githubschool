// Copyright 2018 The Rust Project Developers. See the AUTHORS file.
// This file is part of the "Cargo.toml" file.
// It's subject to the license in the .json-file as supplied with this package.

#[cfg(feature = "serde")]
mod serde;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct MyStruct {
    name: String,
    age: i32,
}

fn main() {} // Main function
