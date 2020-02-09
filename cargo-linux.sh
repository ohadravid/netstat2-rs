#!/bin/bash

docker run --rm -v $(pwd):/project -w /project -e CARGO_HOME=/project/cargo_home rustlang/rust:nightly cargo "$@"