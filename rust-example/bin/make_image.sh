#!/bin/bash

cargo build
./target/debug/raytracing > image.ppm

open image.ppm
