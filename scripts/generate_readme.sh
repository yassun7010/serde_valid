#!/bin/sh

set -e

cd "$(dirname "$0")/../serde_valid"

cargo readme --no-indent-headings --no-title --no-license > README.md
