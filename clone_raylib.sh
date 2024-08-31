#!/usr/bin/env sh

set -xe

# This file is used to create the `raylib` directory
# Originally, a submodule was used, but cloning 400M of history on every install is suboptimal,
# and often fails on computer with bad internet speeds.

# This solution is a bit hacky, but works.

# Additionally, the .zip from github isn't used since changes not yet published in a release (future 5.1)
# are required for wayland to at least render (HI-DPI is still a dream).

rm -rf raylib

mkdir raylib
cd raylib

# Instead of git clone, this permits cloning only part of the history,
# up to the commit we're interested in
# It's still big (120M), but it's better.

git init
git remote add origin https://github.com/raysan5/raylib.git

# -- Pinned revision --
git fetch origin --depth=1 39f12859dcdf92822c8fefece1135bf6e76a1573
git reset --hard 39f12859dcdf92822c8fefece1135bf6e76a1573

# Remove history (~350 MB in a full clone)
# Maybe it's possible to remove all history up until the commit hash,
# but I haven't found a nice way that allowed me to do it "automatically" like submodules
rm -rf .git/ 

# Remove unnecessary files (~50 MB)
rm -rf .github/ examples/ projects/ parser/ logo/

# The resulting raylib directory is barely 15 megs
cd ..
