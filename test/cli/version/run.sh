#!/usr/bin/env bash

set -euo pipefail

src/as_tree -v

echo
echo ---------
echo

src/as_tree --version
