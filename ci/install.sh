#!/bin/bash
set -e

main() {
    if [ $TARGET != x86_64-unknown-linux-gnu ]; then
        rustup target add $TARGET
        rustup component add rustfmt --toolchain nightly-x86_64-unknown-linux-gnu
        rustup component add clippy --toolchain nightly-x86_64-unknown-linux-gnu
        cargo install cargo-update || echo "cargo-update already installed"
        cargo install cargo-travis || echo "cargo-travis already installed"
        cargo install-update -a
    fi
}

main
