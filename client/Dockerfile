FROM rust AS base
WORKDIR /usr/src/app
RUN rustup install nightly && rustup default nightly
RUN rustup target add wasm32-unknown-unknown
RUN cargo install cargo-web

FROM base AS dev
CMD cargo web start --auto-reload --host=0.0.0.0

FROM base AS build
COPY . /usr/src/app
RUN cargo web deploy --release

FROM nginx AS prod
RUN sed -i 's+}+application/wasm        wasm;\n}+' /etc/nginx/mime.types
COPY --from=build /usr/src/app/target/deploy /usr/share/nginx/html
