FROM rust:latest as builder

WORKDIR /wink

COPY ./src ./src
copy ./static ./static
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

RUN cargo build --release

FROM rust:1.49-slim-buster as runtime

COPY --from=builder /wink/target/release/wink .
COPY --from=builder /wink/static ./static

EXPOSE 8000
CMD ["./wink"]
