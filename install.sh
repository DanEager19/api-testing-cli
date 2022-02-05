#!/bin/bash
$HOME/cargo/env build
$HOME/cargo/env install --path ./
cp ./target/release/atc /usr/bin