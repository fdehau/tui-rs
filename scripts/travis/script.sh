#!/bin/bash

set -eu

make build
make test
if [[ "$TRAVIS_RUST_VERSION" == "nightly" ]]; then
  export LD_LIBRARY_PATH=$(rustc +nightly --print sysroot)/lib:$LD_LIBRARY_PATH
  make lint
fi
