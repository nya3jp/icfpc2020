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

function generate_sh() {
    workdir="$1"

    # Generate run.sh
    cat <<EOF > "$workdir/run.sh"
#!/bin/bash

pushd "$root"
./run.sh
popd
EOF
    chmod a+x "$workdir/run.sh"

    # Generate build.sh
    cat <<EOF > "$workdir/build.sh"
#!/bin/bash

pushd "$root"
./build.sh
popd
EOF
    chmod a+x "$workdir/build.sh"
    pushd "$workdir"
    git add .
    popd
}

function generate_cargo_config() {
    mkdir ".cargo"
    cat <<EOF > ".cargo/config"
[source.crates-io]
replace-with = "vendored-sources"

[source.vendored-sources]
directory = "vendor"
EOF
    git add .cargo
}

function copy_dependencies() {
    workdir="$1"
    # Copy interact.py
    mkdir -p "$workdir/infra/interact"
    cp -r "infra/interact/." "$workdir/infra/interact/"
    pushd "$workdir/infra/interact/"
    git add .
    popd

    # Copy rust_game_base
    mkdir -p "$workdir/infra/rust_game_base"
    cp -r "infra/rust_game_base/." "$workdir/infra/rust_game_base/"
    pushd "$workdir/infra/rust_game_base"
    do_vendor
    git add .
    popd
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

    generate_sh "$workdir"
    copy_dependencies "$workdir"

    # Copy bot code
    mkdir -p "$workdir/$root"
    cp -r "$root/." "$workdir/$root/"

    # Copy specified .platform
    pwd
    ls -al "$workdir/$root/.platform"
    cp "$workdir/$root/.platform" "$workdir/"
    pushd "$workdir"
    git add .
    popd

    pushd "$workdir/$root"
    do_vendor
    generate_cargo_config
    git add .

    if git commit -m "$msg"; then
        if [[ "$1" == "--push" ]]; then
            git push origin "$branch"
        fi
    fi
    popd
done
