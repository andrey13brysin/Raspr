FROM rust:latest

WORKDIR /app

COPY . .

RUN cargo install cargo-deb
RUN cargo build
RUN cargo deb
RUN chmod +x target/debian/task_0.1.0-1_amd64.deb
RUN apt-get update && apt-get install -y ./target/debian/task_0.1.0-1_amd64.deb

CMD ["task"]