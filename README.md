# rldap
using ldap bindings for rust

# build statically with musl
cargo build --target x86_64-unknown-linux-musl --release

# run release
cargo run --target x86_64-unknown-linux-musl --release

# build run and remove container
docker build . -t rldap:latest
docker container run -it --name rldap-test rldap:latest
docker container rm rldap-test
