#!/bin/bash

set -e

cargo test --features gaiadr3_gaia_source -- --include-ignored
