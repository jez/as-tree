#!/usr/bin/env bash

set -euo pipefail

script="$1"
expect="$2"

diff -u "$expect" <("$script")
