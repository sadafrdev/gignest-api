FROM rust:1.88

WORKDIR /app

COPY . .

ENV SQLX_OFFLINE=true

RUN cargo build --release --package api

CMD ["./target/release/api"]