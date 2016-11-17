#!/bin/bash

cargo build
~/bin/halite -d "30 30" "target/debug/MyBot" "target/debug/MyBotV3" "target/debug/MyBotV4"
