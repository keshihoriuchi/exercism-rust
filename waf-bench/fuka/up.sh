#!/bin/bash

cd `dirname $0`/repo/waf
cargo run --example json --release
