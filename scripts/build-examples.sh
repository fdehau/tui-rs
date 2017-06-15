#!/bin/bash

# Build all examples in examples directory

set -eu -o pipefail

for file in examples/*.rs; do
  name=$(basename ${file//.rs/})
  echo "[EXAMPLE] $name"
  if [[ "$name" == "rustbox" ]]; then
    cargo build --features rustbox --example "$name"
  else
    cargo build --example "$name"
  fi
done
