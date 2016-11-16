#!/bin/bash

cargo build
./halite -d "30 30" "target/debug/smart_bot" "target/debug/RandomBot"
