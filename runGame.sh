#!/bin/bash

TARGET=release

cargo build --release
~/bin/halite -d "40 40" "target/release/MyBot" "target/release/MyBotV6" "target/release/MyBotV5"
