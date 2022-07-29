#!/bin/sh

set -e

cd "$(dirname "$0")/.."

cd derive
cargo publish

cd ../literal
cargo publish

cd ../
cargo publish
