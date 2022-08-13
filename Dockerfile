FROM alpine:3.16 AS builder
RUN apk update && apk add tzdata
# RUN apk update && apk add ca-certificates && apk add tzdata

FROM scratch
LABEL maintainer="Michael BD7MQB <bd7mqb@qq.com>"

COPY --from=builder /usr/share/zoneinfo /usr/share/zoneinfo
# COPY --from=builder /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/

WORKDIR /app
COPY target/x86_64-unknown-linux-musl/release/aprsproxy .

EXPOSE 14580

ENTRYPOINT ["./aprsproxy"]
