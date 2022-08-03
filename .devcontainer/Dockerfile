# [Choice] Debian OS version (use bullseye on local arm64/Apple Silicon): buster, bullseye
ARG VARIANT="bullseye"
FROM mcr.microsoft.com/vscode/devcontainers/rust:1-${VARIANT}

# [Optional] Uncomment this section to install additional packages.
RUN apt update && export DEBIAN_FRONTEND=noninteractive \
    && apt -y install --no-install-recommends llvm-dev libclang-dev cmake bison flex
USER vscode
WORKDIR /home/vscode
COPY .bash_aliases .bash_aliases
RUN cargo install cargo-wasi && rustup target install wasm32-wasi wasm32-unknown-unknown
ENV WASI_SDK_VER=16
ENV WASI_SDK_VER_FULL=${WASI_SDK_VER}.0
RUN curl -LsS https://github.com/WebAssembly/wasi-sdk/releases/download/wasi-sdk-${WASI_SDK_VER}/wasi-sdk-${WASI_SDK_VER_FULL}-linux.tar.gz -o /tmp/wasi-sdk.tar.gz && \
    tar xvf /tmp/wasi-sdk.tar.gz -C /home/vscode/ && \
    rm -f /tmp/wasi-sdk.tar.gz
ENV WASI_SDK=/home/vscode/wasi-sdk-$WASI_SDK_VER_FULL
ENV PATH=$WASI_SDK/bin:$PATH
RUN curl https://wasmtime.dev/install.sh -sSf | bash