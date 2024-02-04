FROM rustlang/rust:nightly as builder

RUN apt-get update
RUN apt-get -y install upx musl-tools
RUN rustup target add x86_64-unknown-linux-musl
WORKDIR /todo-api
COPY . .
RUN cargo build --release --target x86_64-unknown-linux-musl

RUN ls -lah target/x86_64-unknown-linux-musl/release/todo-api
RUN strip target/x86_64-unknown-linux-musl/release/todo-api
RUN upx --ultra-brute target/x86_64-unknown-linux-musl/release/todo-api
RUN ls -lah target/x86_64-unknown-linux-musl/release/todo-api

FROM busybox:musl

COPY --from=builder /todo-api/target/x86_64-unknown-linux-musl/release/todo-api /bin/todo-api  
CMD ["todo-api"]