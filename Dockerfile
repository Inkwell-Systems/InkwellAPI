FROM rust:1.67.0

WORKDIR /app

#ENV CARGO_HOME=/workdir/.cargo               

COPY ./Cargo.toml ./Cargo.lock ./                       

COPY ./src ./src
COPY ./migrations ./migrations

COPY ./configuration ./configuration
COPY ./sqlx-data.json ./sqlx-data.json

ENV DATABASE_URL "postgres://postgres:password@127.0.0.1:5432/inkwell"
ENV APP_ENV production

ENV SQLX_OFFLINE true

RUN cargo build --release

CMD ["./target/release/inkwell-api"]