#!/bin/bash

set -e -o pipefail

USAGE="$0 --name=STRING [--channel=STRING]"

# Default values
CARGO="cargo"
NAME=""
CHANNEL=""

# Parse args
for i in "$@"; do
  case $i in
    --name=*)
      NAME="${i#*=}"
      shift
      ;;
    --channel=*)
      CHANNEL="${i#*=}"
      shift
      ;;
    *)
      echo $USAGE
      exit 1
      ;;
  esac
done

current_version() {
  local crate="$1"
  $CARGO install --list | \
    grep $crate | \
    head -n 1 | \
    cut -d ' ' -f 2 | \
    sed 's/v\(.*\):/\1/g'
}

upstream_version() {
  local crate="$1"
  $CARGO search $crate | \
    grep $crate | \
    head -n 1 | \
    cut -d' ' -f 3 | \
    sed 's/"//g'
}

if [ "$NAME" == "" ]; then
  echo $USAGE
  exit 1
fi

if [ "$CHANNEL" != "" ]; then
  CARGO+=" +$CHANNEL"
fi

CURRENT_VERSION="$(current_version $NAME)"
UPSTREAM_VERSION="$(upstream_version $NAME)"

if [ "$CURRENT_VERSION" != "$UPSTREAM_VERSION" ]; then
  echo "WARN: Latest version of $NAME not installed: $CURRENT_VERSION -> $UPSTREAM_VERSION"
  $CARGO install --force $NAME
else
  echo "INFO: Latest version of $NAME already installed ($CURRENT_VERSION)"
fi
