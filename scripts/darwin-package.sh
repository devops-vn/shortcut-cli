#!/bin/bash

cd darwin-build

if [[ ! -e macos-installer-builder ]]; then
    git clone https://github.com/KosalaHerath/macos-installer-builder.git
fi

mkdir -p shortcut-cli
cp -r macos-installer-builder/macOS-x64/* shortcut-cli

cp resources/*.html shortcut-cli/darwin/Resources/
cp resources/postinstall shortcut-cli/darwin/scripts/

mkdir -p shortcut-cli/application/bin/
cp ../target/release/shortcut-cli shortcut-cli/application/bin/

cd shortcut-cli

./build-macos-x64.sh shortcut-cli 1.0.0
