#!/usr/bin/env bash

branch=$(git branch --show-current)
hash=$(git rev-parse HEAD | cut -c 1-7)

echo "${branch}-${hash}"
