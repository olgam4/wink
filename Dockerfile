FROM rust:latest as builder

WORKDIR /wink

COPY ./src ./src
COPY ./migrations ./migrations
COPY ./sqlx-data.json ./sqlx-data.json
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

RUN cargo build --release

FROM rust:1.49-slim-buster as runtime

COPY --from=builder /wink/target/release/wink .
COPY --from=builder /wink/src/static ./src/static
COPY --from=builder /wink/migrations ./migrations

EXPOSE 8000
CMD ["./wink"]
