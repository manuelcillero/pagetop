#!/bin/bash

# Navigate to the root workspace directory
cd "$(dirname "$0")"
cd ..

# Check if there are unstaged changes in the Git repository
if [ -n "$(git status --porcelain)" ]; then
    echo "You have local changes!"
    exit 1
fi

# Updates the 'latest' branch with changes from 'main'
read -p "Do you want to update the 'latest' branch? (y/n) " -n 1 -r
echo
if [[ $REPLY =~ ^[Yy]$ ]]
then
    echo "UPDATING 'latest' branch"
    git checkout latest
    git merge main
    echo "PUSHING updated 'latest' branch to remote repository"
    git push origin latest
    git checkout main
else
    echo "Omitting update of 'latest' branch"
fi


# Define a function to publish a crate to crates.io
function publish_crate() {
    echo -e "\nPUBLISHING $CRATE"
    # Get the last published version from crates.io
    PUBLISHED_VERSION=$(cargo search "$CRATE " | grep "^$CRATE = " | sed -E 's/^.*"([^"]+)".*$/\1/')
    # Read the current version from Cargo.toml
    CURRENT_VERSION=$(grep '^version = ' Cargo.toml | head -n 1 | sed -E 's/^version = "([^"]+)".*$/\1/')
    # Compare the versions
    if [ "$PUBLISHED_VERSION" = "$CURRENT_VERSION" ]; then
        echo "Skipping version $CURRENT_VERSION as it already exists on crates.io"
    else
        echo "Publishing version $CURRENT_VERSION..."
        if [ "$CRATE" = "pagetop" ]; then
            cargo publish
        else
            cp ../../LICENSE-MIT .
            cp ../../LICENSE-APACHE .
            git add LICENSE-MIT LICENSE-APACHE
            cargo publish --allow-dirty
        fi
        sleep 20
    fi
}

# If package A depends on package B, B must come before A in this list
HELPERS=(
    pagetop-macros
    pagetop-build
)

# Publish all helper crates
pushd helpers > /dev/null 2>&1
for CRATE in "${HELPERS[@]}"; do
    pushd "$CRATE" > /dev/null 2>&1
    publish_crate
    popd > /dev/null 2>&1
done
popd > /dev/null 2>&1

# Publish the root crate
CRATE=pagetop; publish_crate

# Reset local Git repository to clean licenses after publishing
echo -e "\nCleaning local state"
git reset HEAD --hard
