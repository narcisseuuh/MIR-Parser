#!/bin/bash
set -e
export RUSTC_SYSROOT="~/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/"
./test.exe

