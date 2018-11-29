FROM node AS base
WORKDIR /usr/src/app
RUN curl https://sh.rustup.rs | sh -s -- --default-toolchain nightly -y
ENV PATH=/root/.cargo/bin:$PATH
RUN rustup target add wasm32-unknown-unknown --toolchain nightly

FROM base AS dev
CMD yarn start

FROM base AS build
COPY . /usr/src/app
RUN yarn build

FROM nginx AS prod
COPY --from=build /usr/src/app/build /usr/share/nginx/html
