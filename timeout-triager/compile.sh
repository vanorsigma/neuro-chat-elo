#!/bin/bash
# Quick commands to compile the program

cmake -DCMAKE_EXPORT_COMPILE_COMMANDS=ON -DCMAKE_BUILD_TYPE=debug
make
