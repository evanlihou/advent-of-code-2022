#!/bin/zsh
set -e

# Usage: ./initDay.sh <number> <name>
#        ./initDay.sh 01 calorie counting
#            --> Creates a cargo project in the 01-calorie-counting folder and puts boilerplate in main.rs

DAY=$1;
# Pop the first argument and then take the rest, replacing spaces with hyphen
shift;
NAME=$(echo $@ | tr ' ' '-');

if ! [[ $DAY =~ '[0-9]+' ]]; then
    echo "Error: First argument should be a number" >&2; exit 1;
fi
if [ -z $NAME ]; then
    echo "Error: Name not set" >&2; exit 1;
fi

# Make a directory relative to the current script's path
mkdir "${0:a:h}/$DAY-$NAME";
cd "${0:a:h}/$DAY-$NAME";

cargo init --vcs none --name "aoc-$DAY";
cp ../00-setting-up/src/basic_layout.rs src/main.rs;