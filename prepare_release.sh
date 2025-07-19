#!/bin/bash
set -e

current_version=$(grep "^version = " Cargo.toml | head -1 | sed 's/version = "\(.*\)"/\1/')

echo "Current version: $current_version"
read -p "Enter new version: " new_version

if [[ ! $new_version =~ ^[0-9]+\.[0-9]+\.[0-9]+$ ]]; then
    echo "Error: Version must follow semantic versioning"
    exit 1
fi

if git tag -l | grep -q "^v$new_version$"; then
    echo "Error: Tag v$new_version already exists"
    exit 1
fi

echo "Updating version from $current_version to $new_version"

sed -i "s/^version = \".*\"/version = \"$new_version\"/" Cargo.toml
cargo check --quiet

git add Cargo.toml Cargo.lock
git commit -m "chore: release v$new_version"
git tag "v$new_version"
git push origin HEAD
git push origin "v$new_version"

echo "Released v$new_version"
