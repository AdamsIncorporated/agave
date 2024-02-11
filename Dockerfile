# Build Stage (Frontend)
FROM node:14 AS builder_frontend
WORKDIR /react-app
COPY react-app/package.json react-app/package-lock.json ./
RUN npm install
COPY react-app/ .
RUN npm run build

# Build Stage (Backend)
FROM rust:latest AS builder_backend
WORKDIR /agave
COPY Cargo.toml ./
COPY src ./src/
RUN cargo build --release

# Final Stage
FROM debian:buster-slim
WORKDIR /agave
COPY --from=builder_frontend /react-app/ ./react-app
COPY --from=builder_backend /agave/target/ ./target/
COPY --from=builder_backend /agave/src/ ./src/

EXPOSE 8080

CMD ["target/release/agave.exe"]
