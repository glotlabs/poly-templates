#!/bin/bash
set -e

echo "Cleaning up previous builds..."
mkdir -p dist
rm -rf dist/*

mkdir -p myapp_web/wasm
rm -rf myapp_web/wasm/*

echo
echo "Building workspace..."
cargo build

echo
echo "Building wasm..."
(
    cd myapp_wasm
    wasm-pack build --target web --out-name myapp --out-dir ../myapp_web/wasm
)

echo
echo "Generating html"
cargo run -p myapp_cli -- home_page > dist/index.html


echo
echo "Building web assets..."
(
    cd myapp_web
    npm run build-dev
    cp -rf wasm ../dist/
)

