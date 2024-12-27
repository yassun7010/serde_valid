#!/bin/sh

set -e

cd "$(dirname "$0")/../crates/serde_valid"

cargo readme --no-indent-headings --no-title --no-license >README.md
