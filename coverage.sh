#!/usr/bin/env bash

rm target/debug/deps -r
cargo build
cargo test
grcov ./target/debug/ -s . -t html --llvm --branch --ignore-not-existing -o ./target/debug/coverage/
xdg-open target/debug/coverage/index.html

