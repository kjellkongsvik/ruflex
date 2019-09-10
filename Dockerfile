FROM rust:latest

COPY ./Cargo.toml ./Cargo.lock ./
RUN mkdir src/
RUN echo "fn main() { }" > src/ruflex.rs
RUN echo "fn libmain() { }" > src/libruflex.rs
RUN cargo build

RUN rm ./target/debug/deps/ruflex*
RUN rm ./target/debug/deps/libruflex*

COPY src src
RUN cargo test
