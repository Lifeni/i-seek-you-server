FROM rust:latest AS builder

RUN rustup target add x86_64-unknown-linux-musl
RUN apt update && apt install -y musl-tools musl-dev
RUN update-ca-certificates

RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "10001" \
    "server"

WORKDIR /i-seek-you

COPY ./ .

RUN cargo build --target x86_64-unknown-linux-musl --release


FROM scratch

COPY --from=builder /etc/passwd /etc/passwd
COPY --from=builder /etc/group /etc/group

WORKDIR /i-seek-you

COPY --from=builder /i-seek-you/target/x86_64-unknown-linux-musl/release/signaling ./
COPY --from=builder /i-seek-you/target/x86_64-unknown-linux-musl/release/webrtc ./
COPY --from=builder /i-seek-you/start.sh ./

USER server:server

EXPOSE 8081  8082
CMD ["./start.sh"]