#!/bin/sh

set -e

cd "$(dirname "$0")/.."

cargo readme --no-indent-headings --no-title --no-license > README.md
