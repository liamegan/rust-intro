#!/bin/bash

if [ -z "$1" ]; then
  echo "Usage: $0 <file_path>"
  exit 1
fi

file_path=$1

if [ ! -f "$file_path" ]; then
  echo "Error: File '$file_path' not found!"
  exit 1
fi

rustc "$file_path" -o test.rst

if [ $? -eq 0 ]; then
  echo "Compilation successful. Output file: test.rst"
else
  echo "Compilation failed."
fi