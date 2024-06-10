#!/usr/bin/env bash

# show changes since last release (ignoring nightly) and tag current HEAD with
# nightly
# ----------------------------------------------------------------------------
./scripts/release/nightly/changelog.sh > CHANGELOG.md
git tag -f nightly
git push --tags --force

branch=$(git branch --show-current)
hash=$(git rev-parse HEAD | cut -c 1-7)

# delete last nightly release and create new nightly release from changelog
# create release with:
# - *prerelease*: true
# - *title*: branch-hash
# - *tag*: nightly
# - *branch|target*: nightly
# -------------------------------------------------------------------------
gh release delete nightly -y
gh release create -p nightly --target nightly -t "${branch}-${hash}" -F CHANGELOG.md
