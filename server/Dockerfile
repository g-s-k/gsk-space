FROM scratch

COPY "./target/x86_64-unknown-linux-musl/release/gsk-crud-server" /gsk-crud-server

EXPOSE 8000

ENTRYPOINT ["/gsk-crud-server"]
