ARG BINARY_NAME_DEFAULT=juniper_kv_todos

FROM clux/muslrust:nightly as builder
RUN groupadd -g 10001 -r dockergrp && useradd -r -g dockergrp -u 10001 dockeruser
ARG BINARY_NAME_DEFAULT
ENV BINARY_NAME=$BINARY_NAME_DEFAULT

COPY Cargo.lock .
COPY Cargo.toml .
RUN mkdir src \
    && echo "fn main() {print!(\"Dummy main\");} // dummy file" > src/main.rs
RUN set -x && cargo build --target x86_64-unknown-linux-musl --release
RUN set -x && rm target/x86_64-unknown-linux-musl/release/deps/$BINARY_NAME*

COPY src ./src
RUN set -x && cargo build --target x86_64-unknown-linux-musl --release
RUN mkdir -p /build-out
RUN set -x && cp target/x86_64-unknown-linux-musl/release/$BINARY_NAME /build-out/

FROM scratch

COPY --from=0 /etc/passwd /etc/passwd
USER dockeruser

ARG BINARY_NAME_DEFAULT
ENV BINARY_NAME=$BINARY_NAME_DEFAULT

ENV RUST_LOG="error,$BINARY_NAME=info"
COPY --from=builder /build-out/$BINARY_NAME /

CMD ["/juniper_kv_todos"]
