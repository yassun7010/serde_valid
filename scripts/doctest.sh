#!/bin/sh

set -e

cd "$(dirname "$0")/../docs"

mdbook test --library-path ../target/debug/deps
