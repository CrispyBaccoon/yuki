#!/usr/bin/env bash

SAKUPATH="${SAKUPATH:-$HOME/.saku}"

set -e

printf " -\033[35m setting up\033[0m saku root\n"
mkdir -p $SAKUPATH/repo
cd $SAKUPATH/repo

printf " -\033[35m cloning\033[0m saku from\033[33m https://github.com/comfysage/saku\033[0m\n"
git clone --filter=blob:none https://github.com/comfysage/saku && cd saku

printf " -\033[35m building\033[0m saku\n"
cargo build -r

printf " -\033[35m setting up\033[0m environment\n"
./target/release/sk config init

printf " -\033[35m finishing\033[0m installation\n"
./target/release/sk task install saku

