#!/bin/bash

# A small script to run pfetch (command specified with arguments) with all available logos

while read -r logo; do
    PF_ASCII=$logo "$@"
done < ./all_logos.txt
