FROM rust:1.70.0 as builder

WORKDIR /todo-api

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
COPY ./src ./src

RUN cargo build --release
RUN rm src/*.rs

# The final base image
FROM debian:buster-slim
COPY --from=builder /todo-api/target/release/todo-api /usr/src/todo-api
CMD ["/usr/src/todo-api"]