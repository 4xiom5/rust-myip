FROM liuchong/rustup

COPY src /app/src
COPY Cargo.toml /app

WORKDIR /app

RUN rustup default nightly

RUN cargo build --release

CMD ["cargo", "run"]