#!/bin/bash
cargo build --release 2>&1 | tee /tmp/build_output.txt
grep "error:" /tmp/build_output.txt | head -20