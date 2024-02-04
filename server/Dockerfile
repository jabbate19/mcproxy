FROM rust:latest

WORKDIR /app

RUN apt-get update && apt-get -y install libssl-dev pkg-config libglib2.0-dev

RUN cargo init .

COPY Cargo.* ./

RUN cargo build --release

RUN rm src/*.rs

COPY src/ src/

RUN cargo build --release

CMD ["./target/release/mcproxy-server"]