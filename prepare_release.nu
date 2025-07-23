#!/usr/bin/env nu

def main [--dry-run] {
    let current_version = (open Cargo.toml | get package.version)

    print $"Current version: ($current_version)"
    let new_version = (input "Enter new version: ")

    if not ($new_version =~ '^[0-9]+\.[0-9]+\.[0-9]+$') {
        print "Error: Version must follow semantic versioning"
        exit 1
    }

    let existing_tags = (git tag -l | lines)
    if ($existing_tags | any {|tag| $tag == $"v($new_version)"}) {
        print $"Error: Tag v($new_version) already exists"
        exit 1
    }

    print $"Updating version from ($current_version) to ($new_version)"
    
    let cargo_content = (open Cargo.toml --raw | str replace $'version = "($current_version)"' $'version = "($new_version)"')
    $cargo_content | save -f Cargo.toml
    print "Updated Cargo.toml"

    # Update all package.json files
    let package_files = (glob "**/package.json" | sort)
    for file in $package_files {
        print $"Updating ($file)"
        let package_content = (open $file --raw |
            str replace $'"version": "($current_version)"' $'"version": "($new_version)"' |
            str replace $'"@jamesukiyo/sunny-cli-linux-x64": "($current_version)"' $'"@jamesukiyo/sunny-cli-linux-x64": "($new_version)"' |
            str replace $'"@jamesukiyo/sunny-cli-linux-arm64": "($current_version)"' $'"@jamesukiyo/sunny-cli-linux-arm64": "($new_version)"' |
            str replace $'"@jamesukiyo/sunny-cli-darwin-x64": "($current_version)"' $'"@jamesukiyo/sunny-cli-darwin-x64": "($new_version)"' |
            str replace $'"@jamesukiyo/sunny-cli-darwin-arm64": "($current_version)"' $'"@jamesukiyo/sunny-cli-darwin-arm64": "($new_version)"' |
            str replace $'"@jamesukiyo/sunny-cli-windows-x64": "($current_version)"' $'"@jamesukiyo/sunny-cli-windows-x64": "($new_version)"' |
            str replace $'"@jamesukiyo/sunny-cli-windows-arm64": "($current_version)"' $'"@jamesukiyo/sunny-cli-windows-arm64": "($new_version)"'
        )
        $package_content | save -f $file
    }

    cargo check --quiet

    if not $dry_run {
        git add Cargo.toml Cargo.lock $package_files
        git commit -m $"chore: release v($new_version)"
        git tag $"v($new_version)"
        git push origin HEAD
        git push origin $"v($new_version)"

        print $"Released v($new_version)"
    } else {
        print $"[DRY RUN] Files updated, but not committed. Run 'git status' to see changes."
    }
}
