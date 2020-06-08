#!/bin/bash

[[ -d /tmp/input ]] mkdir /tmp/input
[[ -d /tmp/output ]] mkdir /tmp/output

docker run -d \
  --name compressor \
  -e INPUT_DIR=/tmp/input \
  -e OUTPUT_DIR=/tmp/output \
  compressor
