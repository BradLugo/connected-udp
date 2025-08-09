default: help

# Get a list of recipes you can run
@help:
  just --list

# Run all the checks required for CI to pass
ci: lint spell-check test

# Format all rust code
fmt:
  cargo fmt --all

# Check the format of all rust code (but don't write the changes)
fmt-check:
  cargo fmt --all --check

lint: fmt-check
  cargo clippy --all-targets --all-features -- -Dwarnings

spell-check:
  typos

test:
  cargo test --all-targets --all-features --verbose

# Upgrades cargo dependencies
upgrade-deps:
  cargo-upgrade upgrade -vv

# Adds a specified cargo crate, then sorts the dependencies in Cargo.toml
cargo-add crate:
  cargo add {{ crate }} && cargo sort
