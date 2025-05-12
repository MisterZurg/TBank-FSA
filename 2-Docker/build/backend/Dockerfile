FROM rust:1.80-slim-bookworm as build

# create a new empty shell project
RUN USER=root cargo new --bin holodeck
WORKDIR /holodeck

# copy over your manifests
COPY ../../rust_app/Cargo.lock ./Cargo.lock
COPY ../../rust_app/Cargo.toml ./Cargo.toml

# this build step will cache your dependencies
RUN cargo build --release
RUN rm src/*.rs

# copy your source tree
COPY ../../rust_app/src ./src

# build for release
RUN rm ./target/release/deps/holodeck*
RUN cargo build --release

# our final base
FROM rust:1.80-slim-bookworm

# copy the build artifact from the build stage
COPY --from=build /holodeck/target/release/holodeck .

# set the startup command to run your binary
CMD ["./holodeck"]

