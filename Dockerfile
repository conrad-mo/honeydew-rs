FROM rust:latest as builder
WORKDIR /honeydew-rs
COPY . .
RUN apt-get update && apt-get install libssl-dev pkg-config -y
RUN cargo build --release
RUN chmod +x /honeydew-rs

FROM debian
RUN apt-get update && apt-get install libssl-dev pkg-config -y
COPY --from=builder /honeydew-rs/target/release/honeydew-rs /honeydew-rs
ENTRYPOINT ["/honeydew-rs"]
EXPOSE 3000