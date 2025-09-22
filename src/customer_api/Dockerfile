FROM ubuntu:latest
WORKDIR /app
EXPOSE 8000
COPY target/debug/customer_api .
RUN apt update
RUN apt install -y libpq5
ENTRYPOINT ["./customer_api"]
