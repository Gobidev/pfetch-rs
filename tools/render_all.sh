#!/bin/bash
set -eu

if [ $# -eq 0 ]; then
    echo "Usage: $0 <command> [args...]" >&2
    exit 1
fi

cmd=("$@")
if [ -e "${cmd[0]}" ] && [ "${cmd[0]#/}" = "${cmd[0]}" ]; then
    cmd[0]="$PWD/${cmd[0]}"
fi

cd "$(dirname "$0")"
trap 'rm -f tmp' EXIT

echo "# List Of All Logos" > ../all_logos.md
echo "Generated using \`./tools/render_all.sh\`, powered by [typst](https://typst.app)" >> ../all_logos.md
while read -r logo; do
    echo "$logo"
    PF_ASCII=$logo "${cmd[@]}" --color > ./tmp
    mkdir -p ../assets/logos
    typst compile -f svg renderer.typ ../assets/logos/"$logo".svg
    echo "## $logo" >> ../all_logos.md
    echo "<img src=\"./assets/logos/$logo.svg\" width=\"230\">" >> ../all_logos.md
    echo "" >> ../all_logos.md
done < ./all_logos.txt
