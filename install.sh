#!/bin/bash
cargo build
cargo install --path ./
cp ./target/release/atc /usr/bin