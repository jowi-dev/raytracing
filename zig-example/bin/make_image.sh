#!/bin/bash

zig build
./zig-out/bin/zig-example > image.ppm

open image.ppm
