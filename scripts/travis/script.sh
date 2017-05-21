#!/bin/bash

set -eu

make build
make test
if [[ "$TRAVIS_RUST_VERSION" == "nightly" ]]; then
  make lint
fi
