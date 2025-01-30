#!/bin/sh

# This is verified to work on Linux but works incorrectly on MacOS shell.

COLORS="red green blue"

# the for loop continues until it reads all the values from the COLORS
for COLOR in $COLORS
do
  echo "COLOR: $COLOR"
done
