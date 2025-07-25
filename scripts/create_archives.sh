#!/bin/bash
set -euo pipefail

ARTIFACTS_DIR="${1:-artifacts}"
OUTPUT_DIR="${2:-release-assets}"

echo "Creating release archives from $ARTIFACTS_DIR to $OUTPUT_DIR"
mkdir -p "$OUTPUT_DIR"

# windows zips
for artifact_dir in "$ARTIFACTS_DIR"/binary-windows-*; do
    if [[ -d "$artifact_dir" ]]; then
        platform=$(basename "$artifact_dir" | sed 's/binary-//')
        echo "Creating Windows release archive for $platform"
        (cd "$artifact_dir" && zip -r "../../$OUTPUT_DIR/sunny-cli-$platform.zip" *)
    fi
done

# macos archives
for artifact_dir in "$ARTIFACTS_DIR"/binary-darwin-*; do
    if [[ -d "$artifact_dir" ]]; then
        platform=$(basename "$artifact_dir" | sed 's/binary-//')
        echo "Creating macOS release archive for $platform"
        (cd "$artifact_dir" && tar -czf "../../$OUTPUT_DIR/sunny-cli-$platform.tar.gz" *)
    fi
done

# linux archives
for artifact_dir in "$ARTIFACTS_DIR"/binary-linux-*; do
    if [[ -d "$artifact_dir" ]]; then
        platform=$(basename "$artifact_dir" | sed 's/binary-//')
        echo "Creating Linux release archive for $platform"
        (cd "$artifact_dir" && tar -czf "../../$OUTPUT_DIR/sunny-cli-$platform.tar.gz" *)
    fi
done

echo "Release archives created successfully"
ls -la "$OUTPUT_DIR"