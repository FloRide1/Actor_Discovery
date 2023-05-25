FROM rust:latest as builder

RUN USER=root cargo new --bin actor_discovery
WORKDIR /actor_discovery

COPY ./Cargo.toml ./Cargo.toml

RUN cargo build --release
RUN rm src/*.rs

COPY . ./

RUN rm ./target/release/deps/actor_discovery*
RUN cargo build --release

FROM debian:buster-slim

RUN apt update
RUN apt install -y libpq5

COPY --from=builder /actor_discovery/target/release/actor_discovery .

CMD ["./actor_discovery"]
