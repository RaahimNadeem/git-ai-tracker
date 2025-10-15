#!/bin/bash

set -euo pipefail

mkdir -p ./target/gitwrap/bin

echo "Creating symlinks from in gitwrap folder to target/debug"
ln -sf $(pwd)/target/debug/git-ai-tracker $(pwd)/target/gitwrap/bin/git
ln -sf $(pwd)/target/debug/git-ai-tracker $(pwd)/target/gitwrap/bin/git-ai-tracker

echo "In your shell profile,"
echo "1. Remove any artifacts from running 'install.sh'"
echo "2. Remove any aliases to git"
echo "3. Add the following to your shell profile:"
echo "# git-ai-tracker local dev"
echo "export PATH=\"$(pwd)/target/gitwrap/bin:\$PATH\""
