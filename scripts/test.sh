#!/bin/sh

set -e

cd "$(dirname "$0")/.."

cargo test --all-features
