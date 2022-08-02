#!/bin/bash

SRC_DIR=$(cd $(dirname $0); pwd)
BUILD_DIR=$SRC_DIR/build
CMAKE_DIR=$SRC_DIR/cpp

if [ ! -d $BUILD_DIR ]; then
    mkdir $BUILD_DIR
    cd $BUILD_DIR
    cmake $CMAKE_DIR -DCMAKE_TOOLCHAIN_FILE="$CMAKE_DIR/wasi-sdk.cmake" -DWASI_SDK_PREFIX=$WASI_SDK -DCMAKE_BUILD_TYPE=MinSizeRel
else
    cd $BUILD_DIR
fi
cmake --build . 
