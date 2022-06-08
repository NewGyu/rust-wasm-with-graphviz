# What's this?

This is an example to try to build rust code which depends on Graphviz written in C/C++ and CMake.

I believe that [CMake crate](https://crates.io/crates/cmake) should help linking Rust code and C code.

# How to build

It should be built by only executing `cargo build` command, but I'm not sure about Cargo building features. So for now, there are sevral steps that are not good enough.

It should be organized in the near future.

## 1. init submodules.

The `clib-grapviz` is a sub project to build Graphviz with CMake crate. It has two submodules, so they have to be initialized with `git submodule init && git submodule update`.

- emsdk
- src-graphviz

## 2. Setup emsdk

```shell
$ cd emsdk
$ ./emsdk install latest
$ ./emsdk activate latest
```

Note: For now, `latest` means 3.1.13.

## 3. Install packages.

These steps should be dockalized.
```shell
$ sudo apt update
$ sudo apt install -y cmake bison flex
```

## 4. Build Graphviz

```shell
# Expect to generate some *.h and more.
# To re-try, remove `src-graphviz/build` dir once.
$ ./cpp-install-graphviz.sh
# Expect to build source codes to wasm with emscripten
$ ./cpp-build.sh
```