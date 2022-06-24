# [Choice] Debian OS version (use bullseye on local arm64/Apple Silicon): buster, bullseye
ARG VARIANT="bullseye"
FROM mcr.microsoft.com/vscode/devcontainers/rust:1-${VARIANT}

# [Optional] Uncomment this section to install additional packages.
RUN apt update && export DEBIAN_FRONTEND=noninteractive \
    && apt -y install --no-install-recommends llvm-dev libclang-dev cmake
USER vscode
WORKDIR /home/vscode
RUN git clone https://github.com/emscripten-core/emsdk.git \
    && cd emsdk \
    && git checkout 3.1.13 \
    && ./emsdk install 3.1.13 \
    && ./emsdk activate 3.1.13
COPY .bash_aliases .bash_aliases
ENV EMSDK /home/vscode/emsdk
ENV PATH $EMSDK/upstream/bin:$PATH
RUN cargo install wasm-pack && rustup target install wasm32-wasi
RUN curl https://wasmtime.dev/install.sh -sSf | bash