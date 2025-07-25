#!/bin/bash
set -euo pipefail

ARTIFACTS_DIR="${1:-artifacts}"
OUTPUT_DIR="${2:-npm-package}"

echo "Creating NPM package from $ARTIFACTS_DIR to $OUTPUT_DIR"

VERSION=$(grep '"version"' packages/npm/package.json | cut -d'"' -f4)
echo "Package version: $VERSION"

mkdir -p "$OUTPUT_DIR"
cp -r packages/npm/* "$OUTPUT_DIR/"
cp README.md "$OUTPUT_DIR/"

# add binaries
for artifact_dir in "$ARTIFACTS_DIR"/binary-*; do
    if [[ -d "$artifact_dir" ]]; then
        platform=$(basename "$artifact_dir" | sed 's/binary-//' | tr '-' '_')
        target_dir="$OUTPUT_DIR/sunny-cli-$VERSION-$platform"
        echo "Adding binaries for platform: $platform"
        mkdir -p "$target_dir"
        cp "$artifact_dir"/sunny* "$target_dir/"
        chmod +x "$target_dir"/sunny* 2>/dev/null || true
    fi
done

echo "NPM package created successfully"
ls -la "$OUTPUT_DIR"