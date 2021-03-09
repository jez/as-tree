#!/usr/bin/env bash

set -euo pipefail

echo ----- -f -----
find test/cli/full_path | src/as_tree -f
