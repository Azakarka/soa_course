FROM rust:1.85

WORKDIR /app

RUN apt-get update && apt-get install -y protobuf-compiler

COPY Cargo.toml ./

COPY proto/ proto/

RUN mkdir src
RUN echo "fn main() {println!(\"fake\")}" > src/main.rs
RUN cargo build --release
RUN rm -rf src/

COPY src/ src/
COPY build.rs build.rs
RUN touch src/main.rs

RUN cargo build --bin proxy --release
RUN cp ./target/release/proxy /bin/proxy

CMD ["/bin/proxy"]
