# 1. This tells docker to use the Rust official image
FROM rust as build

# 1. Create a new empty shell project
RUN USER=root cargo new --bin AuctionFlipperNode
WORKDIR /AuctionFlipperNode

# 2. Copy our manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# 3. Build only the dependencies to cache them
RUN cargo build --release
RUN rm src/*.rs

# 4. Now that the dependency is built, copy your source code
COPY ./src ./src


# 5. Build for release.
RUN rm ./target/release/deps/AuctionFlipperNode*
RUN cargo build --release

FROM rust

# copy the build artifact from the build stage
COPY --from=build /AuctionFlipperNode/target/release/AuctionFlipperNode .

# Run the binary
EXPOSE 5000
CMD ["./AuctionFlipperNode"]








