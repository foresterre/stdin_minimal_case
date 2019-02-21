#!/bin/bash
cat in1.jpg | cargo run --bin s -- out1.bmp && display out1.bmp
