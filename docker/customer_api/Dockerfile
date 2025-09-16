FROM rust:1.67 as builder
WORKDIR /usr/src/customer_api
COPY src/customer_api .
RUN cargo install --path .
RUN ls /usr/local/cargo/bin

FROM ubuntu:latest
WORKDIR /app
COPY --from=builder /usr/local/cargo/bin/customer_api /usr/local/bin/customer_api
RUN chmod +x /usr/local/bin/customer_api
CMD ["customer_api"]
