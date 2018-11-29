FROM rust AS base
WORKDIR /usr/src/app

FROM base AS dev
RUN cargo install cargo-watch
CMD cargo watch -x run

FROM base AS build
ENV PROD_ARCH x86_64-unknown-linux-musl
COPY . /usr/src/app
RUN rustup target add ${PROD_ARCH}
RUN apt-get update && apt-get install -y musl-tools musl-dev
RUN cargo build --release --target=${PROD_ARCH}

FROM scratch AS prod
COPY --from=build /usr/src/app/target/x86_64-unknown-linux-musl/release/gsk-crud-server /gsk-crud-server
EXPOSE 8000
ENTRYPOINT ["/gsk-crud-server"]
