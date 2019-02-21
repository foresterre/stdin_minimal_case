#!/bin/bash
cat in2.bmp | cargo run --bin s -- out2.bmp && display out2.bmp
