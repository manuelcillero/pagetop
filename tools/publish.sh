#!/bin/bash

cd "$(dirname "$0")"
cd ..

if [ -n "$(git status --porcelain)" ]; then
    echo "You have local changes!"
    exit 1
fi

function publish_crate() {
    echo -e "\nPublishing ${crate}"
    cp ../LICENSE-MIT "$crate"
    cp ../LICENSE-APACHE "$crate"
    pushd "$crate"
    git add LICENSE-MIT LICENSE-APACHE
#   cargo publish --no-verify --allow-dirty
    cargo publish --allow-dirty
    popd
    sleep 20
}

# If package A depends on package B, B must come before A in this list
helpers=(
    pagetop-macros
    pagetop-build
)
packages=(
    pagetop-homedemo
    pagetop-user
    pagetop-admin
    pagetop-node
    pagetop-bootsier
    pagetop-bulmix
)

pushd helpers
for crate in "${helpers[@]}"; do publish_crate; done
popd

echo -e "\nPublishing root crate"
cargo publish --allow-dirty

pushd packages
for crate in "${packages[@]}"; do publish_crate; done
popd

echo "Cleaning local state"
git reset HEAD --hard
