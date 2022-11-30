FROM rust:alpine3.16
RUN apk update
LABEL maintainer='fabiohkrc@gmail.com'
COPY ./target/x86_64-unknown-linux-musl/release/rldap .
COPY .env .
EXPOSE 636 389
ENTRYPOINT ["tail", "-f", "/dev/null"]
