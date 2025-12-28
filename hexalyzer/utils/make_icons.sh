#!/bin/bash

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
DOCS_DIR="$SCRIPT_DIR/../docs"

# --- Prep icons for MacOS bundle (app finder icon)

mkdir -p "$DOCS_DIR/icon.iconset"

sips -z 16 16     "$DOCS_DIR/icon_1024x1024.png" --out "$DOCS_DIR/icon.iconset/icon_16x16.png"
sips -z 32 32     "$DOCS_DIR/icon_1024x1024.png" --out "$DOCS_DIR/icon.iconset/icon_16x16@2x.png"
sips -z 128 128   "$DOCS_DIR/icon_1024x1024.png" --out "$DOCS_DIR/icon.iconset/icon_128x128.png"
sips -z 256 256   "$DOCS_DIR/icon_1024x1024.png" --out "$DOCS_DIR/icon.iconset/icon_128x128@2x.png"
sips -z 256 256   "$DOCS_DIR/icon_1024x1024.png" --out "$DOCS_DIR/icon.iconset/icon_256x256.png"
sips -z 512 512   "$DOCS_DIR/icon_1024x1024.png" --out "$DOCS_DIR/icon.iconset/icon_256x256@2x.png"
sips -z 512 512   "$DOCS_DIR/icon_1024x1024.png" --out "$DOCS_DIR/icon.iconset/icon_512x512.png"
sips -z 1024 1024 "$DOCS_DIR/icon_1024x1024.png" --out "$DOCS_DIR/icon.iconset/icon_512x512@2x.png"

iconutil -c icns "$DOCS_DIR/icon.iconset"

# --- Prep icon for MacOS doc

magick "$DOCS_DIR/doc_icon_128x128.png" "$DOCS_DIR/icon.rgba"


# NOTE: cd into hexalyzer and do the app release:
# cargo bundle --release