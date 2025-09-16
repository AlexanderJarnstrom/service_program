FROM rust:1.67 as builder
WORKDIR /usr/src/machine_api
COPY src/machine_api .
RUN cargo install --path .
RUN ls /usr/local/cargo/bin

FROM ubuntu:latest
WORKDIR /app
COPY --from=builder /usr/local/cargo/bin/machine_api /usr/local/bin/machine_api
RUN chmod +x /usr/local/bin/machine_api
CMD ["customer_api"]
