#!/usr/bin/env bash

set -euo pipefail

cat > .bazelrc.local <<EOF
common --curses=no
EOF
