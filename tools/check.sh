#!/bin/bash
#
# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at http://mozilla.org/MPL/2.0/.

set -e

cd $(dirname "$0")
cd "$(git rev-parse --show-toplevel)"

source "tools/utils.sh"

assert_installed "cargo-deadlinks"
assert_installed "cargo-fmt"

cargo build --all-targets
cargo test
cargo bench -- --test
cargo doc
cargo deadlinks
cargo package --allow-dirty
cargo +nightly fmt -- --check

exit 0
