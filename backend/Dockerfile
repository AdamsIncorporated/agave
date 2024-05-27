# syntax=docker/dockerfile:1.4
FROM rust:buster AS base

ENV USER=root
ENV ROCKET_ADDRESS=0.0.0.0
ENV ROCKET_ENV=development
ENV APP_ID=${APP_ID}
ENV CLIENT_ID=${CLIENT_ID}
ENV CLIENT_SECRET=${CLIENT_SECRET}
ENV REDIRECT_URL=${REDIRECT_URL}
ENV ADDRESS=${ADDRESS}
ENV RUST_LOG=${RUST_LOG}
ENV PG_DBNAME=${PG_DBNAME}
ENV PG_HOST=${PG_HOST}
ENV PG_USER=${PG_USER}
ENV PG_PASSWORD=${PG_PASSWORD}

WORKDIR /app
RUN cargo init
COPY Cargo.toml /app/Cargo.toml
RUN cargo fetch
COPY . /app

FROM base AS development

EXPOSE 8000

CMD [ "cargo", "run", "--offline" ]

FROM base AS dev-envs

EXPOSE 8000
RUN <<EOF
apt-get update
apt-get install -y --no-install-recommends git
EOF

RUN <<EOF
useradd -s /bin/bash -m vsapp
groupadd docker
usermod -aG docker vsapp
EOF
# install Docker tools (cli, buildx, compose)
COPY --from=gloursdocker/docker / /
CMD [ "cargo", "run", "--offline" ]

FROM base AS builder

RUN cargo build --release --offline

FROM debian:buster-slim

ENV ROCKET_ENV=production


EXPOSE 8000

COPY --from=builder /app/target/release/react-rust-postgres /react-rust-postgres

CMD [ "/react-rust-postgres" ]
