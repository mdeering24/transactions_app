FROM rust:1.62-bullseye

# For rust-analyzer
RUN wget -qO- "https://github.com/rust-analyzer/rust-analyzer/releases/latest/download/rust-analyzer-x86_64-unknown-linux-gnu.gz" | \
    gunzip > /usr/local/bin/rust-analyzer && \
    chmod 500 /usr/local/bin/rust-analyzer
RUN rustup component add clippy
RUN rustup component add rustfmt
