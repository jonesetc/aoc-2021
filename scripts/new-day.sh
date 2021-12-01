#!/usr/bin/env bash

base_path=$( cd "$(dirname "${BASH_SOURCE[0]}")" ; pwd -P )

function terminate {
    for line in "$@"; do
        echo "$line" >&2
    done
    exit 1
}

if [[ -z "$1" ]]; then
    terminate "Missing day to generate" \
        "usage: $0 <day>" \
        "example: $0 1"
fi

if [[ -e "$base_path/../aoc2021-day$1" ]]; then
    terminate "Package for day '$1' already exists"
fi

(
    cd "$base_path/.."
    cp -R "aoc2021-day0" "aoc2021-day$1"
    sed -i '' "s/\"utils\"/\"aoc2021-day$1\",\n    \"utils\"/g" Cargo.toml
    cd "aoc2021-day$1"
    sed -i '' "s/day0/day$1/g" Cargo.toml
    sed -i '' "s/day 0/day $1/g" Cargo.toml
    sed -i '' "s/day0/day$1/g" wrangler.toml
    sed -i '' "s/\"0\"/\"$1\"/g" src/lib.rs
    wrangler build
    echo "$AOC_SESSION" | wrangler secret put AOC_SESSION
    wrangler publish
)
