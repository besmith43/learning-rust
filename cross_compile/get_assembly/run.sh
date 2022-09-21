#!/bin/bash

# for debug
cargo rustc -- --emit asm

cat ./target/debug/deps/get_assembly-*.s

# for release
# cargo rustc --release -- --emit asm

# cat ./target/release/deps/get_assembly-*.s

# get llvm ir
cargo rustc -- --emit llvm-ir

cat ./target/debug/deps/get_assembly-*.ll

