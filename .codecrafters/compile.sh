#!/bin/sh

set -e # Exit on failure

cargo build --release --target-dir=/tmp/codecrafters-build-http-server-rust --manifest-path Cargo.toml
