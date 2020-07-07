#!/bin/bash

sudo perf record -F 99 -g ../target/debug/create-label-run
sudo perf script -i perf.data > perf.unfold
sudo ./FlameGraph/stackcollapse-perf.pl perf.unfold > perf.folded
./FlameGraph/flamegraph.pl perf.folded > layer$1.svg
