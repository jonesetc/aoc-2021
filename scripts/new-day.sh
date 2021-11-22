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

if [[ -e "$base_path/../packages/day$1" ]]; then
    terminate "Package for day '$1' already exists"
fi

(
    cd "$base_path/../packages"
    npx wrangler generate "day$1"
    cd "day$1"
    echo "$AOC_SESSION" | npx wrangler secret put AOC_SESSION
)
