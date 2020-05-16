#!/usr/bin/env bash

set -euo pipefail

run_sh="$1"
run_sh_exp="$2"

diff -u "$run_sh_exp" <("$run_sh")
