FROM rust:1.85

WORKDIR /app

COPY Cargo.toml ./

RUN mkdir src
RUN echo "fn main() {println!(\"fake\")}" > src/main.rs
RUN cargo build --release
RUN rm -rf src/

COPY Cargo.toml .
COPY src/ src/

RUN cargo build --bin proxy --release
RUN cp ./target/release/proxy /bin/proxy

CMD ["/bin/proxy"]
