#!/bin/sh

# Make temporary working directory
mkdir _tmp
cd _tmp

# Clone gh-pages branch and sync new files
git clone -b gh-pages git@github.com:ahsparrow/asselect
rsync -r --delete --filter "P .*" --filter "P CNAME" ../dist/ asselect

# Commit and push new files
cd asselect
git commit -a -m "Update"
git push

# Clean up
cd ../..
rm -rf _tmp
