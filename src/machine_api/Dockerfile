FROM ubuntu:latest
WORKDIR /app
EXPOSE 8000
COPY target/debug/machine_api .
RUN apt update
RUN apt install -y libpq5
CMD ["./machine_api"]
