#!/bin/bash

set -ex

function generate_code() {
	# $1 blockchain
	# $2 destination

	local git_root
	git_root="$(git rev-parse --show-toplevel)"

	PYTHONPATH="${git_root}/catbuffer/parser" python3 -m catparser \
		--schema "${git_root}/catbuffer/schemas/$1/all_generated.cats"  \
		--include "${git_root}/catbuffer/schemas/$1" \
		--output "${git_root}/sdk/rust/src/$2" \
		--quiet \
		--generator generator.Generator
}

if [[ $# -eq 0 ]]; then
	echo "updating generated code in git"
	# generate_code "nem" "nem"
	generate_code "symbol" "symbol"
# elif [[ "$1" = "dryrun" ]]; then
# 	echo "running dryrun diff"
# 	generate_code "nem" "nc2"
# 	generate_code "symbol" "sc2"

# 	for name in "nc" "sc";
# 	do
# 		diff "./symbolchain/${name}/__init__.py" "./symbolchain/${name}2/__init__.py"
# 		rm -rf "./symbolchain/${name}2"
# 	done
# else
# 	echo "unknown options"
# 	exit 1
fi
