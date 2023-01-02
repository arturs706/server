#!/usr/bin/env bash
cargo +stable fmt
cargo build --all
cargo run --release