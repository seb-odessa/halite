#!/bin/bash

TARGET=release
rm *hlt
cargo build --release
~/bin/halite -d "30 30" "target/release/MyBot" "target/release/MyBotV7" "target/release/MyBotV6"
