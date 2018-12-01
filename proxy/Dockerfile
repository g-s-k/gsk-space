FROM nginx AS dev
COPY nginx.conf /etc/nginx/nginx.conf

FROM umputun/nginx-le AS prod
ENV TZ America/New_York
ENV LETSENCRYPT true
ENV LE_EMAIL george@georgekaplan.xyz
ENV LE_FQDN g-s-k.space
COPY ./service-*.conf /etc/nginx/conf.d/
