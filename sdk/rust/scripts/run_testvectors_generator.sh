#!/bin/bash

set -ex

function generate_model_vectors() {
	# $1 blockchain

	local git_root
	git_root="$(git rev-parse --show-toplevel)"

	PYTHONPATH="${git_root}/catbuffer/parser" python3 -m catparser \
		--schema "${git_root}/catbuffer/schemas/$1/all_generated.cats"  \
		--include "${git_root}/catbuffer/schemas/$1" \
		--output "${git_root}/sdk/rust/tests/${1}_models.rs" \
		--quiet \
		--generator generator.models_test_vector.Generator
}

generate_model_vectors "symbol"
cargo fmt
cargo check