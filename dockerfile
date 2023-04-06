FROM rust:1.68.2-alpine3.17 as build
WORKDIR /app
RUN USER=root
RUN apk update && apk add pkgconfig openssl-dev libc-dev
COPY Cargo.toml .
COPY Cargo.lock .
RUN mkdir ./src && echo 'fn main() {}' > ./src/main.rs
RUN cargo build --release
RUN rm -rf ./src
COPY . .
RUN touch -a -m ./src/main.rs
ENV RUSTFLAGS="-C target-feature=-crt-static"
RUN cargo build --release

FROM alpine:3.17
WORKDIR /app
RUN apk update && apk add openssl ca-certificates
RUN addgroup -S app && adduser -S -G app app 
USER app
COPY --from=build /app/target/release/krabster .
CMD ["./krabster"]
