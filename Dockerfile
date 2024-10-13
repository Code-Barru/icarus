FROM rust:slim-buster AS builder

WORKDIR /app

COPY ./server ./server
COPY ./shared ./shared
COPY ./agent ./agent
COPY Cargo.toml .
COPY Cargo.lock .

RUN cargo build --release -p server
RUN cp target/release/server /usr/bin/server

FROM debian:buster-slim AS runtime

COPY --from=builder /usr/bin/server /usr/bin/server

CMD ["server"]