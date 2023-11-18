# List commands
default:
    @just --list

# Debug build
dbuild:
    cargo build

# Release build
rbuild:
    cargo build --release

# Create docs
doc:
    cargo doc --no-deps --open

# Print library size
size: dbuild rbuild
    @ls -sh ./target/debug/librobot_hat_rs.rlib
    @ls -sh ./target/release/librobot_hat_rs.rlib

# Clean target
clean:
    cargo clean

# Git
git:
    git status
    git diff
