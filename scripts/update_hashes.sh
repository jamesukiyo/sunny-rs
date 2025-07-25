#!/bin/bash
set -euo pipefail

RELEASE_ASSETS_DIR="${1:-release-assets}"

echo "Updating package manifest hashes from $RELEASE_ASSETS_DIR"

# scoop hashes
if [[ -f "$RELEASE_ASSETS_DIR/sunny-cli-windows-x64.zip" && -f "$RELEASE_ASSETS_DIR/sunny-cli-windows-arm64.zip" ]]; then
    echo "Updating Scoop manifest hashes..."
    
    HASH_WIN_X64=$(sha256sum "$RELEASE_ASSETS_DIR/sunny-cli-windows-x64.zip" | cut -d' ' -f1)
    HASH_WIN_ARM64=$(sha256sum "$RELEASE_ASSETS_DIR/sunny-cli-windows-arm64.zip" | cut -d' ' -f1)

    sed -i "0,/\"hash\": \"\"/{s/\"hash\": \"\"/\"hash\": \"$HASH_WIN_X64\"/}" packages/scoop/sunny-cli.json
    sed -i "0,/\"hash\": \"\"/{s/\"hash\": \"\"/\"hash\": \"$HASH_WIN_ARM64\"/}" packages/scoop/sunny-cli.json

    echo "Updated scoop manifest with hashes:"
    grep "hash" packages/scoop/sunny-cli.json
fi

# homebrew hashes
if [[ -f "$RELEASE_ASSETS_DIR/sunny-cli-darwin-x64.tar.gz" && -f "$RELEASE_ASSETS_DIR/sunny-cli-darwin-arm64.tar.gz" && -f "$RELEASE_ASSETS_DIR/sunny-cli-linux-x64.tar.gz" && -f "$RELEASE_ASSETS_DIR/sunny-cli-linux-arm64.tar.gz" ]]; then
    echo "Updating Homebrew formula hashes..."
    
    HASH_MAC_X64=$(sha256sum "$RELEASE_ASSETS_DIR/sunny-cli-darwin-x64.tar.gz" | cut -d' ' -f1)
    HASH_MAC_ARM64=$(sha256sum "$RELEASE_ASSETS_DIR/sunny-cli-darwin-arm64.tar.gz" | cut -d' ' -f1)
    HASH_LINUX_X64=$(sha256sum "$RELEASE_ASSETS_DIR/sunny-cli-linux-x64.tar.gz" | cut -d' ' -f1)
    HASH_LINUX_ARM64=$(sha256sum "$RELEASE_ASSETS_DIR/sunny-cli-linux-arm64.tar.gz" | cut -d' ' -f1)

    sed -i "0,/sha256 \"\"/{s/sha256 \"\"/sha256 \"$HASH_MAC_X64\"/}" HomebrewFormula/sunny-cli.rb
    sed -i "0,/sha256 \"\"/{s/sha256 \"\"/sha256 \"$HASH_MAC_ARM64\"/}" HomebrewFormula/sunny-cli.rb
    sed -i "0,/sha256 \"\"/{s/sha256 \"\"/sha256 \"$HASH_LINUX_X64\"/}" HomebrewFormula/sunny-cli.rb
    sed -i "0,/sha256 \"\"/{s/sha256 \"\"/sha256 \"$HASH_LINUX_ARM64\"/}" HomebrewFormula/sunny-cli.rb

    echo "Updated homebrew formula with hashes:"
    grep -A1 "sha256" HomebrewFormula/sunny-cli.rb
fi

echo "Package manifest hash updates completed"