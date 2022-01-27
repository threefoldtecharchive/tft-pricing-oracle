FROM frolvlad/alpine-rust
RUN apk add gcc musl-dev python3-dev libffi-dev openssl-dev
RUN apk add --no-cache ca-certificates
WORKDIR /app
COPY . /app/
RUN cargo build --release
EXPOSE 8000
CMD ["/app/target/release/tft-pricing-oracle"]