#!/bin/sh

set -e

cd "$(dirname "$0")/.."

cd serde_valid_derive
cargo publish

cd ../serde_valid_literal
cargo publish

# wait tarball package publishment
sleep 20

cd ../serde_valid
cargo publish
