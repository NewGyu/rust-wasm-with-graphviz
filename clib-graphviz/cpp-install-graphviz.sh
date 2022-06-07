#!/bin/bash

if [ ! -d "src-graphviz/build" ] 
then
    #  Generate grammar files (and others)  ---
    cd ./src-graphviz
    mkdir ./build
    cd ./build
    cmake ..
    cmake --build . #  -- -j (See https://gitlab.com/graphviz/graphviz/-/issues/2098)
    cd ..

    cd ..
fi
