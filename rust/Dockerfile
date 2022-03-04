# Rust as the base image
FROM rust:slim
RUN rustup override set nightly

# 1. Create a new empty shell project
RUN USER=root cargo new --bin rust
WORKDIR /rust

# 2. Copy our manifests
COPY ./lib ./lib
COPY ./Cargo.toml ./Cargo.toml

# 3. Build only the dependencies to cache them
RUN cargo build --release
RUN rm src/*.rs

# 4. Now that the dependency is built, copy your source code
COPY ./.env ./.env
COPY ./src ./src

# 5. Build for release.
RUN rm ./target/release/deps/rust*
RUN cargo install --path .
RUN export RUST_BACKTRACE=1
CMD [ "rust" ]