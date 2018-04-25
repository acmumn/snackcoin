#!/bin/bash

# Yup, we're going back to the 80's. Fun! This script has only been tested on
# x86_64 Linux. macOS users and ARMies, you've been warned.

set -eu # Error out if anything goes wrong.

cd $(dirname $BASH_SOURCE)
mkdir -p dist

cd client
yarn
yarn build
tar -cC dist -f ../dist/site.tar .
cd ..

cd server
cargo build --release
install target/release/snackcoin ../dist/snackcoin
cd ..
