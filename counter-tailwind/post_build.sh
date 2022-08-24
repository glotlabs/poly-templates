#!/bin/bash
set -e

# Generate html
cargo run -p myapp_cli -- home_page > dist/index.html
