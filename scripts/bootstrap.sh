#!/bin/bash

yum_cmd=$(which yum)
apt_get_cmd=$(which apt-get)
brew_cmd=$(which brew)

base_packages="gcc"
deb_packages="make libssl-dev pkg-config"
yum_packages="make openssl-devel pkgconfig"

if [[ ! -z $yum_cmd ]]; then
    sudo yum -y install $base_packages $yum_packages
elif [[ ! -z $apt_get_cmd ]]; then
    sudo apt-get install $base_packages $deb_packages -y
elif [[ ! -z $brew_cmd ]]; then
    brew install $base_packages
else
    echo "error can't install package $base_packages"
    exit 1;
fi

ci="cargo install"
ca_bin=$HOME/.cargo/bin

if [[ ! -e $ca_bin/cargo-rpm ]]; then
    $ci cargo-rpm
fi

if [[ ! -e $ca_bin/cargo-deb ]]; then
    $ci cargo-deb
fi
