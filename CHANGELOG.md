# [0.2.0]

## BREAKING
- Switched from reqwest to ureq crate for HTTP calls. This implies a different variant in the GaiaError enum.

# [0.1.4]

## Changed
- Updated strum crates and switched to Rust Edition 2024.

# [0.1.3]

## Added
- Support for LAMOST DR9 Tables

# [0.1.2]

## Changed
- Switched to default-tls feature for reqwest crate to avoid downstream dependances with copyleft license.

# [0.1.1]

## Added
- GreaterThanOrEqual and LessThanOrEqual conditions

# [0.1.0]

## Added
- Functionality to query all currently available Gaia schemas, tables and columns.
- Selecting the top results.
- Filtering results with greater than, lesser than and between conditions for numbers.
