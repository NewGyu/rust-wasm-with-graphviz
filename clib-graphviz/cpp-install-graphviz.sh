#!/bin/bash
BUILD_DIR=$(dirname $0)/src-graphviz/build

if [ ! -d $BUILD_DIR ]; then
    mkdir $BUILD_DIR
fi

cd $BUILD_DIR
cmake .. -Dwith_expat=OFF
cmake --build . #  -- -j (See https://gitlab.com/graphviz/graphviz/-/issues/2098)
cd $(dirname $0)