#!/bin/bash

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ASSETS_DIR="$SCRIPT_DIR/../assets"

# --- Prep icons for MacOS bundle (app finder icon)

mkdir -p "$ASSETS_DIR/icon.iconset"

sips -z 16 16     "$ASSETS_DIR/icon_1024x1024.png" --out "$ASSETS_DIR/icon.iconset/icon_16x16.png"
sips -z 32 32     "$ASSETS_DIR/icon_1024x1024.png" --out "$ASSETS_DIR/icon.iconset/icon_16x16@2x.png"
sips -z 128 128   "$ASSETS_DIR/icon_1024x1024.png" --out "$ASSETS_DIR/icon.iconset/icon_128x128.png"
sips -z 256 256   "$ASSETS_DIR/icon_1024x1024.png" --out "$ASSETS_DIR/icon.iconset/icon_128x128@2x.png"
sips -z 256 256   "$ASSETS_DIR/icon_1024x1024.png" --out "$ASSETS_DIR/icon.iconset/icon_256x256.png"
sips -z 512 512   "$ASSETS_DIR/icon_1024x1024.png" --out "$ASSETS_DIR/icon.iconset/icon_256x256@2x.png"
sips -z 512 512   "$ASSETS_DIR/icon_1024x1024.png" --out "$ASSETS_DIR/icon.iconset/icon_512x512.png"
sips -z 1024 1024 "$ASSETS_DIR/icon_1024x1024.png" --out "$ASSETS_DIR/icon.iconset/icon_512x512@2x.png"

iconutil -c icns "$ASSETS_DIR/icon.iconset"

# --- Prep icon for MacOS doc

magick "$ASSETS_DIR/doc_icon_128x128.png" "$ASSETS_DIR/icon_mac.rgba"

# --- Prep icon for Win / Linux taskbar

magick "$ASSETS_DIR/full_icon_128x128.png" "$ASSETS_DIR/icon_win.rgba"


# NOTE: cd into hexalyzer and do the app release:
# cargo bundle --release