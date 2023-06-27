# Rust as the base image
FROM rust:1.70

# 1. Create a new empty shell project
RUN USER=root cargo new --bin permer 
WORKDIR /permer

# 2. Copy our manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# 3. Build only the dependencies to cache them
RUN cargo build --release
RUN rm src/*.rs

# 4. Now that the dependency is built, copy your source code
COPY ./src ./src

# 5. Build for release.
RUN rm ./target/release/deps/permer*
RUN cargo install --path .

CMD ["permer"]

