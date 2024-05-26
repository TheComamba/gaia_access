#!/bin/bash

cargo install cargo-release

if [[ "$1" == "--execute" ]]; then
    cargo release --execute
else
    cargo release
    echo "The script was run without --execute argument. If you want to execute the release, run the script with --execute argument."
fi
