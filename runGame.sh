#!/bin/bash

cargo build
./halite -d "30 30" "target/debug/SmartBot" "target/debug/RandomBot"
