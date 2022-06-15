FROM mcr.microsoft.com/vscode/devcontainers/universal:1-linux

USER root

ENV DENO_INSTALL=/deno \
    DENO_DIR=/deno/.cache/deno \
    CARGO_BACKTRACE

RUN mkdir -p /deno \
    && curl -fsSL https://deno.land/x/install/install.sh | sh \
    && ln -s /deno/bin/deno /usr/bin/deno \
    && chown -R codespace /deno /usr/local

USER codespace

RUN    rustup toolchain add stable beta nightly \
    && rustup component add --toolchain stable  rustfmt clippy         \
    && rustup component add --toolchain beta    rustfmt clippy         \
    && rustup component add --toolchain nightly rustfmt clippy         \
    && rustup target    add wasm32-wasi wasm32-unknown-unknown

RUN cargo install cargo-expand cargo-udeps cargo-watch git-uncommit https ripgrep rust-exe save tokio-console
