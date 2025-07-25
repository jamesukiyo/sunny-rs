#!/bin/bash
set -euo pipefail

PACKAGE_DIR="${1:-npm-package}"

echo "Publishing NPM package from $PACKAGE_DIR"

publish_with_retry() {
    local package_dir=$1
    local max_attempts=5
    local attempt=1
    local base_delay=10

    while [ $attempt -le $max_attempts ]; do
        echo "Attempt $attempt/$max_attempts: Publishing NPM package"
        cd "$package_dir"

        if npm publish --access public; then
            echo "Successfully published NPM package"
            cd ..
            return 0
        else
            echo "Failed to publish NPM package (attempt $attempt)"
            cd ..

            if [ $attempt -lt $max_attempts ]; then
                local delay=$((base_delay * attempt))
                echo "Waiting ${delay}s before retry..."
                sleep $delay
            fi
        fi

        attempt=$((attempt + 1))
    done

    echo "Failed to publish NPM package after $max_attempts attempts"
    return 1
}

publish_with_retry "$PACKAGE_DIR"