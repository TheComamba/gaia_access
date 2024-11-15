#!/bin/bash

set -e

if [ ! -f ~/.cargo/credentials.toml ]
then
    echo "You are not logged in to cargo."
    echo "Please visit "
    echo "https://crates.io/me"
    echo "to generate a token with the publish-update capability, and then run "
    echo "cargo login <your-api-token>"
    echo "to log in."
    exit 1
fi

cargo install cargo-release

if [[ "$1" == "--execute" ]]; then
    cargo release --execute
    echo "The GitHub release needs to be executed manually."
else
    cargo release
    echo "The script was run without --execute argument. If you want to execute the release, run the script with --execute argument."
fi
