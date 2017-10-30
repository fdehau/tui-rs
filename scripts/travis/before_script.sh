#!/bin/bash

set -eu

export PATH="$PATH:$HOME/.cargo/bin"
if [[ "$TRAVIS_RUST_VERSION" == "nightly" ]]; then
  export LD_LIBRARY_PATH=$(rustc +nightly --print sysroot)/lib:$LD_LIBRARY_PATH
  make install-tools
fi
