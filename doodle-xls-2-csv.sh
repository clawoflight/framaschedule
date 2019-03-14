#!/bin/sh
set -e

# check for argument
if [ $# -ne 1 ]; then
    echo "Usage: ${0} INPUT.xls" >&2
    exit 1
fi

readonly file="${1%%.*}"

# convert to csv
localc --convert-to csv "${1}"
mv "${file}.csv"{,.old}

# remove headers
tail -n +4 "${file}.csv.old" > "${file}.csv"
mv "${file}.csv"{,.old}

# remove count
head -n -1 "${file}.csv.old" > "${file}.csv"
mv "${file}.csv"{,.old}

# remove non-ascii chars
cat "${file}.csv.old" | tr -cd '\11\12\15\40-\176' > "${file}.csv"
rm "${file}.csv.old"
