FROM rust:1.77-alpine3.19 AS development

WORKDIR /opt/encurtador-de-url/app

ENV PKG_CONFIG_PATH=/usr/local/lib/pkgconfig

ENV SQLX_OFFLINE=true

RUN apk add --no-cache musl-dev openssl-dev pkgconfig

RUN cargo install cargo-watch

COPY . .

RUN cargo build --release

CMD ["cargo", "watch -w src -x run"]

FROM rust:alpine3.16 AS release

WORKDIR /opt/encurtador-de-url/app

COPY --from=development /opt/encurtador-de-url/app/target/release/encurtador-de-url /usr/bin/

ENTRYPOINT ["encurtador-de-url"]
