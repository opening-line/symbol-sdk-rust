#!/bin/bash

set -ex

function generate_code() {
	# $1 blockchain

	local git_root
	git_root="$(git rev-parse --show-toplevel)"

	PYTHONPATH="${git_root}/catbuffer/parser" python3 -m catparser \
		--schema "${git_root}/catbuffer/schemas/$1/all_generated.cats"  \
		--include "${git_root}/catbuffer/schemas/$1" \
		--output "${git_root}/sdk/rust/src/$1" \
		--quiet \
		--generator generator.Generator
}

if [[ $# -eq 0 ]]; then
	generate_code "symbol"
fi

cargo fmt
cargo check