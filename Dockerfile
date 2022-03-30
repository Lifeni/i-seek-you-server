# https://dev.to/rogertorres/first-steps-with-docker-rust-30oi

FROM rust as build

# create a new empty shell project
RUN USER=root cargo new --bin i-seek-you
WORKDIR /i-seek-you

# copy over your manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# this build step will cache your dependencies
# https://dev.to/tomasfejfar/comment/1kpi7
RUN cargo build --release & rm src/*.rs

# copy your source tree
COPY ./src ./src

# build for release
RUN rm ./target/release/deps/i-seek-you*
RUN cargo build --release

# our final base
FROM debian:buster-slim

# copy the build artifact from the build stage
COPY --from=build /i-seek-you/target/release/i-seek-you .

# set the startup command to run your binary
CMD ["./i-seek-you"]