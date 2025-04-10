#!/bin/bash

# Run Ferris in a loop and logs the stdout and stderr.

LOG_DIR="$HOME/logs"
LOG_FILE="$LOG_DIR/ferris.log"

# Create the log file.
if [ ! -d "$LOG_DIR" ]; then
  mkdir -p "$LOG_DIR"
  touch "$LOG_FILE"
fi

# Check if ferris binary exists and is executable
if ! command -v ferris &> /dev/null; then
  echo "ERROR: Ferris not found." >> "$LOG_FILE"
  exit 1
fi

while true; do
  ferris &> "$LOG_FILE"  # Redirect both stdout and stderr to ferris.log
  if [ $? -gt 0 ]; then
    mv "$LOG_FILE" "$LOG_DIR/old.log"  # Rename ferris.log to old.log on error
  fi
done
