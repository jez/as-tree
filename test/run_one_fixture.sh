#!/usr/bin/env bash

set -euo pipefail

mode="$1"
input_txt="$2"
input_txt_exp="$3"

if [ "$mode" = "test" ]; then
  diff -u "$input_txt_exp" <(src/as_tree "$input_txt")
elif [ "$mode" = "update" ]; then
  src/as_tree "$input_txt" > "$input_txt_exp"
else
  echo "Invalid mode: $mode"
  exit 1
fi
