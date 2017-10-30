#!/bin/bash

set -eu -o pipefail

crate_name="rustfmt-nightly"
has_rustfmt=$(cargo +nightly install --list | { grep $crate_name || true; })

if [ -z "$has_rustfmt" ]; then
  echo "WARN: $crate_name not found."
  echo "INFO: Installing latest version from crates.io."
  cargo +nightly install $crate_name
else
  current_version=$(rustfmt --version | cut -d '-' -f 1)
  upstream_version=$(cargo +nightly search $crate_name| head -n 1 | cut -d ' ' -f 3 | tr -d '"')
  if [ "$current_version" != "$upstream_version" ]; then
    echo "WARN: New version of $crate_name available: $upstream_version (current=$current_version)"
    echo "INFO: Installing latest version from crates.io."
    cargo +nightly install $crate_name --force
  else
    echo "INFO: $crate_name is up to date"
  fi
fi
