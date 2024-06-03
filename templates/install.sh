#!/usr/bin/bash

# This script install all the templates
# Note: it is assumed `makeit` is already installed
#       it installs templates to default templates folder

dst="$HOME/.config/makeit/templates"
mkdir -p "$dst"

dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" &> /dev/null & pwd)"

for template in "$dir"/*/; do
    if [ -d "$template" ]; then
        echo "Copying $template...";
        cp -r "$template" "$dst";
    fi
done

echo "All templates have been copied to $dst";
