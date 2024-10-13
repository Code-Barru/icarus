FROM rust:slim-buster AS builder

WORKDIR /app

COPY . .
RUN cargo build --release
RUN cp target/release/server /usr/bin/server

FROM debian:buster-slim AS runtime

COPY --from=builder /usr/bin/server /usr/bin/server

CMD ["server"]