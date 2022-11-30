FROM alpine:latest
MAINTAINER Fabio Kleis <fabiohkrc@gmail.com>

RUN apk update
COPY ./target/x86_64-unknown-linux-musl/release/rldap .
COPY .env .
EXPOSE 636 389
ENTRYPOINT ["tail", "-f", "/dev/null"]
