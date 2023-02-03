FROM mcr.microsoft.com/devcontainers/rust:0-1-bullseye

# Install build tools
RUN DEBIAN_FRONTEND=noninteractive apt-get update \
    # Apt install without frontend
    && apt-get install -y --no-install-recommends \
        build-essential \
        cmake \
        fontconfig \
        libfontconfig1-dev \
        pkg-config

USER vscode

# Install toolchain
RUN rustup target add wasm32-unknown-unknown \
    rustup default nightly
 
# Install trunk and mdbook
RUN cargo install --locked cargo-quickinstall \
    && cargo quickinstall trunk \
    && cargo quickinstall mdbook
