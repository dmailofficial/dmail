FROM --platform=linux/amd64 ubuntu:20.04
# Install a basic environment needed for our build tools
ARG DEBIAN_FRONTEND=noninteractive
RUN \
    apt -yq update && \
    apt -yqq install --no-install-recommends curl ca-certificates \
        build-essential pkg-config libssl-dev llvm-dev liblmdb-dev clang cmake git

# Replace your Rust version here
ARG rust_version=1.61.0
ENV RUSTUP_HOME=/opt/rustup \
    CARGO_HOME=/opt/cargo \
    PATH=/opt/cargo/bin:$PATH \
    CARGO_GIT_NET_FETCH_WITH_CLI=true
RUN curl --fail https://sh.rustup.rs/ -sSf \
        | sh -s -- -y --default-toolchain ${rust_version}-x86_64-unknown-linux-gnu --no-modify-path && \
    rustup default ${rust_version}-x86_64-unknown-linux-gnu && \
    rustup target add wasm32-unknown-unknown
RUN cargo install ic-cdk-optimizer

# Install dfx; the version is picked up from the DFX_VERSION environment variable
# Replace your dfx version here
ENV DFX_VERSION=0.9.3
RUN sh -ci "$(curl -fsSL https://sdk.dfinity.org/install.sh)"
RUN dfx cache install

WORKDIR /canister
