#!/usr/bin/env bash

set -euo pipefail

mode="$1"
run_sh="$2"
run_sh_exp="$3"

if [ "$mode" = "test" ]; then
  diff -u "$run_sh_exp" <("$run_sh")
elif [ "$mode" = "update" ]; then
  "$run_sh" > "$run_sh_exp"
else
  echo "Invalid mode: $mode"
  exit 1
fi
