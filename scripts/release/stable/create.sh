#!/usr/bin/env bash

version="$(git cliff --bumped-version)"
n_version="$(git cliff --bumped-version | sed -e 's/^v//')"

# show changes since last release (ignoring nightly) and tag current HEAD with
# bumped version
# ----------------------------------------------------------------------------
./scripts/release/stable/changelog.sh > CHANGELOG.md
git tag -f "$version"
git push --tags --force

# delete last nightly release and create new nightly release from changelog
# create release with:
# - *title*: number of version
# - *tag*: version
# - *branch|target*: version
# -------------------------------------------------------------------------
[[ $FORCE -gt 0 ]] && gh release delete "$n_version" -y
gh release create "$n_version" -t "$version" -F CHANGELOG.md
