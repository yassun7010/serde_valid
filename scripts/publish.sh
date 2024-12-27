#!/bin/sh

set -e

cd "$(dirname "$0")"/../xtask

cargo xtask update-tags

cd ../crates/serde_valid_derive
cargo publish

cd ../crates/serde_valid_literal
cargo publish

# wait tarball package publishment
sleep 20

cd ../crates/serde_valid
cargo publish
