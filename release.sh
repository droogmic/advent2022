#!/bin/bash

set -e
set -u
set -o pipefail
set -x

BRANCH="$(git rev-parse --abbrev-ref HEAD)"
if [[ "$BRANCH" != "main" ]]; then
  echo 'Aborting script';
  exit 1;
fi
if [ -z "$1" ]
  then
    echo "No argument supplied"
fi
trunk --config Release.toml build
git switch gh-pages
git rm *.html
git rm *.js
git rm *.wasm
git rm *.css
cp dist/* ./
git add *.html
git add *.js
git add *.wasm
git add *.css
git commit -am "$1"
git push origin HEAD
git switch main
