#!/bin/bash

export RUST_LOG="debug"

glslc shaders/shader.vert -o shaders/vert.spv
glslc shaders/shader.frag -o shaders/frag.spv

cargo run
