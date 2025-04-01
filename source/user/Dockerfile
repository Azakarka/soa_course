FROM rust:1.85

WORKDIR /app

COPY Cargo.toml ./

RUN mkdir src
RUN echo "fn main() {println!(\"fake\")}" > src/main.rs
RUN cargo build --release
RUN rm -rf src/


COPY src/ src/
RUN touch src/main.rs
COPY cert/ cert/

RUN cargo build --bin user-service --release
RUN cp ./target/release/user-service /bin/user-service

CMD ["/bin/user-service"]
