FROM rust:latest

RUN apt-get update -y && apt-get install -y cmake g++ libpq-dev
RUN cargo install diesel_cli --no-default-features --features postgres 
COPY ./css ./css 
COPY ./javascript ./javascript
COPY ./templates ./templates
COPY ./migrations ./migrations
COPY ./src ./src
COPY ./Cargo.toml ./Cargo.toml
COPY ./diesel.toml ./diesel.toml
COPY ./.env ./.env

WORKDIR .

RUN cargo build --release 
EXPOSE 8000

CMD ["cargo", "run","--release"]