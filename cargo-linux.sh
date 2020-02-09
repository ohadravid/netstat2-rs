#!/bin/bash

docker run --rm -v $(pwd):/project -w /project -e CARGO_HOME=/project/cargo_home -e CARGO_BUILD_TARGET_DIR=/project/target/x86_64-unknown-linux-gnu rustlang/rust:nightly cargo "$@"