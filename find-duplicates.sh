#!/bin/bash

possible="adjusted-possible-2025-02-10.txt"
allowed="new-allowed.txt"

for word in $(bat "${possible}")
do
    if [[ "$(rg "${word}" "${allowed}")" != "" ]]
    then
        echo "found duplicate: ${word}"
        sd "${word}\n" "" "${allowed}"
    fi
done
