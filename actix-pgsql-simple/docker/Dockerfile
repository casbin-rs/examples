FROM rust:1

RUN cargo install diesel_cli --no-default-features --features postgres

RUN cargo install cargo-watch

WORKDIR /opt/app

VOLUME ["/usr/local/cargo"]

#ENV RUST_BACKTRACE=1
CMD ["sh", "-c", "diesel database setup --database-url postgres://postgres:postgres@casbin-actix-pgsql-db:5432/casbin && cargo watch -w src -w Cargo.toml -w .env -d 2 -x run"]
