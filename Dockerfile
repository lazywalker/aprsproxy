FROM scratch

LABEL maintainer="Michael BD7MQB <bd7mqb@qq.com>"

WORKDIR /app
COPY target/x86_64-unknown-linux-musl/release/aprsproxy .

USER 1000:1000
EXPOSE 14580

ENTRYPOINT ["./aprsproxy"]
