FROM mcr.microsoft.com/devcontainers/rust:0-1-bullseye

# Install trunk and mdbook
RUN cargo install --locked cargo-quickinstall \
    && cargo quickinstall trunk mdbook

# Copy the project in and cargo build
COPY . /workspace

RUN cargo build

USER vscode