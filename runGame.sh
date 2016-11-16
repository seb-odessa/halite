#!/bin/bash

cargo build
~/bin/halite -d "25 25" "target/debug/MyBot" "target/debug/MyBot"
