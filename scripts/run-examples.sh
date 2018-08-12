#!/bin/bash

# Run all examples in examples directory

set -u -o pipefail

for file in examples/*.rs; do
  name=$(basename ${file//.rs/})
  if [[ "$name" == "rustbox" ]]; then
    cargo run --features rustbox --example "$name"
  else
    cargo run --example "$name"
  fi
done
