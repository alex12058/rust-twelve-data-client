#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$ROOT"

# Generate the modified openapi spec
if command -v python &> /dev/null; then
  python generate_openapi.py
elif command -v python3 &> /dev/null; then
  python3 generate_openapi.py
else
  echo "Error: python or python3 not found in PATH"
  exit 1
fi

# Build the rust client library
rm -rf src
rm -rf docs
java -jar "$ROOT/openapi-generator-rust-builders/modules/openapi-generator-cli/target/openapi-generator-cli.jar" generate \
  -i "$ROOT/openapi.json" \
  -g rust \
  -o "$ROOT" \
  --additional-properties=packageName=twelve_data_client,packageVersion=0.1.0,library=reqwest

# Format the rust code
cargo fmt

# Drop/clean generated files we don't keep
git checkout -- Cargo.toml README.md || true
rm -f git_push.sh
