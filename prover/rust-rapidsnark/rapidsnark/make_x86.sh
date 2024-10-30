#!/bin/bash

LD_LIBRARY_PATH=./package/lib g++ -I./package/include -std=c++17 src/main.cpp ./package/lib/librapidsnark-fr-fq.a -lgmp -L./package/lib -ltbb -pthread -g
