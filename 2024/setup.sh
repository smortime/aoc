#!/bin/bash
read -p "What day are you setting up? " day
full_day="day$day"

if ((day < 1 || day > 25))
then
    echo "$day is invalid please select one between 1 and 25"
    exit 1
fi

if [[ -d $full_day ]]
then
    echo "$full_day already exists please delete and try again if intentional"
    exit 1
fi

# create new rust project for the day
cargo new $full_day
mkdir $full_day/input

# in case below curl fails
touch $full_day/input/input.txt

year=$(basename "$PWD")
# Downloads personal input using env with session cookie
curl -H "Cookie: session=$ADV_SESSION_TOKEN" https://adventofcode.com/$year/day/$day/input > $full_day/input/input.txt

echo "$full_day has been setup"
