FROM rust:latest
RUN rustup target add x86_64-unknown-linux-gnu
RUN apt-get update && apt-get install -y build-essential
