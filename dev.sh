#!/usr/bin/env bash

prep() {
    rustup component add llvm-tools-preview
    rustup toolchain add nightly
    cargo install bootimage
}

build() {
    cargo bootimage
}

run() {
    # this doesnt work use virt-manager
    qemu-system-x86_64 -drive format=raw,file=target/x86_64-calcOS/debug/bootimage-calcOS.bin
}

case "$1" in
    "prep") prep ;;
    "build") build ;;
    "run") run ;;
esac