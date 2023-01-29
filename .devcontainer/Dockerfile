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
 
# Install trunk and mdbook
RUN cargo install --locked cargo-quickinstall \
    && cargo quickinstall trunk \
    && cargo quickinstall mdbook

# This doesn't work for now
# # Copy the project in and cargo build
# COPY . /workspace

# RUN chown -R vscode:vscode /workspace

# USER vscode

# RUN cd /workspace \
#     && cargo build
