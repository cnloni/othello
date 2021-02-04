#! /bin/bash
cargo rustc --release --bin board -- -C target-cpu=native
