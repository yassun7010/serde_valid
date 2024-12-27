#!/bin/sh

set -e

cd "$(dirname "$0")"/../

cargo xtask update-tags

cd "$(dirname "$0")"/../crates/serde_valid_derive
cargo publish

cd "$(dirname "$0")"/../crates/serde_valid_literal
cargo publish

# wait tarball package publishment
sleep 20

cd "$(dirname "$0")"/../crates/serde_valid
cargo publish
