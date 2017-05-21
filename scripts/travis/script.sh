#!/bin/bash

set -eu

if [[ "$TRAVIS_RUST_VERSION" == "nightly" ]]; then
  make lint
fi
