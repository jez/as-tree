#!/usr/bin/env bash

set -euo pipefail

input_txt="$1"
input_txt_exp="$2"

diff -u "$input_txt_exp" <(src/as_tree "$input_txt")
