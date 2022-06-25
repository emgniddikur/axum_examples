FROM rust:1.59.0

WORKDIR /todo_v2

COPY . .

RUN cargo build --release

ENTRYPOINT ["/todo_v2/target/release/todo_v2"]
