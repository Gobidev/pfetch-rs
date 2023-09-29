#!/bin/bash

echo "# List Of All Logos" > ../all_logos.md
echo "Generated using \`./tools/render_all.sh\`" >> ../all_logos.md
while read -r logo; do
    echo "$logo"
    PF_ASCII=$logo "$@" > ./tmp || exit 1
    mkdir -p ../assets/logos
    typst compile -f svg renderer.typ ../assets/logos/"$logo".svg || exit 1
    echo "## $logo" >> ../all_logos.md
    echo "<img src=\"./assets/logos/$logo.svg\" width=\"230\">" >> ../all_logos.md
    echo "" >> ../all_logos.md
done < ./all_logos.txt
rm tmp
