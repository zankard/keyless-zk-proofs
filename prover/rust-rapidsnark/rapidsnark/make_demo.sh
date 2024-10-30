#!/bin/bash

g++ -O3 -DUSE_LOGGER -D_LONG_LONG_LIMB src/*.cpp  build/fr.cpp build/fr_raw_arm64.s build/fr_raw_generic.cpp build/fr_generic.cpp build/fq.cpp build/fq_raw_arm64.s build/fq_raw_generic.cpp build/fq_generic.cpp -std=c++17 -I./depends/json/single_include -I./build/ -I./src -I./package/include -I./depends/gmp/package_macos_arm64/include -L./depends/gmp/package_macos_arm64/lib -I/opt/homebrew/include -L/opt/homebrew/lib -lgmp -stdlib=libc++ -ltbb
