#!/bin/zsh
cd ./index/index-compile/
cargo b --release
cp ./target/release/personal-web ../
cd ./..
./personal-web > ../../index.html
