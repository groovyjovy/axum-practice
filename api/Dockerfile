FROM rust:1.81
RUN apt-get upgrade && apt-get update && apt-get install libpq-dev
RUN cargo install diesel_cli --no-default-features --features postgres && cargo install sqlx-cli --no-default-features --features postgres
WORKDIR /api
COPY . .
RUN cargo build
