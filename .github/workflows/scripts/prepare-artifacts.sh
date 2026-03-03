#!/bin/bash
set -e

TARGET=$1
OS=$2
ARTIFACT_PREFIX=$3

if [[ -z "$TARGET" || -z "$OS" || -z "$ARTIFACT_PREFIX" ]]; then
  echo "Usage: $0 <target> <os> <artifact_prefix>"
  exit 1
fi

mkdir -p dist
cd target/"$TARGET"/release

# Binary extensions
EXT=""
if [[ "$OS" == "windows-latest" ]]; then
  EXT=".exe"
fi

# Rename and move binaries to dist folder
cp vigild"$EXT" ../../../dist/vigild-"$ARTIFACT_PREFIX""$EXT"
cp vigil"$EXT" ../../../dist/vigil-"$ARTIFACT_PREFIX""$EXT"
cp vigil-logger"$EXT" ../../../dist/vigil-logger-"$ARTIFACT_PREFIX""$EXT"
cp vigil-installer"$EXT" ../../../dist/vigil-installer-"$ARTIFACT_PREFIX""$EXT"
