#!/bin/bash

cd `dirname $0`/repo/rust/wafwaf
cargo run --example json --release
