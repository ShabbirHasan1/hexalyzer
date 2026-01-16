#!/bin/bash
# This script generates a 10MB random binary file and converts it to hex

BIN_FILENAME="random_data_1MB.bin"
HEX_FILENAME="random_data_1MB.hex"
SIZE="1"

echo "Generating $SIZE MB random file: $BIN_FILENAME..."
dd if=/dev/urandom of="build/$BIN_FILENAME" bs=1m count=$SIZE
echo "Done! File saved to: $(pwd)/build/$BIN_FILENAME"

objcopy -I binary -O ihex "build/$BIN_FILENAME" "build/$HEX_FILENAME"