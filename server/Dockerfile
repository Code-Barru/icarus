FROM rust:slim-buster AS builder

WORKDIR /app
COPY . .
RUN cargo build --release
RUN cp target/release/icarus-server /usr/bin/icarus-server

FROM debian:buster-slim AS runtime

COPY --from=builder /usr/bin/icarus-server /usr/bin/icarus-server

CMD ["icarus-server"]