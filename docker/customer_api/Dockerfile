FROM alpine:3.14
WORKDIR /app
RUN apk add make build-base
COPY src/customer_api .
RUN make
RUN chmod +x customer
ENTRYPOINT ["sh", "-l", "-c", "./app/customer"]
