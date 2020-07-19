#!/bin/bash -ex

cd "$(dirname "$0")/.."

rm -rf .worktrees
git worktree prune

msg="$(git show -s --format=%B)"

function do_vendor() {
    if [ -f "Cargo.toml" ]; then
        cargo vendor
    fi
}

for platform in $(find . -name .platform); do
    root="$(dirname "$platform")"
    name="$(echo -n "$root" | tr -C 'a-z0-9_' _)"
    name="${name#_}"
    name="${name#_}"
    if [[ -z "$name" ]]; then
        continue
    fi
    echo "$name: $root"
    workdir=".worktrees/$name"
    branch="submissions/$name"
    if ! git worktree add "$workdir" "$branch"; then
        git worktree add "$workdir" submission-base
        pushd "$workdir"
        git checkout -b "$branch"
        popd
    fi
    pushd "$workdir"
    git rm -rf --ignore-unmatch .
    popd
    cp -r "$root/." "$workdir/"
    pushd "$workdir"
    do_vendor
    git add .
    if git commit -m "$msg"; then
        if [[ "$1" == "--push" ]]; then
            git push origin "$branch"
        fi
    fi
    popd
done
