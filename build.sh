#!/usr/bin/env bash

SQLITE_WASM_VERSION="v0.18.1"

set -o errexit -o nounset -o pipefail

# set current working directory to script directory to run script from everywhere
cd "$(dirname "$0")"

# build test.wasm
marine build --release --bin test

# copy .wasm to artifacts
rm -f artifacts/*
mkdir -p artifacts
cp target/wasm32-wasi/release/test.wasm artifacts/

# download SQLite 3 to use in tests
if [[ ! -f artifacts/sqlite3.wasm ]]; then
  curl -L https://github.com/fluencelabs/sqlite/releases/download/sqlite-wasm-${SQLITE_WASM_VERSION}/sqlite3.wasm -o artifacts/sqlite3.wasm
fi
