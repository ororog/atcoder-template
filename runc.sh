#!/bin/sh
IN=src/$1.cpp
OUT=src/$1.out
clang++ --std=c++14 $IN -o $OUT && $OUT