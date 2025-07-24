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

    # Update npm package.json
    let npm_package = "packages/npm/package.json"
    if ($npm_package | path exists) {
        print $"Updating ($npm_package)"
        let package_content = (open $npm_package --raw | str replace $'"version": "($current_version)"' $'"version": "($new_version)"')
        $package_content | save -f $npm_package
    }

    # Update scoop manifest
    let scoop_manifest = "packages/scoop/sunny-cli.json"
    if ($scoop_manifest | path exists) {
        print $"Updating ($scoop_manifest)"
        let scoop_content = (open $scoop_manifest --raw |
            str replace $'"version": "($current_version)"' $'"version": "($new_version)"' |
            str replace --all $'/download/v($current_version)/' $'/download/v($new_version)/'
        )
        $scoop_content | save -f $scoop_manifest
    }

    cargo check --quiet

    if not $dry_run {
        git add Cargo.toml Cargo.lock $npm_package $scoop_manifest
        git commit -m $"chore: release v($new_version)"
        git tag $"v($new_version)"
        git push origin HEAD
        git push origin $"v($new_version)"

        print $"Released v($new_version)"
    } else {
        print $"[DRY RUN] Files updated, but not committed. Run 'git status' to see changes."
    }
}
