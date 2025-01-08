FROM rust:1.83.0-bookworm
RUN apt-get update && apt-get install libpq-dev -y
WORKDIR /app/auther
COPY . .

RUN cargo install --path .
CMD ["/usr/local/cargo/bin/api_starter"]
