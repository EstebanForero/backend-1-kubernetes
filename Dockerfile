FROM rust:latest AS builder

WORKDIR /usr/src/app

COPY . .

ENV SQLX_OFFLINE=true

RUN cargo test --release

RUN cargo build --release

FROM gcr.io/distroless/cc-debian12

COPY --from=builder /usr/src/app/target/release/backend /backend

EXPOSE 8080

CMD ["/backend"]
