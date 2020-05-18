#!/usr/bin/env bash

set -euo pipefail

version="${1:-}"

if [ "$version" = "" ]; then
  echo "usage: $0 <new-version>"
  exit 1
fi

sed -i.bak -e "s/version = \"[^\"]*\"/version = \"$version\"/" Cargo.toml src/BUILD
rm Cargo.toml.bak src/BUILD.bak
cargo generate-lockfile --offline

bazel test --test_output=errors //test:update_test/cli/version/run.sh

git add Cargo.toml Cargo.lock src/BUILD test/cli/version/run.sh.exp

git commit -m "Release version $version"
git tag "$version"
git push origin master --tags
