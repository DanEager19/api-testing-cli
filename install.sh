#!/bin/bash
cargo build
cargo install --path ./
sudo cp ./target/release/atc /usr/bin