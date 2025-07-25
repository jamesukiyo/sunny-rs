#!/bin/bash
set -euo pipefail

ARTIFACTS_DIR="${1:-artifacts}"
OUTPUT_DIR="${2:-release-assets}"

echo "Creating additional GitHub release archives from $ARTIFACTS_DIR to $OUTPUT_DIR"
mkdir -p "$OUTPUT_DIR"

# create any missing archives (package update job creates windows/macos/linux archives)
# this script creates any that might be missing or additional formats needed
for artifact_dir in "$ARTIFACTS_DIR"/binary-*; do
    if [[ -d "$artifact_dir" ]]; then
        platform=$(basename "$artifact_dir" | sed 's/binary-//')
        echo "Processing platform: $platform"

        # only create if doesn't already exist
        if [[ "$platform" == *"windows"* ]] && [[ ! -f "$OUTPUT_DIR/sunny-cli-$platform.zip" ]]; then
            echo "Creating Windows release archive for $platform"
            (cd "$artifact_dir" && zip -r "../../$OUTPUT_DIR/sunny-cli-$platform.zip" *)
        elif ([[ "$platform" == *"darwin"* ]] || [[ "$platform" == *"linux"* ]]) && [[ ! -f "$OUTPUT_DIR/sunny-cli-$platform.tar.gz" ]]; then
            echo "Creating release archive for $platform"
            (cd "$artifact_dir" && tar -czf "../../$OUTPUT_DIR/sunny-cli-$platform.tar.gz" *)
        else
            echo "Archive for $platform already exists, skipping"
        fi
    fi
done

echo "GitHub release archives completed"
ls -la "$OUTPUT_DIR"