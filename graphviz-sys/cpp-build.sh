#!/bin/bash

SRC_DIR=$(cd $(dirname $0); pwd)
BUILD_DIR=$SRC_DIR/build
CMAKE_DIR=$SRC_DIR/cpp
EMSDK_DIR=$EMSDK

source $EMSDK_DIR/emsdk_env.sh
if [ ! -d $BUILD_DIR ]; then
    mkdir $BUILD_DIR
    cd $BUILD_DIR
    cmake $CMAKE_DIR -DCMAKE_TOOLCHAIN_FILE="$EMSDK_DIR/upstream/emscripten/cmake/Modules/Platform/Emscripten.cmake" -DCMAKE_BUILD_TYPE=MinSizeRel
else
    cd $BUILD_DIR
fi
cmake --build . 
