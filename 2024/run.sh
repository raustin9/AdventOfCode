#!/bin/bash

if [ -z "$1" ]; then
    echo "Usage: $0 <Day to run>"
    exit 1
fi

DAY_TO_RUN="$1"
SOURCE_NAME="$DAY_TO_RUN.rs"
PROGRAM_NAME="bin/$DAY_TO_RUN"

# if ! "$SOURCE_NAME" &> /dev/null; then
#     echo "Error: cannot find file $SOURCE_NAME"
#     exit 1
# fi

echo "Compiling $SOURCE_NAME..."
rustc "$SOURCE_NAME" -o "$PROGRAM_NAME"
echo "done."
echo "Running..."
$PROGRAM_NAME
