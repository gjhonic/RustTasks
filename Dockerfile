FROM rust:1.60-slim-bullseye AS chef
WORKDIR /usr/src/rusttasks
RUN cargo install cargo-chef

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /usr/src/rusttasks/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
RUN cargo install --path .

FROM debian:bullseye-slim
COPY --from=builder /usr/local/cargo/bin/rusttasks /usr/local/bin/rusttasks
WORKDIR /usr/local/bin
CMD ["rusttasks"]