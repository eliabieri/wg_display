#!/bin/sh
set -e

crates=(
    app
    common
    frontend
)
for i in "${crates[@]}"; do
    printf "\nChecking %s\n" "$i"
    cargo update --manifest-path "$i"/Cargo.toml
done
