FROM rust:1.77-alpine3.19 AS development

RUN apk add --no-cache musl-dev openssl-dev pkgconfig curl

RUN adduser --disabled-password --shell /bin/bash appuser

WORKDIR /opt/encurtador-de-url/app

ENV PKG_CONFIG_PATH=/usr/local/lib/pkgconfig

ARG SQLX_OFFLINE=true

ENV SQLX_OFFLINE=$SQLX_OFFLINE

RUN cargo install cargo-watch

COPY --chown=appuser:appuser . .

RUN cargo build --release

RUN chown appuser:appuser target/debug/encurtador-de-url

USER appuser

CMD ["cargo", "watch -w src -x run"]

FROM rust:alpine3.16 AS release

WORKDIR /opt/encurtador-de-url/app

COPY --from=development /opt/encurtador-de-url/app/target/release/encurtador-de-url /usr/bin/

USER appuser

ENTRYPOINT ["encurtador-de-url"]
