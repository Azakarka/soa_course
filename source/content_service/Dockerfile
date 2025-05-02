FROM rust:1.85

WORKDIR /app

RUN apt-get update && apt-get install -y protobuf-compiler

COPY proto/ ./proto/

COPY Cargo.toml ./


RUN mkdir src
RUN echo "fn main() {println!(\"fake\")}" > src/main.rs
RUN cargo build --release
RUN rm -rf src/

COPY build.rs .
COPY src/ src/
COPY build.rs build.rs
RUN touch src/main.rs

RUN cargo build --bin content-service --release
RUN cp ./target/release/content-service /bin/content-service

CMD ["/bin/content-service"]
