#!/bin/sh
cd ./api-testing-cli
cargo build
cargo install --path ./
echo "alias atc='~/.cargo/bin/api-testing-cli'" >> ~/.bashrc