#!/usr/bin/env bash

set -euo pipefail

as_tree="$1"

"$as_tree" -h

echo
echo ---------
echo

"$as_tree" --help
