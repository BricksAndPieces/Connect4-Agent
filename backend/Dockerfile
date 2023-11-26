FROM rust:latest

RUN mkdir /app
COPY . /app
WORKDIR /app
RUN cargo build --release

EXPOSE 8081
CMD ["./target/release/connect4", "compiled_db.bin", "--webserver"]
