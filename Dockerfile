FROM rust:latest

WORKDIR /var/app

COPY interpreter/ /var/app/

RUN cargo build --release

CMD ["target/release/interpreter"]