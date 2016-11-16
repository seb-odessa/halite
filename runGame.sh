#!/bin/bash

cargo build
~/bin/halite -d "30 30" "target/debug/MyBot" "../MyBot/target/debug/MyBot"
