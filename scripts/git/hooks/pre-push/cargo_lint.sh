#!/bin/bash

set -eu
RUSTFLAGS="-D warnings" cargo clippy
