#!/bin/bash -e

cargo test --features "toml yaml"
cargo test --features "toml yaml serde_error"
