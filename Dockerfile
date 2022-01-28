FROM frolvlad/alpine-rust as builder

RUN apk add openssl-dev
RUN apk add --no-cache ca-certificates

WORKDIR /app
COPY . .

RUN cargo build --release

FROM scratch

COPY --from=builder /app/target/release/tft-pricing-oracle /bin/
COPY --from=builder /etc/ssl /etc/ssl

EXPOSE 8000
CMD ["/bin/tft-pricing-oracle"]