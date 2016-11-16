#!/bin/bash

cargo build --release
./halite -d "30 30" "target/release/smart_bot" "target/release/s1bot"
