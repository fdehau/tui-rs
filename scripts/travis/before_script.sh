#!/bin/bash

set -eu

export PATH="$PATH:$HOME/.cargo/bin"
if [[ "$TRAVIS_RUST_VERSION" == "nightly" ]]; then
  make install
fi
