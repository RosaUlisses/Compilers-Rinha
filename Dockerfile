FROM rust:latest

WORKDIR /var/rinha

COPY interpreter/ /var/rinha/

RUN cargo build --release

CMD ["target/release/interpreter"]