#!/bin/bash

cd $(git rev-parse --show-toplevel)

# Check if cargo deny is installed
code=$(cargo deny --help > /dev/null 2>&1; echo $?)
if [ $code -ne 0 ]; then
    echo "cargo-deny is not installed. Installing..."
    cargo install --locked cargo-deny
fi

# Check if deny.toml exists
if [ ! -f "deny.toml" ]; then
    echo "deny.toml not found. Creating..."
    cargo deny init
fi

# Check licenses
cargo deny check licenses
