FROM node AS base
WORKDIR /usr/src/app

FROM base AS dev
CMD yarn start

FROM base AS build
COPY . /usr/src/app
RUN yarn build

FROM nginx AS prod
COPY --from=build /usr/src/app/build /usr/share/nginx/html
