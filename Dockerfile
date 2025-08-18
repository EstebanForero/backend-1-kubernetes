FROM rust:latest-slim

WORKDIR /usr/src/app

COPY . .

RUN cargo test --release

RUN cargo build --release

FROM debian:bullseye-slim
RUN apt-get update && apt-get install -y extra-runtime-dependencies && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/myapp /usr/local/bin/myapp
CMD ["myapp"]
