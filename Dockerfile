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
COPY . .
COPY --from=cacher /app/target target
COPY --from=cacher /usr/local/cargo /usr/local/cargo
RUN cargo build --release

#build the actual app
ADD . ./
RUN cargo build --release

#run it
FROM rust as runtime
COPY --from=builder /app/target/release/sample /usr/local/bin/sample
ENV ROCKET_ADDRESS=0.0.0.0
EXPOSE 8000
ENTRYPOINT ["sample"]