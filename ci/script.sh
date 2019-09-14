#!/bin/bash

# This script takes care of testing your crate

set -ex

# TODO This is the "test phase", tweak it as you see fit
main() {

    cargo check --target $TARGET
    cargo clippy --target $TARGET --all-features

    # cargo build --target $TARGET
    cargo build --verbose --target $TARGET --all && cargo coverage --verbose && bash <(curl -s https://codecov.io/bash) -s target/kcov
    cargo coverage --verbose

    if [ ! -z $DISABLE_TESTS ]; then
        return
    fi

    # cross test --target $TARGET
    cargo test --target $TARGET --release --verbose --all

    cargo doc --verbose --no-deps --document-private-items
    cargo doc-upload
}

# we don't run the "test phase" when doing deploys
if [ -z $TRAVIS_TAG ]; then
    main
fi