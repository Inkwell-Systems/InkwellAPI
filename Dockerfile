FROM rust:1.67.0

WORKDIR /app

#ENV CARGO_HOME=/workdir/.cargo               

COPY ./Cargo.toml ./Cargo.lock ./                       

COPY ./src ./src
COPY ./.env ./.env
COPY ./migrations ./migrations

COPY ./configuration.json ./configuration.json
COPY ./sqlx-data.json ./sqlx-data.json

ENV SQLX_OFFLINE true

RUN cargo build --release

CMD ["./target/release/inkwell-api"]