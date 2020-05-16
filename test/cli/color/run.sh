#!/usr/bin/env bash

set -euo pipefail

export LS_COLORS="no=0:rs=0:di=00;34:ln=00;36:ex=00;32"

echo ----- --color always -----
find test/cli/color | src/as_tree --color always

echo ----- --color auto -----
find test/cli/color | src/as_tree --color auto

echo ----- --color never -----
find test/cli/color | src/as_tree --color never
