#!/usr/bin/env bash
set -x
set -eo pipefail

for FILE in $(find tests/files -name '*.new.pdf'); do
    echo "Renaming file $FILE"
    mv $FILE $(echo $FILE | sed -e 's/.new.pdf/.pdf/')
done
