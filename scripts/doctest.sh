#!/bin/sh

set -e

cd "$(dirname "$0")/.."

cargo build

cd docs

mdbook test --library-path ../target/debug/deps
