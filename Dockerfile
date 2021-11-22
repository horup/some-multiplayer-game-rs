FROM rust as planner
WORKDIR /app
RUN cargo install cargo-chef
COPY . .
RUN cargo chef prepare  --recipe-path recipe.json

FROM rust as cacher
WORKDIR /app
RUN cargo install cargo-chef
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

FROM rust as builder
WORKDIR /app
RUN cargo install wasm-pack --version 0.9.1
COPY . .
COPY --from=cacher /app/target target
COPY --from=cacher /usr/local/cargo /usr/local/cargo
RUN cargo build --release
RUN wasm-pack build --target web --out-dir public/pkg

#run it
FROM rust as runtime
COPY --from=builder /app/target/release/some-multiplayer-game /usr/local/bin/some-multiplayer-game
COPY --from=builder /app/public /public
ENV ROCKET_ADDRESS=0.0.0.0
EXPOSE 8080
ENTRYPOINT ["some-multiplayer-game"]